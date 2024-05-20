/**
 * $File: server.rs $
 * $Date: 2024-05-17 22:19:28 $
 * $Revision: $
 * $Creator: Jen-Chieh Shen $
 * $Notice: See LICENSE.txt for modification and distribution information
 *                   Copyright © 2024 by Shen, Jen-Chieh $
 */
use crate::handler;
use crate::packet;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

const SEPARATOR_LEN: usize = "\r\n".len();
const BUF_SIZE: usize = 1024 * 1;

fn get_content_len(line: &str) -> usize {
    if !line.starts_with("Content-Length: ") {
        tracing::error!("Invalid content length: {:?}", line);
        return 0;
    }
    let rm_len = "Content-Length: ".len();
    let len_str = &line[rm_len..];
    len_str.parse::<usize>().unwrap()
}

struct Connection {
    pub socket: tokio::net::TcpStream,
    pub addr: std::net::SocketAddr,
    read_buf: [u8; BUF_SIZE],
    data: Vec<u8>,
    packets: Vec<packet::Packet>,
}

impl Connection {
    pub fn new(_socket: tokio::net::TcpStream, _addr: std::net::SocketAddr) -> Self {
        let connection = Self {
            socket: _socket,
            addr: _addr,
            read_buf: [0; BUF_SIZE],
            data: Vec::new(),
            packets: Vec::new(),
        };
        connection
    }

    pub async fn run(&mut self) {
        // In a loop, read data from the socket and write the data back.
        loop {
            self.read().await;
            // Write the data back
            //self.write(self.read_buf).await;
        }
    }

    pub async fn read(&mut self) {
        let _ = match self.socket.read(&mut self.read_buf).await {
            // socket closed
            Ok(n) if n == 0 => return,
            Ok(n) => {
                tracing::trace!("{} ({:?})", self.to_string(), n);

                // Add new data to the end of data buffer.
                {
                    let new_data = &self.read_buf[0..n];
                    self.data.append(&mut new_data.to_vec());
                }

                self.process();

                n
            }
            Err(e) => {
                println!("failed to read from socket; err = {:?}", e);
                return;
            }
        };
    }

    pub fn process(&mut self) {
        let decrypted = String::from_utf8_lossy(&self.data);

        let chopped = decrypted.split("\r\n");
        let size = chopped.clone().count();

        if size < 3 {
            return;
        }

        let mut content_len: usize = 0;
        let mut op = 0;
        let mut boundary = 0;
        let mut process = false;

        for line in chopped {
            let current_op = op % 3;

            match current_op {
                0 => {
                    boundary += line.len() + SEPARATOR_LEN;
                    content_len = get_content_len(line);
                }
                1 => {
                    boundary += line.len() + SEPARATOR_LEN;
                }
                2 => {
                    if content_len <= line.len() {
                        boundary += content_len;

                        let data = &line[..content_len];
                        handler::handle(data);
                        //println!("{}: {}", "receive all", data);

                        process = true;
                        break;
                    }
                }
                _ => {
                    tracing::error!("Invalid operation id: {:?}", current_op);
                }
            }
            op += 1;
        }

        if process {
            self.data = self.data[boundary..].to_vec();
            tracing::trace!(
                "data left ({}) {:?}",
                boundary,
                String::from_utf8_lossy(&self.data)
            );
            self.process();
        }
    }

    pub async fn write(&mut self, buf: [u8; BUF_SIZE]) {
        if let Err(e) = self.socket.write_all(&buf).await {
            tracing::warn!("failed to write to socket {:?}; err = {:?}", self.socket, e);
            return;
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}", &self.addr)
    }
}

pub struct Server {
    host: String,
    port: u16,
    path: String,
    password: String,
    connections: Vec<Connection>,
}

impl Server {
    pub fn new(_host: &str, _port: u16, _path: &str, _password: &str) -> Self {
        Self {
            host: _host.to_string(),
            port: _port,
            path: _path.to_string(),
            password: _password.to_string(),
            connections: Vec::new(),
        }
    }

    fn addr(&mut self) -> String {
        self.host.to_string() + ":" + &self.port.to_string()
    }

    pub async fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        tracing::info!("Listening on port {}", self.addr());

        let listener = TcpListener::bind(self.addr()).await?;

        loop {
            let (socket, addr) = listener.accept().await?;
            let mut connection = Connection::new(socket, addr);
            tracing::info!("New connection from {}", connection.to_string());

            //self.connections.push(connection);

            tokio::spawn(async move {
                connection.run().await;
            });
        }
    }
}
