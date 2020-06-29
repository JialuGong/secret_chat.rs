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