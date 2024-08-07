use std::io::Write;
use std::net::TcpListener;

fn handle_client<T: Write>(mut stream: T) {
    let status = "HTTP/1.1 200 All Good Mate";
    let body = "你好 world";
    let body_len = body.as_bytes().len();
    let headers = format!("Content-Length: {}\r\n", body_len);
    let msg = format!("{status}\r\n{headers}\r\n{body}");
    let msg_binary = msg.as_bytes();
    let total_bytes = msg_binary.len();
    let mut total_written = 0;
    while total_written < total_bytes {
        let res = stream.write(&msg_binary[total_written..total_bytes]);
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
        let stream = stream?;
        std::thread::spawn(move || {
            handle_client(stream);
        });
    }
    Ok(())
}
