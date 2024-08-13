use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str::Utf8Error;
use std::string::FromUtf8Error;

fn handle_client(mut stream: TcpStream) {
    let mut writer = stream.try_clone().unwrap();
    let mut reader = BufReader::new(stream);

    // Parse headers
    let mut lines = reader.lines();
    let first_line = lines.next().unwrap().unwrap();
    println!("First line: {first_line}");
    let mut headers = Vec::new();
    loop {
        let line = match lines.next().unwrap() {
            Ok(line) => line,
            Err(e) => {
                println!("Error while reading HTTP headers: {e}");
                return;
            }
        };
        if line.is_empty() {
            break;
        }
        headers.push(line);
    }
    println!("Headers: {headers:#?}");
    let content_length = todo!("Parse the headers and find the content length");

    // Done with headers.
    let name = "World".to_owned();
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
                eprintln!("Could not write request: {e}");
                return;
            }
        }
    }
    println!("Request hung up");
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Listening...");

    // accept connections and process them serially
    for stream in listener.incoming() {
        println!("Accepted request");
        // TODO: Ignore errors listening
        let mut stream = stream?;

        std::thread::spawn(|| {
            handle_client(stream);
        });
    }
    Ok(())
}
