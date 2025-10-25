use aes::Aes128;
use aes::cipher::{BlockEncrypt, KeyInit, generic_array::GenericArray};
use thiserror::Error;

const KEYLENGTH: usize = 16;
const BLOCK_SIZE: usize = 16;
// Maximum number of blocks that can be encrypted with a 4-byte counter (2^32)
const MAX_BLOCKS: usize = u32::MAX as usize;

type Key = [u8; KEYLENGTH];
type Nonce = [u8; BLOCK_SIZE];

#[derive(Error, Debug)]
pub enum CtrError {
    #[error("Ciphertext too short: expected at least {expected} bytes, got {actual}")]
    CiphertextTooShort { expected: usize, actual: usize },
    #[error("Invalid nonce size")]
    InvalidNonceSize,
    #[error("Not divisible by block size")]
    NotDivisibleByBlockSize,
    #[error("Block count exceeds maximum for 4-byte counter")]
    BlockCountExceeded,
}

/// Key Generation, which should output a random key of length 128 bits
pub fn keygen() -> Key {
    let mut key = Key::default();
    rand::fill(&mut key);
    key
}

/// Generate a random nonce for CTR mode
pub fn generate_nonce() -> Nonce {
    let mut nonce = Nonce::default();
    rand::fill(&mut nonce);
    nonce
}

/// Generate keystream using AES in CTR mode
fn generate_keystream(key: &Key, nonce: &Nonce, length: usize) -> Result<Vec<u8>, CtrError> {
    debug_assert!(
        length % BLOCK_SIZE == 0,
        "Length must be a multiple of BLOCK_SIZE"
    );

    let cipher = Aes128::new(GenericArray::from_slice(key));
    let mut keystream = Vec::with_capacity(length);

    // Calculate length in blocks
    let blocks_count = length / BLOCK_SIZE;

    if blocks_count > MAX_BLOCKS {
        return Err(CtrError::BlockCountExceeded);
    }

    // Initial counter block structure: [N(12 bytes) | C(4 bytes)]
    let mut counter_block = *nonce;

    counter_block[12..16].fill(0);

    for i in 0..blocks_count {
        // For blocks within 4-byte counter limit, use simple increment
        if i < MAX_BLOCKS {
            // Convert i to big-endian bytes and place in last 4 bytes
            let counter_bytes = (i as u32).to_be_bytes();
            counter_block[12..16].copy_from_slice(&counter_bytes);
        }

        let mut block = GenericArray::clone_from_slice(&counter_block);
        cipher.encrypt_block(&mut block);
        keystream.extend_from_slice(block.as_slice());
    }

    // Truncate to exact length needed
    keystream.truncate(length);
    Ok(keystream)
}

/// Encryption, which should encrypt a message of ℓ blocks of length 128
/// bits each using a key k using AES-128 in CTR mode.
/// Returns: nonce || ciphertext
pub fn encrypt(plaintext: &[u8], key: &Key) -> Result<Vec<u8>, CtrError> {
    if plaintext.len() % BLOCK_SIZE != 0 {
        return Err(CtrError::NotDivisibleByBlockSize);
    }
    let nonce = generate_nonce();

    // Generate keystream
    let keystream = generate_keystream(key, &nonce, plaintext.len())?;

    // XOR plaintext with keystream
    let ciphertext: Vec<u8> = plaintext
        .iter()
        .zip(keystream.iter())
        .map(|(p, k)| p ^ k)
        .collect();

    // Prepend nonce to ciphertext
    let mut result = nonce.to_vec();
    result.extend_from_slice(&ciphertext);
    Ok(result)
}

