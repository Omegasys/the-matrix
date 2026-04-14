use crate::core::security::encryption::Encryptor;

#[test]
fn test_encrypt_decrypt() {
    let key = [0u8; 32];
    let encryptor = Encryptor::new(&key);

    let data = b"secret data";

    let encrypted = encryptor.encrypt(data).unwrap();
    let decrypted = encryptor.decrypt(&encrypted).unwrap();

    assert_eq!(decrypted, data);
}
