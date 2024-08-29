# Below C Level HTTP Server

## Setup

1. [Install Rustup](https://rustup.rs/)
2. Install the latest Rust + cargo (Rust's build tool): `rustup install stable`
3. Run it: `cargo run`

If you have [just](https://github.com/casey/just?tab=readme-ov-file#packages) you can run
some common tasks e.g. `just watch` or `just run`.

## HTTP structure

* This is a request/response protocol.
* What is a HTTP request?
  * Request first lines have the its HTTP "verb", request's path, and the HTTP version (1.1)
  * After that, HTTP header

HTTP requests look something like this:

```
GET / HTTP/1.1
Host: google.com
User-Agent: curl/8.6.0
Accept: */*

<body></body>
```

HTTP responses look something like:

```
HTTP/1.1 301 Moved Permanently
Location: http://www.google.com/
Content-Type: text/html; charset=UTF-8

<HTML><HEAD><meta http-equiv="content-type" content="text/html;charset=utf-8">
```

* Accept HTTP requests
* Always respond with a string, "Hello World"
* 
