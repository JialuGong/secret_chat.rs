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
use tokio::stream::Stream;
use tokio::sync::{mpsc, Mutex};
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

    let mut stream_buf = BufReader::new(stream);
    let mut key_str = format!("{}\r\n", key_to_str(pub_key));
    key_str.push_str("\r\n");
    println!("your key is {}", key_str);
    println!("client  key is {}", key_str);

    stream_buf.write_all(key_str.as_bytes()).await?;
    // stream.write_all ().await?;
    let mut buf = String::new();
    let _is_rec = match stream_buf.read_line(&mut buf).await {
        Ok(s) => {}
        _ => {
            println!("Failed to get response from {}. Client disconnected.", addr);
            return Ok(());
        }
    };
    println!("{}", buf);
    let (mut tx_in, mut rx_in) = mpsc::channel::<String>(800000);
    let (mut tx_out, mut rx_out) = mpsc::channel::<String>(800000);

    //read from the console
    task::spawn(async move {
        loop {
            let mut line = String::new();
            line = task::spawn_blocking(move || {
                io::stdin().read_line(&mut line).unwrap();
                println!("read from console:{}", line);
                line
            })
            .await
            .unwrap();
            tx_in.send(line.trim().to_string()).await.unwrap();
        }
    });

    //write to the console
    task::spawn(async move {
        loop {
            match rx_out.recv().await {
                Some(data) => {
                    task::spawn_blocking(move || {
                        println!("Message from client:\n{}", data);
                    })
                    .await
                    .unwrap();
                }

                None => (),
            }
        }
    });

    loop {
        let mut buf = String::new();
        match rx_in.recv().await {
            Some(data) => {
                println!("-read from the channel you type {}-", data);
                let data_encode = format!("{}\r\n", MessageCode::encode_with_key(priv_key, data));
                println!("console data after encode {}", data_encode);
                stream_buf.write_all(data_encode.as_bytes()).await?;
                //tx_out.send(data).await.unwrap();
            }
            None => {}
        }

        match stream_buf.read_line(&mut buf).await {
            Ok(size) => {
                println!("size is {}", size);
                if size > 2 {
                    let data_decode = messageCoder.decode(buf);
                    tx_out.send(data_decode).await?;
                }
            }
            _ => {}
        }
    }
    Ok(())
}
