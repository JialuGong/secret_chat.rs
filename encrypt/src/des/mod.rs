extern crate num_cpus;
extern crate rand;
mod data;
pub mod des;

use rand::prelude::*;
use std::str;



pub struct DES {
    key: u64,
}
impl DES {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let key: u64 = rng.gen_range(0,i32::max_value() as u64);
        DES { key: key }
    }
    pub fn get_key(&self) -> u64 {
        self.key
    }
    pub fn decrypt_with_key(key:u64,value:Vec<u64>)->String{
        let keys=des::generate_round_keys(key);
        let decode=value.iter().map(|x| des::decrypt_block(*x, keys)).collect::<Vec<u64>>();
        Self::u642string(decode)
    }
    pub fn encrypt(&self, value: String) -> Vec<u64> {
        let keys = des::generate_round_keys(self.key);
        Self::string2u64(value)
            .iter()
            .map(|x| des::encrypt_block(*x, keys))
            .collect::<Vec<u64>>()
    }
    pub fn encrypt_with_key(key:u64,value:String)->Vec<u64>{
        let keys = des::generate_round_keys(key);
        Self::string2u64(value)
            .iter()
            .map(|x| des::encrypt_block(*x, keys))
            .collect::<Vec<u64>>()
    }
    pub fn decrypt(&self, v: Vec<u64>) -> String {
        let keys: [u64; 16] = des::generate_round_keys(self.key);
        let dec = v
            .iter()
            .map(|x| des::decrypt_block(*x, keys))
            .collect::<Vec<u64>>();
        Self::u642string(dec)
    }
    fn string2u64(s: String) -> Vec<u64> {
        let mut ans = Vec::new();
        let mut i = 0;
        let mut tmp = 0;
        for &ch in s.as_bytes() {
            i %= 8;
            tmp = tmp | (ch as u64) << (8 * i);
            if tmp == 0 {
                break;
            }
            if i == 7 {
                ans.push(tmp);
                tmp = 0;
            }
            i += 1;
        }
        ans.push(tmp);
        ans
    }
    fn u642string(v: Vec<u64>) -> String {
        let mut ans = Vec::new();
        'outer: for num in v {
            if num > u64::max_value() {
                panic!("ERROR:INVALID DEC!");
            } else {
                for i in 0..8 {
                    let k = (num >> (8 * i)) & 255;
                    if k == 0 {
                        break 'outer;
                    }
                    ans.push(k as u8);
                }
            }
        }
        str::from_utf8(&ans).unwrap().to_string()
    }
}
