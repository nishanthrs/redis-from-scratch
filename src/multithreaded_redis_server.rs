use std::io::{Read,Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use anyhow::bail;
use log::{info,debug};
use env_logger::{Env};


const CHUNK_SIZE: usize = 1024;
const NULL_BYTE: &str = "\0";

fn handle_connection(stream: &mut TcpStream) -> anyhow::Result<()> {
    let mut read_buffer = [0;CHUNK_SIZE];
    loop {
        let num_bytes_read = stream.read(&mut read_buffer).expect("Reading from stream into buffer failed!");
        debug!("Num bytes read: {}", num_bytes_read);
        if num_bytes_read == 0 {
            break;
        }

        let request = std::str::from_utf8(&read_buffer).expect("Couldn't parse buffer into str.").split(NULL_BYTE).next();
        info!("Stream input: {:?}", request);
        match request {
            Some(request) => {
                for req_part in request.split("\r\n") {
                    match req_part.to_uppercase().as_str() {
                        "PING" => stream.write(b"+PONG\r\n"),
                        "" => {
                            debug!("Reached end of input.");
                            Ok(0)
                        }
                        other_input => {
                            debug!("Input: {} is currently not handled.", other_input);
                            Ok(0)
                        },
                    }.expect("Parsing request failed!");
                }
            },
            None => bail!("No data after split by null byte"),
        }
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let env = Env::default().default_filter_or("debug");
    env_logger::init_from_env(env);

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                info!("accepted new connection");
                /* Creates a new thread for each connection; this is expensive and not feasible in the real world!
                Better to use a thread pool with a fixed number of threads */
                thread::spawn(move || {
                    // Within same connection, accept multiple commands in loop; if # bytes read is 0, exit connection
                    handle_connection(&mut stream).expect("Something went wrong while handling connection.");
                });
            }
            Err(e) => {
                bail!("error: {}", e);
            }
        }
    }

    Ok(())
}
