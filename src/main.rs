use std::cmp::min;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};

// On MacOS, the `write` syscall can only handle a certain number
// of bytes at once. Anything larger will return OS Error 22,
// i.e. EINVAL.
// From the `man 2 write`:
//   write() and pwrite() will fail if the parameter nbyte exceeds INT_MAX, and they do not attempt a partial
//   write.
// So, let's limit it to MAX_INT. I imported MAX_INT in C and printed it, and it was
// this.
/// Maximum number of bytes that can be written in a single `write` call.
const MAX_WRITE_LEN: usize = i32::MAX as usize;

fn handle_client(stream: TcpStream) -> Result<(), String> {
    let writer = stream.try_clone().unwrap();
    let reader = BufReader::new(stream);

    respond(writer, reader)
}

fn respond<W: Write, R: BufRead>(mut writer: W, mut reader: R) -> Result<(), String> {
    // Parse headers
    let mut line_buf = String::new();
    reader
        .read_line(&mut line_buf)
        .map_err(|e| format!("Could not read first line of HTTP request: {e}"))?;
    // We'll probably need this in the future to keep the HTTP verb.
    let first_line = line_buf.clone();
    println!("First line: {first_line}");
    let mut headers: HashMap<String, String> = HashMap::new();
    loop {
        line_buf.clear();
        reader
            .read_line(&mut line_buf)
            .map_err(|e| format!("Error reading from HTTP request: {e}"))?;
        if line_buf == "\r\n" {
            break;
        }
        let (k, v) = match line_buf.split_once(": ") {
            Some((k, v)) => (k.to_owned(), v.trim().to_owned()),
            None => return Err(format!("Invalid HTTP header {line_buf} (missing a ': ')")),
        };
        headers.insert(k, v);
    }
    println!("Headers: {headers:#?}");
    let content_length = headers
        .get("Content-Length")
        .ok_or("Missing content-length header".to_owned())?
        .trim()
        .parse()
        .map_err(|e| format!("Content-length header was not a valid number: {e}"))?;
    println!("Content length is {content_length}");

    if let Some(v) = headers.get("Expect") {
        if v == "100-continue" {
            // See https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/100
            // Note that the server must still write a final response status
            // after it reads the HTTP body.
            let first_line_resp = b"HTTP/1.1 100 Continue\r\n\r\n";
            let n = writer.write(first_line_resp).unwrap();
            assert_eq!(n, first_line_resp.len());
        }
    }

    // Read the HTTP body.
    let mut body_buf = Vec::new();
    reader
        .take(content_length)
        .read_to_end(&mut body_buf)
        .map_err(|e| format!("IO error, could not read the HTTP body: {e}"))?;

    // Write the response.
    let name = String::from_utf8_lossy(&body_buf);
    let status = "HTTP/1.1 200 All Good Mate";
    let body = format!("你好 {name}");
    let body_len = body.as_bytes().len();
    let headers = format!("Content-Length: {}\r\n", body_len);
    let msg = format!("{status}\r\n{headers}\r\n{body}");
    let msg_binary = msg.as_bytes();
    let total_bytes = msg_binary.len();
    let mut total_written = 0;
    let mut num_writes = 0;
    while total_written < total_bytes {
        // Don't write more than `limit` bytes at once.
        let write_cap = min(MAX_WRITE_LEN, body_len);
        // Don't write more bytes than remain in the body!
        let write_cap = min(write_cap, total_bytes - total_written);

        let write_fragment = &msg_binary[total_written..total_written + write_cap];
        let res = writer.write(write_fragment);
        match res {
            Ok(n) => {
                println!("Wrote {n} bytes (write #{num_writes})");
                total_written += n;
            }
            Err(e) => {
                return Err(format!("Could not write request: {e:?}"));
            }
        }
        num_writes += 1;
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr)?;
    println!("Listening on {addr}...");

    // accept connections and process them serially
    for stream in listener.incoming() {
        println!("Accepted request");
        // TODO: Ignore errors listening
        let stream = stream?;

        std::thread::spawn(|| {
            if let Err(e) = handle_client(stream) {
                eprintln!("{e}");
            }
        });
    }
    Ok(())
}
