use alloc::vec::Vec;
use chacha20poly1305::{
    aead::{Aead, KeyInit},
    XChaCha20Poly1305, XNonce,
};
use rand::{rngs::OsRng, RngCore};

use crate::{PrivacyError, PrivacyResult};

pub type EnvelopeKey = [u8; 32];
pub type CipherText = Vec<u8>;

#[derive(Debug, Clone)]
pub struct EncryptedBlob {
    pub nonce:      [u8; 24],
    pub ciphertext: Vec<u8>,
}

pub fn generate_random_key() -> EnvelopeKey {
    let mut key = [0u8; 32];
    OsRng.fill_bytes(&mut key);
    key
}

pub fn generate_nonce() -> [u8; 24] {
    let mut nonce = [0u8; 24];
    OsRng.fill_bytes(&mut nonce);
    nonce
}

pub fn encrypt_encrypted_bytes(
    plaintext: &[u8],
    key: &[u8],
    nonce: &[u8],
) -> PrivacyResult<EncryptedBlob> {
    if key.len() != 32 {
        return Err(PrivacyError::InvalidKeyLength { expected: 32, found: key.len() });
    }
    if nonce.len() != 24 {
        return Err(PrivacyError::InvalidNonceLength { expected: 24, found: nonce.len() });
    }

    let cipher = XChaCha20Poly1305::new_from_slice(key)
        .map_err(|_| PrivacyError::EncryptionFailed)?;
    let ct = cipher
        .encrypt(XNonce::from_slice(nonce), plaintext)
        .map_err(|_| PrivacyError::EncryptionFailed)?;

    let mut nonce_buf = [0u8; 24];
    nonce_buf.copy_from_slice(nonce);
    Ok(EncryptedBlob { nonce: nonce_buf, ciphertext: ct })
}

pub fn decrypt_encrypted_bytes(
    ciphertext: &[u8],
    key: &[u8],
    nonce: &[u8],
) -> PrivacyResult<Vec<u8>> {
    if key.len() != 32 {
        return Err(PrivacyError::InvalidKeyLength { expected: 32, found: key.len() });
    }
    if nonce.len() != 24 {
        return Err(PrivacyError::InvalidNonceLength { expected: 24, found: nonce.len() });
    }

    let cipher = XChaCha20Poly1305::new_from_slice(key)
        .map_err(|_| PrivacyError::DecryptionFailed)?;
    cipher
        .decrypt(XNonce::from_slice(nonce), ciphertext)
        .map_err(|_| PrivacyError::DecryptionFailed)
}
