use std::{io::Read, io::Write, net::TcpListener, str};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");
                loop {
                    let mut buf: [u8; 1024] = [0; 1024];
                    let read_result = stream.read(&mut buf);
                    match read_result {
                        Ok(_num_bytes) => {
                            let message = str::from_utf8(&buf[.._num_bytes]).unwrap();
                            print!("received message: {}", message);
                            // TODO: verify that we are really receiving a ping
                            let response = b"+PONG\r\n";
                            let _write_result = stream.write(response);
                        }
                        Err(e) => {
                            println!("error: {}", e);
                            break;
                        }
                    }
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
