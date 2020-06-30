#[cfg(test)]
mod tests {
    use encrypt::des::DES;
    use encrypt::rsa::RSA;
    #[test]
    fn rsa_test() {
        let rsa = RSA::new();
        let (pub_k, priv_k) = (rsa.get_pub_key(), rsa.get_priv_key());
        println!(
            "pub key is {} {}, private key is {} {}",
            pub_k.0, pub_k.1, priv_k.0, priv_k.1
        );
        let value = 5551;
        assert_eq!(value, RSA::decrypt(priv_k, RSA::encrypt(pub_k, value)));
    }
    #[test]
    fn all_test() {
        let rsa = RSA::new();
        let (pub_k, priv_k) = (rsa.get_pub_key(), rsa.get_priv_key());
        let value="hello,rust/r/n".to_string();
        let value_clone=value.clone();
        let des=DES::new();
        let mut des_key=des.get_key();
       // des_key=5551;
        let des_value= DES::encrypt_with_key(des_key, value);
        println!("key_u4 is {},key_i64 is{}",des_key,des_key as i64);
        let pub_des_key=RSA::encrypt(priv_k, des_key as i64);
        let de_des_key=RSA::decrypt(pub_k, pub_des_key);
        assert_eq!(des_key as i64,de_des_key);
        let de_value=DES::decrypt_with_key(de_des_key as u64, des_value);
        assert_eq!(value_clone,de_value);
    }
}
