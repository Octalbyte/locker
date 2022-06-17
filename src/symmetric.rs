
mod bytes {
    use aes_gcm_siv::aead::{Aead, NewAead};
    use aes_gcm_siv::{Aes256GcmSiv, Key, Nonce}; // Or `Aes128GcmSiv`
    use std::str;
    pub fn encrypt() -> String {

        let key = Key::from_slice(b"an example very very secret key.");
        let cipher = Aes256GcmSiv::new(key);

        let nonce = Nonce::from_slice(b"unique nonce"); // 96-bits; unique per message

        let ciphertext = cipher
            .encrypt(nonce, b"plaintext message".as_ref())
            .expect("encryption failure!"); // NOTE: handle this error to avoid panics!

        let cipherstring = match str::from_utf8(&ciphertext) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        let plaintext = cipher
            .decrypt(nonce, ciphertext.as_ref())
            .expect("decryption failure!"); // NOTE: handle this error to avoid panics!

        assert_eq!(&plaintext, b"plaintext message");
        return String::from(cipherstring);
    }
    pub fn decrypt() -> String {
        let key = Key::from_slice(b"an example very very secret key.");
        let cipher = Aes256GcmSiv::new(key);
        let nonce = Nonce::from_slice(b"unique nonce"); // 96-bits; unique per message
        let ciphertext = vec![];
        let plaintext = cipher
            .decrypt(nonce, ciphertext.as_ref())
            .expect("failed");
        let plainstring = match str::from_utf8(&plaintext) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        return String::from(plainstring);

    }

}
