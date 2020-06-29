mod utile;
extern crate bytes;
extern crate encrypt;
extern crate tokio;

use std::error::Error;
use std::{io,env};
use crate::utile::Lines;
use encrypt::rsa::RSA;
use futures::stream::StreamExt;

use tokio::net::tcp::{ReadHalf, WriteHalf};
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;
use tokio::{sync::mpsc, task};
use tokio_util::codec::{Framed, LinesCodec, LinesCodecError};


#[tokio::main]
async fn main()->Result<(),Box<dyn Error>>{
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:6142".to_string());
    let mut stream=TcpStream::connect(addr).await.unwrap();
    process(stream)
    
}

async fn process(stream:TcpStream)-> Result<(), Box<dyn Error>>{
    let mut lines=Framed::new(stream,LinesCodec::new());
    let pub_key=match  lines.next().await {
        Some(Ok(key))=>{
            key
        },
        _=>{
            println!("Failed to get username from {}. Client disconnected.", addr);
            return Ok(());
        }
    };


     //let (write_half,read_half)=lines.split();
    let (mut tx_in,mut rx_in)=mpsc::channel::<String>(800000);
    let (mut tx_out,mut rx_out)=mpsc::channel::<String>(800000);


    //read from the console
    task::spawn(async move{
        loop{
            let mut line=String::new();
            line=task::spawn_blocking(move||{
                 io::stdin().read_line(&mut line).unwrap();
                line
            }).await?;
             tx_in.send(line.trim().to_string()).await.unwrap();
        }
    });


    //write to the console
    task::spawn(async move{
        loop{
            match rx_out.recv().await {
                Some(data) => {
                    task::spawn_blocking(move || {
                        println!("Message from client:\n{}", data);
                    }).await.unwrap();
                }, 

                None => ()
            }
        }
    });

    loop{
        match rx_in.recv().await{
            Some(data)=>{
            lines.send(data).await?;
                //tx_out.send(data).await.unwrap();
            }
            None=>{}
        }
        match lines.next().await{
            Some(Ok(data))=>{
                tx_out.send(data).await?;
            }
            None=>{}
        }
    }
    Ok(())

}