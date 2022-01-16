pub mod hello {

    pub fn hello() {
        extern crate rand;
        use rand::rngs::OsRng;
        use rsa::{PaddingScheme, PublicKey, RsaPrivateKey, RsaPublicKey};
        use std::str;

        let mut rng = OsRng;
        let bits = 2048;
        let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
        let pub_key = RsaPublicKey::from(&priv_key);

        // Encrypt
        let data = b"hello world";
        let enc_data = pub_key
            .encrypt(&mut rng, PaddingScheme::new_pkcs1v15_encrypt(), &data[..])
            .expect("failed to encrypt");
        assert_ne!(&data[..], &enc_data[..]);

        // Decrypt
        let dec_data = priv_key
            .decrypt(PaddingScheme::new_pkcs1v15_encrypt(), &enc_data)
            .expect("failed to decrypt");
        assert_eq!(&data[..], &dec_data[..]);

        println!("{:?} -> {:?}", enc_data, dec_data);

        let s = match str::from_utf8(&enc_data) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        let t = match str::from_utf8(&dec_data) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };

        println!("result: {} -> {}", s, t);
    }
}
