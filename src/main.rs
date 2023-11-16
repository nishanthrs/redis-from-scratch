use anyhow::bail;
use log::{info,debug};
use env_logger::{Env};
use std::collections::HashMap;
use std::io::{Read,Write};
use std::net::{TcpListener, TcpStream};


const NULL_BYTE: &str = "\0";
const CHUNK_SIZE: usize = 1024;

struct RedisServer {
    pub ip_addr: String,
    pub port_num: u16,
    // pub cache: HashMap<String, &mut [u8]>
}

impl RedisServer {
    // fn handle_echo_cmd(&self, resp_array: Vec<&str>);

    // fn handle_ping_cmd(&self, resp_array: Vec<&str>);

    // fn handle_set_cmd(&self, resp_array: Vec<&str>);

    // fn handle_get_cmd(&self, resp_array: Vec<&str>);

    async fn handle_connection(stream: &mut TcpStream) -> anyhow::Result<()> {
        let mut read_buffer = [0; CHUNK_SIZE];
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
                    let _num_elems = first_elem[1..].parse::<usize>().expect("First element of client request is not a number!");
                    for (i, req_part) in resp_array.iter().enumerate() {
                        match req_part.to_uppercase().as_str() {
                            "PING" => {
                                stream.write(b"+PONG\r\n").expect("Writing PING response to stream failed!");
                            },
                            "ECHO" => {
                                let echo_output = format!("+{}\r\n", resp_array.get(i+2).expect("Couldn't find ECHO output."));
                                let echo_output_bytes = echo_output.as_bytes();
                                stream.write(echo_output_bytes).expect("Writing ECHO response to stream failed!");
                            },
                            "" => debug!("Reached end of input."),
                            other_input => debug!("Input: {} is currently not handled.", other_input),
                        };
                    }
                },
                None => bail!("No data after split by null byte"),
            }
        }

        Ok(())
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        let tcp_listener_addr = format!(
            "{}:{}",
            self.ip_addr,
            self.port_num
        );
        let tcp_listener = TcpListener::bind(tcp_listener_addr).unwrap();
        for stream in tcp_listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    info!("Accepted new connection");
                    /* tokio::spawn creates an async task that runs the future (I/O function) passed as argument
                    Returns a Result<JoinHandle> (i.e. spawned async task) */
                    tokio::spawn(async move {
                        // Within same connection, accept multiple commands in loop; if # bytes read is 0, exit connection
                        Self::handle_connection(&mut stream).await.expect("Something went wrong while handling connection.");
                    });
                }
                Err(e) => {
                    bail!("Error in accepting TCP connection: {}", e);
                }
            }
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let env = Env::default().default_filter_or("debug");
    env_logger::init_from_env(env);

    let redis_server = RedisServer {
        ip_addr: String::from("127.0.0.1"),
        port_num: 6379,
        // cache: HashMap::new()
    };
    redis_server.run().await;

    Ok(())
}
