use std::io::{Read,Write};
use std::net::TcpListener;
use anyhow::bail;
use log::{info,debug};
use env_logger::{Env};


const CHUNK_SIZE: usize = 1024;
const NULL_BYTE: &str = "\0";

fn main() -> anyhow::Result<()> {
    let env = Env::default().default_filter_or("debug");
    env_logger::init_from_env(env);

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                info!("accepted new connection");
                let mut read_buffer = [0;CHUNK_SIZE];
                let num_bytes_read = stream.read(&mut read_buffer).expect("Reading from stream into buffer failed!");
                debug!("Num bytes read: {}", num_bytes_read);
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
                                other_cmd => {
                                    debug!("Input: {} is currently not handled.", other_cmd);
                                    Ok(0)
                                },
                            }.expect("Parsing request failed!");
                        }
                    },
                    None => bail!("No data after split by null byte"),
                }
            }
            Err(e) => {
                bail!("error: {}", e);
            }
        }
    }

    Ok(())
}
