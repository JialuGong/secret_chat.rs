extern crate rand;
use rand::Rng;
const LIMIT: i32 = 10000;
pub struct RSA {
    pub_key: (i32, i32),
    priv_key: (i32, i32),
}
impl RSA {
    pub fn new() -> Self {
        let (pub_k, priv_k) = Self::gen_keys();
        RSA {
            pub_key: pub_k,
            priv_key: priv_k,
        }
    }
    pub fn get_pub_key(&self) -> (i32, i32) {
        return self.pub_key;
    }
    pub fn get_priv_key(&self) -> (i32, i32) {
        return self.priv_key;
    }
    pub fn encrypt(&self, value: i32) -> i32 {
        Self::log_power(value, self.pub_key)
    }
    pub fn decrypt(&self, value: i32) -> i32 {
        Self::log_power(value, self.priv_key)
    }

    fn gen_keys() -> ((i32, i32), (i32, i32)) {
        let ( p, q) = (Self::genrate_prime(), Self::genrate_prime());
        let n = p * q;
        let phi = (p - 1) * (q - 1);
        let e = Self::generate_coprime(phi);
        let d = Self::modular_inverse(e, phi);
        ((n, e), (n, d))
    }
    fn log_power(mut n: i32, key: (i32, i32)) -> i32 {
        let (mut p, m) = key;
        let mut res = 1;
        loop {
            if p == 0 {
                break;
            }
            res = ((1i64 * res as i64 * n as i64) % m as i64) as i32;
            n = ((1i64 * n as i64 * n as i64) % m as i64) as i32;
            p=p>>1;
        }
        res
    }
    fn ranbin_miller(n: i32) -> bool {
        let mut ok = true;
        for _i in 0..5 {
            let a = Self::rand_i32() + 1;
            let res = Self::log_power(a, (n - 1, n));
            ok &= res == 1;
        }
        ok
    }
    fn rand_i32() -> i32 {
        let mut rng = rand::thread_rng();
        let res: i32 = rng.gen_range(1, LIMIT);
        res
    }
    fn genrate_prime() -> i32 {
        let mut generated = Self::rand_i32();
        while !Self::ranbin_miller(generated) {
            generated = Self::rand_i32();
        }
        return generated;
    }
    fn gcd(mut a: i32, mut b: i32) -> i32 {
        while b > 0 {
            let r = a % b;
            a = b;
            b = r;
        }
        return a;
    }
    fn generate_coprime(n: i32) -> i32 {
        let mut generated = Self::rand_i32();
        while Self::gcd(n, generated) != 1 {
            generated = Self::rand_i32();
        }
        return generated;
    }
    fn euclid_extended(a: i32, b: i32) -> (i32, i32) {
        if b == 0 {
            return (1, 0);
        } else {
            let (c, d) = Self::euclid_extended(a, a % b);
            return (d, c - (a / b));
        }
    }
    fn modular_inverse(n: i32, m: i32) -> i32 {
        let (mut invers, _) = Self::euclid_extended(n, m);
        while invers < 0 {
            invers += m;
        }
        return invers;
    }
}
