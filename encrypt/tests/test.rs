#[cfg(test)]
mod tests {
    use encrypt::rsa::RSA;
    use encrypt::des::DES;
    #[test]
    fn rsa_test() {
        let rsa = RSA::new();
        let (pub_k, priv_k) = (rsa.get_pub_key(), rsa.get_priv_key());
        println!("pub key is {} {}, private key is {} {}",pub_k.0,pub_k.1,priv_k.0,priv_k.1);
        let value=9999;
        assert_eq!(value,RSA::decrypt(priv_k, RSA::encrypt(pub_k, value)));
    }
    #[test]
    fn des_test(){
        let des=DES::new();
        let value="hello world \n hello rust".to_string();
        assert_eq!(value,des.decrypt(des.encrypt(value.clone())));
    }
}
