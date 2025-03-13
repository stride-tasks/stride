use crate::openssh::{CipherName, KdfName, KeyType, PrivateKey, PrivateSection, PublicSection};

const ED25519_ENCODED: &str =
    "b3BlbnNzaC1rZXktdjEAAAAABG5vbmUAAAAEbm9uZQAAAAAAAAABAAAAMwAAAAtzc2gtZW
QyNTUxOQAAACDb1+CZUNAPGaBf7ShTYyH4em4k18dUW0NJbn9PFbIR1QAAAJAeXjwyHl48
MgAAAAtzc2gtZWQyNTUxOQAAACDb1+CZUNAPGaBf7ShTYyH4em4k18dUW0NJbn9PFbIR1Q
AAAECtAPI8CAnjOYMMhQKz5ASRgOmx6VrUfCn4TCG0oIywqNvX4JlQ0A8ZoF/tKFNjIfh6
biTXx1RbQ0luf08VshHVAAAADGFyY2hAYXJjaC1wYwE=";

fn ed25519_decoded() -> PrivateKey {
    PrivateKey {
        cipher_name: CipherName::None,
        kdf_name: KdfName::None,
        kdf: vec![],
        public_section: PublicSection {
            keytype: KeyType::Ed25519,
            pub0: vec![
                219, 215, 224, 153, 80, 208, 15, 25, 160, 95, 237, 40, 83, 99, 33, 248, 122, 110,
                36, 215, 199, 84, 91, 67, 73, 110, 127, 79, 21, 178, 17, 213,
            ],
        },
        private_section: PrivateSection {
            checkint: 509_492_274,
            keytype: KeyType::Ed25519,
            priv0: vec![
                173, 0, 242, 60, 8, 9, 227, 57, 131, 12, 133, 2, 179, 228, 4, 145, 128, 233, 177,
                233, 90, 212, 124, 41, 248, 76, 33, 180, 160, 140, 176, 168,
            ],
        },
        comment: "arch@arch-pc".to_string(),
    }
}

#[test]
fn ed25519_decode() {
    let private_key = ED25519_ENCODED.parse::<PrivateKey>().unwrap();

    assert_eq!(private_key, ed25519_decoded());
}

#[test]
fn ed25519_encode() {
    let pem_encoded = ed25519_decoded().encode_pem();

    assert_eq!(
        pem_encoded,
        String::new()
            + "-----BEGIN OPENSSH PRIVATE KEY-----\n"
            + ED25519_ENCODED
            + "\n-----END OPENSSH PRIVATE KEY-----\n"
    );
}
