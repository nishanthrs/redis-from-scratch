use anyhow::bail;
use log::{info,debug};
use env_logger::{Env};
// use std::collections::HashMap;
use std::io::{Read,Write};
use std::net::{TcpListener, TcpStream};
use std::str::FromStr;
use strum_macros::EnumString;


const NULL_BYTE: &str = "\0";
const CHUNK_SIZE: usize = 1024;
const RESP_DELIMITER: &str = "\r\n";

struct RedisServer {
    pub ip_addr: String,
    pub port_num: u16,
    // pub cache: HashMap<String, &mut [u8]>
}

#[derive(Debug, EnumString)]
#[strum(serialize_all = "shouty_snake_case")]
enum Command {
    Ping,
    Echo,
    // Get,
    // Set,
}

impl RedisServer {
    // TODO[1]: Make error-handling more robust; return any errors back to the client

    fn handle_ping_cmd(stream: &mut TcpStream) {
        /* Write to stream the response for PING commands */
        let ping_resp = format!("+PONG{}", RESP_DELIMITER).into_bytes();
        stream.write(&ping_resp).expect("Writing PING response to stream failed!");
    }

    fn handle_echo_cmd(stream: &mut TcpStream, echo_data: Vec<&str>) {
        /* Fetch the echo output and write it to stream */
        assert!(echo_data.len() == 2, "Wrong number of args for ECHO command: {:?}!", echo_data);
        let echo_resp = format!(
            "+{}{}", echo_data.get(1).expect("Couldn't find ECHO output!"), RESP_DELIMITER
        ).into_bytes();
        stream.write(&echo_resp).expect("Writing ECHO response to stream failed!");
    }

    // fn handle_set_cmd(stream: &mut TcpStream, resp_array: Vec<&str>);

    // fn handle_get_cmd(stream: &mut TcpStream, resp_array: Vec<&str>);

    fn handle_cmd(redis_cmd: Command, request: &str, stream: &mut TcpStream) {
        // Should return a Redis RESP array: https://redis.io/docs/reference/protocol-spec
        let resp_array = request.split_terminator(RESP_DELIMITER).collect::<Vec<&str>>();
        match redis_cmd {
            Command::Ping => {
                Self::handle_ping_cmd(stream)
            },
            Command::Echo => {
                Self::handle_echo_cmd(stream, resp_array[3..].to_vec())
            },
            // Command::Get => {
            //     bail!("GET cmd currently not implemented!")
            // },
            // Command::Set => {
            //     bail!("SET cmd currently not implemented!")
            // },
        };
    }

    fn decode_request(request: &str) -> Command {
        /*
        Decode a Redis RESP request string into a RESP array and determine the Redis command

        Example Redis requests as bytes:
        1. PING : request = "*1\r\n$4\r\nPING\r\n"
        2. ECHO "Hello World" : request = "*2\r\n$4\r\necho\r\n$11\r\nHello World\r\n"
        3. GET mykey : request = "*2\r\n$3\r\nGET\r\n$5\r\nmykey\r\n"
        4. SET mykey myval : request = "*3\r\n$3\r\nSET\r\n$5\r\nmykey\r\n$5\r\nmyval\r\n"
        */
        let resp_array = request.split_terminator(RESP_DELIMITER).collect::<Vec<&str>>();  // Should return a Redis RESP array: https://redis.io/docs/reference/protocol-spec
        let first_elem = resp_array.get(0).expect(
            format!("Client request not a valid RESP object; no {} separator found!", RESP_DELIMITER).as_str()
        );
        let num_elems = first_elem[1..].parse::<usize>().expect(
            format!(
                "Request is not a valid RESP array: {}. First element of client request is not a valid array identifier: {}.",
                request,
                first_elem
            ).as_str()
        );
        info!("Number of elements in request: {}", num_elems);
        let cmd: &str = resp_array.get(2).expect(
            format!("Unable to find a command at idx 2 in RESP array: {}", request).as_str()
        );
        Command::from_str(cmd.to_uppercase().as_str()).unwrap()
    }

    async fn handle_connection(stream: &mut TcpStream) -> anyhow::Result<()> {
        /* Handle a given stream/connection/request in an async task */
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
                    let cmd = Self::decode_request(request);
                    Self::handle_cmd(cmd, request, stream);
                },
                None => bail!("No data after split by null byte"),
            }
        }

        Ok(())
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        /*
        Setup a TCP listener on an IP addr and port, listen for incoming requests,
        and spawn an async task to handle the stream/connection/request
        */
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
    /* Init a Redis server and start it */
    let env = Env::default().default_filter_or("debug");
    env_logger::init_from_env(env);

    let redis_server = RedisServer {
        ip_addr: String::from("127.0.0.1"),
        port_num: 6379,
        // cache: HashMap::new()
    };
    redis_server.run().await
}
