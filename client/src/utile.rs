use encrypt::des::DES;
use encrypt::rsa::RSA;
use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize, Debug)]
 struct Message {
    key: i64,
    value: Vec<u64>,
}

impl Message {
    pub fn new(key: i64, value: Vec<u64>) -> Self {
        Message {
            key: key,
            value: value,
        }
    }
}

#[derive(Clone,Copy)]
pub struct MessageCode {
    key: (i64, i64),
}
impl MessageCode {
   pub  fn new(key: (i64, i64)) -> MessageCode {
        MessageCode { key }
    }
    pub fn encode(&self, data: String) -> String {
        let des = DES::new();
        let des_key = des.get_key();
        let des_key_code = RSA::encrypt(self.key, des_key as i64);
        let message = Message::new(des_key_code, des.encrypt(data));
        serde_json::to_string(&message).unwrap()
    }
    pub fn decode(&self, data: String) -> String {
        let message: Message = serde_json::from_str(&data).unwrap();
        let des_key =RSA::decrypt(self.key, message.key);
        println!("des_key is {}",des_key);
        DES::decrypt_with_key(des_key as u64, message.value)
    }
    pub fn encode_with_key(key:(i64,i64),data:String)->String{
        let des = DES::new();
        let des_key = des.get_key();
        let des_key_code = RSA::encrypt(key, des_key as i64);
        let message = Message::new(des_key_code, des.encrypt(data));
        serde_json::to_string(&message).unwrap()

    }
}

pub fn str_to_key(data:String)->(i64,i64){
    let strs=data.split(",").collect::<Vec<&str>>();
    println!("strs is {},{}",strs[0],strs[1]);
    (str_to_i64(strs[0]),str_to_i64(strs[1]))
}
fn str_to_i64(data:&str)->i64{
    println!("str is b{}b",data);
    data.chars().filter(|&x|x as i32>=48&&(x as i32<=58)).rev().enumerate().map(|(i,x)|{
       ( x as i64 -48)*10i64.pow(i as u32)
    }).sum::<i64>()
}