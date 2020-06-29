use bytes::BytesMut;
use encrypt::des::DES;
use encrypt::rsa::RSA;
use serde::{Deserialize, Serialize};
use std::io;
use tokio::net::tcp::{ReadHalf, WriteHalf};
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;
use tokio::{sync::mpsc, task};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    key: i32,
    value: Vec<u64>, 
}

impl Message {
    pub fn new(key: i32, value: Vec<u64>) -> Self {
        Message {
            key: key,
            value: value,
        }
    }
}
pub struct Lines {
    rsa: RSA,
}
impl Lines {
    pub fn new(rsa:RSA)->Self{
        Lines{
            rsa,
        }
    }
    pub fn wrap_line(&self, line: String) -> String {
        let des = DES::new();
        let des_key = des.get_key();
        let des_key_code = self.rsa.encrypt(des_key as i32);
        let message = Message::new(des_key_code, des.encrypt(line));
        serde_json::to_string(&message).unwrap()
    }
    pub fn unwrap_line(&self, line: String) -> String {
        let message: Message = serde_json::from_str(&line).unwrap();
        let des_key = self.rsa.decrypt(message.key);
        DES::decrypt_with_key(des_key as u64, message.value)
    }
}

