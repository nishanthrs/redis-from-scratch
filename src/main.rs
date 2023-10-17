use std::io::{Read,Write};
use std::net::{TcpListener, TcpStream};
use anyhow::bail;
use log::{info,debug};
use env_logger::{Env};


const CHUNK_SIZE: usize = 1024;
const NULL_BYTE: &str = "\0";

async fn handle_connection(stream: &mut TcpStream) -> anyhow::Result<()> {
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
                // Should return a Redis RESP array: https://redis.io/docs/reference/protocol-spec
                let resp_array = request.split("\r\n").collect::<Vec<&str>>();
                let first_elem = resp_array.get(0).expect("Client request not a RESP array; no \r\n separator found!");
                let num_elems = first_elem[1..].parse::<usize>().expect("First element of client request is not a number!");
                for (i, req_part) in resp_array[..resp_array.len()-3].iter().enumerate() {
                    match req_part.to_uppercase().as_str() {
                        "PING" => stream.write(b"+PONG\r\n"),
                        "ECHO" => {
                            // println!("Rest of RESP array: {:?}, {}", resp_array, i);
                            let echo_output = format!("+{}\r\n", resp_array[i+2]);
                            let echo_output_bytes = echo_output.as_bytes();
                            stream.write(echo_output_bytes)
                        },
                        "" => {
                            debug!("Reached end of input.");
                            Ok(0)
                        },
                        other_input => {
                            debug!("Input: {} is currently not handled.", other_input);
                            Ok(0)
                        }
                    }.expect("Parsing part of request failed!");
                }
            },
            None => bail!("No data after split by null byte"),
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let env = Env::default().default_filter_or("debug");
    env_logger::init_from_env(env);

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                info!("accepted new connection");
                /* tokio::spawn creates an async task that runs the future (I/O function) passed as argument
                Returns a Result<JoinHandle> (i.e. spawned async task) */
                tokio::spawn(async move {
                    // Within same connection, accept multiple commands in loop; if # bytes read is 0, exit connection
                    handle_connection(&mut stream).await.expect("Something went wrong while handling connection.");
                });
            }
            Err(e) => {
                bail!("error: {}", e);
            }
        }
    }

    Ok(())
}
