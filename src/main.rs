use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(stream: TcpStream) -> Result<(), String> {
    let mut writer = stream.try_clone().unwrap();
    let mut reader = BufReader::new(stream);

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
            Some((k, v)) => (k.to_owned(), v.to_owned()),
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

    // Read the HTTP body.
    let mut body_buf = Vec::new();
    reader
        .take(content_length)
        .read_to_end(&mut body_buf)
        .map_err(|e| format!("IO error, could not read the HTTP body: {e}"))?;
    let name = String::from_utf8(body_buf)
        .map_err(|e| format!("HTTP body was not a UTF-8 string: {e}"))?;

    // Write the response.
    let status = "HTTP/1.1 200 All Good Mate";
    let body = format!("你好 {name}");
    let body_len = body.as_bytes().len();
    let headers = format!("Content-Length: {}\r\n", body_len);
    let msg = format!("{status}\r\n{headers}\r\n{body}");
    let msg_binary = msg.as_bytes();
    let total_bytes = msg_binary.len();
    let mut total_written = 0;
    while total_written < total_bytes {
        let res = writer.write(&msg_binary[total_written..total_bytes]);
        match res {
            Ok(n) => {
                println!("Wrote {n} bytes");
                total_written += n;
            }
            Err(e) => {
                return Err(format!("Could not write request: {e}"));
            }
        }
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Listening...");

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
