use base64::Engine;

use crate::Crypter;

#[test]
fn simple() {
    // openssl::init();
    let key = [1u8; 16];
    let iv = [1u8; 12];

    let mut tag = [0u8; 16];
    let ciphertext = openssl::symm::encrypt_aead(
        openssl::symm::Cipher::aes_128_ocb(),
        &key,
        Some(&iv),
        "Hello???".as_bytes(),
        "WORLD!!!".as_bytes(),
        &mut tag,
    )
    .map_err(|x| x.to_string())
    .unwrap();

    assert_eq!(ciphertext, [171, 237, 64, 3, 14, 91, 10, 36]);

    // let mut tag = [0u8; 16];
    let ciphertext = openssl::symm::decrypt_aead(
        openssl::symm::Cipher::aes_128_ocb(),
        &key,
        Some(&iv),
        "Hello???".as_bytes(),
        &ciphertext,
        &tag,
    )
    .map_err(|x| x.to_string())
    .unwrap();

    assert_eq!(ciphertext, "WORLD!!!".as_bytes());
}

#[test]
fn simple2() {
    let key = [1u8; 16];
    let crypter = Crypter::<5>::new(key);

    let ciphertext = crypter.encrypt(
        11,
        "Hello".as_bytes(),
        "world".as_bytes().try_into().unwrap(),
    );

    let base = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(&ciphertext);
    assert_eq!(base, "d29ybGRwNRw_5f3KIVL0EEyMDaDLdA5jQjI");

    let (aad, plaintext) = crypter.decrypt(11, &ciphertext).unwrap();

    assert_eq!(aad, "world".as_bytes());
    assert_eq!(plaintext, "Hello".as_bytes());
}
