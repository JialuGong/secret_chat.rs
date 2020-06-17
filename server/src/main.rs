mod utile;
extern crate tokio;
extern crate bytes;
extern crate encrypt;
use tokio::net::{TcpListener,TcpStream};
use tokio::prelude::*;
use tokio::net::tcp::{ReadHalf,WriteHalf};
use futures::stream::StreamExt;


#[tokio::main]
async fn main(){
    let addr="127.0.0.1:3680";
    let mut listener = TcpListener::bind(addr).await.unwrap();
    let server=async move{
        let mut incomming=listener.incoming();
        while let Some(socket_res)=incomming.next().await{
            match socket_res{
                Ok(socket)=>{
                   tokio::spawn(async move {
                       excute(socket);
                   });
                }
                Err(err)=>{
                    println!("Accept error ={:?}",err);
                }
            }
        }
    };
    println!("Server running on {}",addr);
    server.await;
}

fn excute(mut socket:TcpStream){
    let (mut reader,mut writer)=socket.split();
    let rsa_priv=distribute_rsa(&mut writer);

}
fn distribute_rsa(writer:&mut WriteHalf)->String{
    let rsa=RSA::new();
    let(pub_key,priv_key)=(rsa.get_pub_key(),rsa.get_priv_key());
    //TODO
    unimplemented!();
}
fn secrete_chant(){
    //TODO
    unimplemented!();
}