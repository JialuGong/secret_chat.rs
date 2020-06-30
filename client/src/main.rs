mod utile;
extern crate bytes;
extern crate encrypt;
extern crate tokio;

use utile::{str_to_key, MessageCode};

use std::error::Error;
use std::{env, io};

use encrypt::rsa::RSA;
use futures::stream::StreamExt;

use tokio::io::{BufReader, BufWriter};
use tokio::net::TcpStream;
use tokio::prelude::*;
use tokio::{sync::mpsc, task};
use tokio_util::codec::{Framed, LinesCodec, LinesCodecError};

#[tokio::main]

async fn main() -> Result<(), Box<dyn Error>> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:6142".to_string());
    let stream = TcpStream::connect(&addr).await.unwrap();
    process(stream, &addr).await?;

    Ok(())
}

async fn process(mut stream: TcpStream, addr: &String) -> Result<(), Box<dyn Error>> {
    // let mut lines=Framed::new(stream,LinesCodec::new());
    println!("Connecto to server : {}", &addr);
    let mut buf = String::new();
    
    let (read_half,mut writer) = stream.into_split();
    let mut reader = BufReader::new(read_half);
    //let mut writer = BufWriter::new(write_half);

    reader.read_line(&mut buf).await?;
    println!("key is {}", buf);
    let pub_key = str_to_key(buf);
    println!("pub key is {},{}", pub_key.0, pub_key.1);
    writer.write_all(b"-- client receive the key\r\n").await?;
    let message_coder = MessageCode::new(pub_key);
    let message_coder_clone = message_coder.clone();

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
            let data_encode = format!("{}\r\n", message_coder.encode(line));
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
                    let data_decode = message_coder_clone.decode(buf);
                    println!("from server:\n {}", data_decode)
                }
            }
            _ => {}
        }
    }

    // loop {
    //     let mut buf = String::new();
    //     match stream_buf.read_line(&mut buf).await {
    //         Ok(size) => {
    //             if size > 2 {
    //                 println!("buf {}, size is {}", buf, size);
    //                 let data_decode = message_coder.decode(buf);
    //                 tx_out.send(data_decode).await?;
    //             }
    //         }
    //         _ => {}
    //     }

    //     match rx_in.recv().await {
    //         Some(data) => {
    //             println!("{}", data);
    //             let data_encode = format!("{}\r\n", message_coder.encode(data));
    //             stream_buf.write_all(data_encode.as_bytes()).await?;
    //             //tx_out.send(data).await.unwrap();
    //         }
    //         None => {}
    //     }
    // }
}
