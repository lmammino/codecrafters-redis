use std::{io::Read, io::Write, net::TcpListener, str, thread};

mod resp;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                thread::spawn(move || {
                    println!("accepted new connection");
                    loop {
                        let mut buf: [u8; 1024] = [0; 1024];
                        let read_result = stream.read(&mut buf);
                        match read_result {
                            Ok(num_bytes) => {
                                if num_bytes == 0 {
                                    continue;
                                }
                                let message = str::from_utf8(&buf[..num_bytes]).unwrap();
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
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
