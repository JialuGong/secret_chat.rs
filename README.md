# secret chat

A simple c/s console  chat tool.

#### How do we encrypt
```
server-----------(RSA)------------client
  |                                |
(private_rsa)                (public_rsa)
                                          |
  message                                 |
    |                                     |
   (des)--(rsa)---encrypt_des_key         |
    |                 |                   |
encrypt_message________________________(socket)

```

#### 还需要提高的姿势水平(What should I  know about socket in rust)

- the difference bettween `split` and `into_split`
```rust 
 let stream = TcpStream::connect(&addr).await.unwrap();
 //fake split.You ONLY GET THE BORROW VALUE OF STREAM
 let(read,write)=stream.split();

 //real split.you get the ownership 
 let(read,write)=stream.into_split();
```