use base64::Engine;

use crate::crypter::Crypter;

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
#[ignore = "TODO: account for new changes to encryption"]
fn simple2() {
    let key = [1u8; 32];
    let aad_len = 5;
    let crypter = Crypter::new(key);

    let ciphertext = crypter
        .encrypt_with_nonce(11, "Hello".as_bytes(), "world".as_bytes())
        .unwrap();

    let base = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(&ciphertext);
    assert_eq!(base, "d29ybGQAAAAAAAAACwAAAABTWUvZlvpf-bvKds1iZYresLOf_gg");

    let (aad, plaintext) = crypter
        .decrypt_with_nonce(11, &ciphertext, aad_len)
        .unwrap();

    assert_eq!(aad, "world".as_bytes());
    assert_eq!(plaintext, "Hello".as_bytes());
}

#[test]
fn simple3() {
    let key = [1u8; 32];
    let aad_len = 5;

    let crypter = Crypter::new(key);

    let data = "Hello".as_bytes();
    let aad = "world".as_bytes();

    let ciphertext = crypter.encrypt(data, aad).unwrap();

    // let base = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(&ciphertext);
    // assert_eq!(base, "d29ybGRROiQ0PaG1PtRcHnFucDoEyAuLfv7dQ0CCIezxWneJsaQ");

    let (aad, _iv, plaintext) = crypter.decrypt(&ciphertext, aad_len).unwrap();

    assert_eq!(aad, "world".as_bytes());
    assert_eq!(plaintext, "Hello".as_bytes());
}
