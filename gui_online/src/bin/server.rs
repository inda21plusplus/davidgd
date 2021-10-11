#![warn(rust_2018_idioms)]

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

use std::env;
use std::error::Error;

use std::{num::ParseIntError};

pub fn decode_binary(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(9)
        .map(|i| u8::from_str_radix(&s[i..i + 8], 2))
        .collect()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:1337".to_string());

    let listener = TcpListener::bind(&addr).await?;
    println!("Listening on: {}", addr);

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = vec![0; 1024];

            loop {
                let n = socket
                    .read(&mut buf)
                    .await
                    .expect("failed to read data from socket");

                if n == 0 {
                    return;
                }
                
                let message = decode_message(&buf);

                println!("{}", message);

                let message = encode_message("rnbqkbnr/pppppppp/8/6P/6p/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string());


                socket
                    .write_all(&message)
                    .await
                    .expect("failed to write data to socket");
            }
        });
    }
}

fn decode_message(buf:  &Vec<u8>) -> String {
    let message: String = buf.iter().map(|x| char::from(*x)).collect();
    let message = message.lines().next().unwrap();
    let message = message.to_string();
    message
}

fn encode_message(message: String) -> Vec<u8> {
    let mut message_vec = message.into_bytes();
    message_vec.resize(1024, 0);
    message_vec
}
