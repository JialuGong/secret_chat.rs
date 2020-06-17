#[cfg(test)]
mod tests {
    use encrypt::rsa::RSA;
    use encrypt::des::DES;
    // #[test]
    // fn rsa_test() {
    //     let rsa = RSA::new();
    //     let (pub_k, prvi_k) = (rsa.get_pub_key(), rsa.get_priv_key());
    //     let value=8954;
    //     assert_eq!(value,rsa.decrypt(rsa.encrypt(value)));
    // }
    #[test]
    fn des_test(){
        let des=DES::new();
        let value="hello world \n hello rust".to_string();
        assert_eq!(value,des.decrypt(des.encrypt(value.clone())));
    }
}
