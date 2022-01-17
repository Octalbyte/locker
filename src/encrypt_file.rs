pub mod encrypt_file {

    extern crate rand;
    use rand::rngs::OsRng;
    use rsa::{PaddingScheme, PublicKey, RsaPrivateKey, RsaPublicKey};
    use std::fs;
    use std::io::Write;
    use std::str;

    pub fn hello() {
        //complete uselessness
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

        let t = match str::from_utf8(&dec_data) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };

        println!("result -> {}", t);
    }

    pub fn write_create(
        path: &str,
        buf: &Vec<u8>,
        key: &RsaPublicKey,
    ) -> Result<(), std::io::Error> {
        let mut rng = OsRng;
        let enc_data = key
            .encrypt(&mut rng, PaddingScheme::new_pkcs1v15_encrypt(), &buf[..])
            .expect("failed to encrypt");

        let mut file = fs::File::create(path)?;
        // Write a slice of bytes to the file
        file.write_all(&enc_data)?;
        Ok(())
    }
}
