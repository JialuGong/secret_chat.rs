mod utile;
extern crate bytes;
extern crate encrypt;
extern crate tokio;
//use crate::utile::Lines;
use utile::{key_to_str, MessageCode};

use encrypt::rsa::RSA;
use futures::stream::StreamExt;

use tokio_util::codec::{Framed, LinesCodec, LinesCodecError};

use std::error::Error;
use std::net::SocketAddr;
use std::sync::Arc;
use std::task::{Context, Poll};
use std::{env, io};

//use tokio::net::tcp::{ReadHalf, WriteHalf};
use tokio::io::{BufReader, BufWriter};
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;
use tokio::task;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:6142".to_string());

    let mut listener = TcpListener::bind(&addr).await.unwrap();
    println!("server running on {}", addr);

    loop {
        let (stream, addr) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            if let Err(e) = process(stream, addr).await {
                println!("an error occurred; error = {:?}", e);
            }
        });
    }
}

/**
*Process an individual chat client
*/
async fn process(mut stream: TcpStream, addr: SocketAddr) -> Result<(), Box<dyn Error>> {
    println!("connect to {:?}", &addr);
    let rsa = RSA::new();
    let (pub_key, priv_key) = (rsa.get_pub_key(), rsa.get_priv_key());
    // let (pub_key, priv_key) = ((1, 2), (3, 4));
    let messageCoder = MessageCode::new(priv_key);
    let MessageCoder_clone = messageCoder.clone();
    let (read_half, mut writer) = stream.into_split();

    let mut reader = BufReader::new(read_half);
   // let mut writer = BufWriter::new(write_half);
    let mut key_str = format!("{}\r\n", key_to_str(pub_key));
    key_str.push_str("\r\n");
    println!("your key is {}", key_str);
    println!("client  key is {}", key_str);

    writer.write_all(key_str.as_bytes()).await?;
    // stream.write_all ().await?;
    let mut buf = String::new();
    let _is_rec = match reader.read_line(&mut buf).await {
        Ok(s) => {}
        _ => {
            println!("Failed to get response from {}. Client disconnected.", addr);
            return Ok(());
        }
    };
    println!("{}", buf);
    // let (mut tx_in, mut rx_in) = mpsc::channel::<String>(800000);
    // let (mut tx_out, mut rx_out) = mpsc::channel::<String>(800000);

    //read from the console

    task::spawn(async move {
        loop {
            let mut line = String::new();
            line = task::spawn_blocking(move || {
                io::stdin().read_line(&mut line).unwrap();
                line
            })
            .await
            .unwrap();
            let data_encode = format!("{}\r\n", messageCoder.encode(line));
            writer.write_all(data_encode.as_bytes()).await.unwrap();
            // tx_in.send(line.trim().to_string()).await.unwrap();
        }
    });

    //write to the console

    loop {
        let mut buf = String::new();
        match reader.read_line(&mut buf).await {
            Ok(size) => {
                if size > 2 {
                    println!("buf {}, size is {}", buf, size);
                    let data_decode = MessageCoder_clone.decode(buf);
                    println!("from client:\n {}", data_decode)
                }
            }
            _ => {}
        }
    }

    
}
