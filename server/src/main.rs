mod utile;
extern crate bytes;
extern crate encrypt;
extern crate tokio;
//use crate::utile::Lines;
use encrypt::rsa::RSA;
use futures::stream::StreamExt;

use tokio_util::codec::{Framed, LinesCodec, LinesCodecError};

use std::collections::HashMap;
use std::error::Error;
use std::net::SocketAddr;
use std::sync::Arc;
use std::{env, io};
use std::pin::Pin;
use std::task::{Context, Poll};


//use tokio::net::tcp::{ReadHalf, WriteHalf};
use tokio::net::{TcpListener, TcpStream};
use tokio::stream::{Stream};
use tokio::prelude::*;
use tokio::sync::{mpsc, Mutex};
use tokio::task;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let state = Arc::new(Mutex::new(Shared::new()));
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:6142".to_string());

    let mut listener = TcpListener::bind(&addr).await.unwrap();
    println!("server running on {}", addr);

    loop {
        let (stream, addr) = listener.accept().await.unwrap();
        let state = Arc::clone(&state);
        tokio::spawn(async move {
            if let Err(e) = process(state, stream, addr).await {
                println!("an error occurred; error = {:?}", e);
            }
        });
    }
}

// Shorthand for the transmit half of the message channel.
type Tx = mpsc::UnboundedSender<String>;

/// Shorthand for the receive half of the message channel.
type Rx = mpsc::UnboundedReceiver<String>;

struct Shared {
    peers:HashMap<SocketAddr,Tx>
}

impl Shared {
    fn new() -> Shared {
        Shared {
            peers:HashMap::new()
        }
    }
}
struct Peer{
    lines:Framed<TcpStream,LinesCodec>,
    rx:Rx,
}
impl Peer{
    async fn new(
        state:Arc<Mutex<Shared>>,
        lines:Framed<TcpStream,LinesCodec>
    )->io::Result<Peer>{
        let addr=lines.get_ref().peer_addr()?;
        let(tx,rx)=mpsc::unbounded_channel();
        state.lock().await.peers.insert(addr,tx);
        Ok(Peer{lines,rx})
    }
}
/**
*Process an individual chat client
*/
async fn process(
    state: Arc<Mutex<Shared>>,
    stream: TcpStream,
    addr: SocketAddr,
) -> Result<(), Box<dyn Error>> {
    let mut lines=Framed::new(stream, LinesCodec::new());
    let rsa=RSA::new();
    //FIXME convert(i64,i64) to string
    lines.send(rsa.get_pub_key()).await?;
    let is_rec=match lines.next().await {
        Some(Ok(line))=>{line},
         _ => {
            println!("Failed to get response from {}. Client disconnected.", addr);
            return Ok(());
        }
    };

     let mut peer = Peer::new(state.clone(), lines).await?;
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
            peer.send(data).await?;
                //tx_out.send(data).await.unwrap();
            }
            None=>{}
        }
        match peer.next().await{
            Some(Ok(data))=>{
                tx_out.send(data).await?;
            }
            None=>{}
        }
        {
            let mut state=state.lock().await;
        }
    }
    
    Ok(())
}

//TODO
struct Message;
impl Message{
    fn to_string(&self)->String{
        
        unimplemented!()

    }
}
impl Stream for Peer{
    type Item=Result<Message,LinesCodecError>;
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        // First poll the `UnboundedReceiver`.

        if let Poll::Ready(Some(v)) = Pin::new(&mut self.rx).poll_next(cx) {
            return Poll::Ready(Some(Ok(Message::Received(v))));
        }

        // Secondly poll the `Framed` stream.
        let result: Option<_> = futures::ready!(Pin::new(&mut self.lines).poll_next(cx));

        Poll::Ready(match result {
            // We've received a message we should broadcast to others.
            Some(Ok(message)) => Some(Ok(Message::Broadcast(message))),

            // An error occurred.
            Some(Err(e)) => Some(Err(e)),

            // The stream has been exhausted.
            None => None,
        })
    }
}
// fn excute(mut socket: TcpStream) {
//     let (mut reader, mut writer) = socket.split();
//     let rsa_priv = distribute_rsa(&mut writer);
// }
// fn distribute_rsa(writer: &mut WriteHalf) -> Lines {
//     let mut rsa = RSA::new();
//     let (pub_key, priv_key) = (rsa.get_pub_key(), rsa.get_priv_key());
//     static lines: Lines = Lines::new(rsa);
//     //TODO
//     unimplemented!();
// }
// async fn secrete_chant(lines: Lines) {
//     //TODO
//     unimplemented!();
// }

// fn on_type<Lines: 'static>(lines: &Lines) {
//     let (mut tx_in, mut rx_in) = mpsc::channel::<String>(800000000);
//     task::spawn(async move {
//         loop {
//             let mut line = String::new();
//             line = task::spawn_blocking(move || {
//                 io::stdin().read_line(&mut line).unwrap();
//                 line
//             })
//             .await
//             .unwrap();
//             line = tx_in.send(line.trim().to_string()).await.unwrap();
//             //TODO
//         }
//     });

//     let (mut tx_out, mut rx_out) = mpsc::channel::<String>(800000000);
//     task::spawn(async move {
//         loop {
//             match rx_out.recv().await {
//                 Some(data) => {
//                     task::spawn_blocking(move || {
//                         //TODO writer
//                     })
//                     .await
//                     .unwrap();
//                 }

//                 None => (),
//             }
//         }
//     });

//     loop {
//         match rx_in.recv().await {
//             Some(data) => {
//                 tx_out.send(data).await.unwrap();
//             }
//             None => (),
//         }
//     }
// }