/// Decryption, which should decrypt a ciphertext of ℓ + 1 blocks of length
/// 128 each using a key k and AES-128 in CTR mode.
/// Input format: nonce || ciphertext
pub fn decrypt(ciphertext: &[u8], key: &Key) -> Result<Vec<u8>, CtrError> {
    if ciphertext.len() % BLOCK_SIZE != 0 {
        return Err(CtrError::NotDivisibleByBlockSize);
    }
    if ciphertext.len() < BLOCK_SIZE {
        return Err(CtrError::CiphertextTooShort {
            expected: BLOCK_SIZE,
            actual: ciphertext.len(),
        });
    }

    // Extract nonce and actual ciphertext
    let (nonce_bytes, encrypted_data) = ciphertext.split_at(BLOCK_SIZE);
    let nonce: &[u8; BLOCK_SIZE] = nonce_bytes
        .try_into()
        .map_err(|_| CtrError::InvalidNonceSize)?;

    // Generate keystream (same as encryption)
    let keystream = generate_keystream(key, nonce, encrypted_data.len())?;

    // XOR ciphertext with keystream to get plaintext
    let plaintext: Vec<u8> = encrypted_data
        .iter()
        .zip(keystream.iter())
        .map(|(c, k)| c ^ k)
        .collect();

    Ok(plaintext)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_keygen() {
        let key1 = keygen();
        let key2 = keygen();
        assert_ne!(key1, key2, "Two generated keys should not be equal");
    }
    #[test]
    fn test_encrypt_decrypt() {
        let key = keygen();
        let plaintext = b"Hello, world!!!!"; // 16 bytes exactly (1 block)
        let ciphertext = encrypt(plaintext, &key).unwrap();
        let decrypted = decrypt(&ciphertext, &key).expect("Decryption should succeed");
        assert_eq!(
            plaintext.to_vec(),
            decrypted,
            "Decrypted text should match the original plaintext"
        );
    }

    #[test]
    fn test_encrypt_decrypt_large() {
        let key = keygen();
        let plaintext =
            b"This is a longer message that spans multiple AES blocks for a better CTR testing"
                .to_vec(); //  80 bytes = 5 blocks
        let ciphertext = encrypt(&plaintext, &key).unwrap();
        let decrypted = decrypt(&ciphertext, &key).expect("Decryption should succeed");
        assert_eq!(
            plaintext, decrypted,
            "Decrypted text should match the original plaintext"
        );
    }

    #[test]
    fn test_ciphertext_too_short() {
        let key = keygen();
        let short_ciphertext = vec![0u8; 8]; // Less than BLOCK_SIZE
        let result = decrypt(&short_ciphertext, &key);
        assert!(result.is_err(), "Should return error for short ciphertext");
    }

    #[test]
    fn test_different_keys_produce_different_ciphertexts() {
        let key1 = keygen();
        let key2 = keygen();
        let plaintext = b"Same message!!!!"; // 16 bytes exactly (1 block)

        let ciphertext1 = encrypt(plaintext, &key1).unwrap();
        let ciphertext2 = encrypt(plaintext, &key2).unwrap();

        // Skip the nonce part and compare only the encrypted data
        assert_ne!(
            &ciphertext1[16..],
            &ciphertext2[16..],
            "Different keys should produce different ciphertexts"
        );
    }

    ///  2. CTR-mode is not IND-CCA secure, i.e. it is possible to modify cipher-
    ///  texts so that the plaintexts get modified in a predictable way. Write a
    ///  test that performs this attack, and shows that it works on your AES-
    ///  CTR implementation.
    #[test]
    fn test_ctr_malleability_attack() {
        //  CTR mode encrypts plaintext by XORing it with a keystream: C = P ⊕ Keystream.
        //  During decryption: P = C ⊕ Keystream. Because XOR is its own inverse,
        //  if an attacker modifies ciphertext bits, those exact bit changes will appear
        //  in the decrypted plaintext.

        let key = keygen();
        let original_plaintext = b"Transfer $100.00";
        let malicious_plaintext = b"Transfer $999.00";

        let original_ciphertext = encrypt(original_plaintext, &key).unwrap();

        // We calculate the XOR difference between original and target plaintexts
        let mut xor_diff = [0u8; 16];
        for i in 0..16 {
            xor_diff[i] = original_plaintext[i] ^ malicious_plaintext[i];
        }

        // We create malicious ciphertext by XORing the difference with the original ciphertext
        // We skip the first 16 bytes (nonce) and only modify the actual ciphertext
        let mut malicious_ciphertext = original_ciphertext.clone();
        for i in 0..16 {
            malicious_ciphertext[16 + i] ^= xor_diff[i];
        }

        let decrypted_malicious = decrypt(&malicious_ciphertext, &key).unwrap();

        // To verify that the attack worked - the decrypted text should match our target
        assert_eq!(
            decrypted_malicious,
            malicious_plaintext.to_vec(),
            "Malleability attack failed: decrypted text doesn't match target"
        );

        // Verify that it's different from the original
        assert_ne!(
            decrypted_malicious,
            original_plaintext.to_vec(),
            "Attack didn't modify the plaintext"
        );

        println!("CTR Malleability Attack Successful!");
        println!(
            "Original:  {:?}",
            String::from_utf8_lossy(original_plaintext)
        );
        println!(
            "Modified:  {:?}",
            String::from_utf8_lossy(&decrypted_malicious)
        );
    }
}
