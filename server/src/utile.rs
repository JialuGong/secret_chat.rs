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

pub fn key_to_str(key:(i64,i64))->String{
   format!("{},{}",key.0.to_string(),key.1.to_string())
}
// pub struct Lines {
//     rsa: RSA,
// }
// impl Lines {
//     pub fn new(rsa: RSA) -> Self {
//         Lines { rsa }
//     }
//     pub fn wrap_line(&self, line: String) -> String {
//         let des = DES::new();
//         let des_key = des.get_key();
//         let des_key_code = self.rsa.encrypt(des_key as i32);
//         let message = Message::new(des_key_code, des.encrypt(line));
//         serde_json::to_string(&message).unwrap()
//     }
//     pub fn unwrap_line(&self, line: String) -> String {
//         let message: Message = serde_json::from_str(&line).unwrap();
//         let des_key = self.rsa.decrypt(message.key);
//         DES::decrypt_with_key(des_key as u64, message.value)
//     }
// }
