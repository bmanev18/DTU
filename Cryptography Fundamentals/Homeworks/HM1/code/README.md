# AES-128 CTR Mode Implementation

This project implements AES-128 in Counter (CTR) mode of operation as part of Exercise 1.3 for Cryptography Fundamentals. The implementation provides key generation, encryption, and decryption functions, along with a demonstration of CTR mode's malleability vulnerability.

## Compilation and Installation

### Installation Steps

1. **Install Rust** (if not already installed):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   ```

2. **Navigate to the project directory**:
   ```bash
   cd /path/to/code
   ```

3. **Install dependencies and compile**:
   ```bash
   cargo build
   ```

   This command will automatically download and compile all required dependencies as specified in `Cargo.toml`.

### Compilation Commands

- **Build the project**: `cargo build`
- **Build in release mode**: `cargo build --release`
- **Check compilation without building**: `cargo check`

## Running the Tests

### Test Execution Commands

1. **Run all tests**:
   ```bash
   cargo test
   ```

2. **Run all tests with debug output**:
    ```bash
    cargo test -- --show-output
    ```

3. **Run a specific test**:
   ```bash
   cargo test [TEST_NAME] 
   ```

### Available Tests

The implementation includes the following comprehensive tests:

#### 1. `test_keygen()`
**Purpose**: Verifies that key generation produces different keys on subsequent calls.

**Expected Output**: Test passes silently (no output on success).

**Interpretation**: Confirms the randomness of key generation.


#### 2. `test_encrypt_decrypt()`
**Purpose**: Tests basic encryption/decryption functionality with a single block message.

**Expected Output**: Test passes silently.

**Interpretation**: Verifies that encryption and decryption are inverse operations.


#### 3. `test_encrypt_decrypt_large()`
**Purpose**: Tests encryption/decryption with multi-block messages (5 blocks, 80 bytes).

**Expected Output**: Test passes silently.

**Interpretation**: Confirms proper handling of multi-block messages and counter increment logic.


#### 4. `test_ciphertext_too_short()`
**Purpose**: Verifies proper error handling when ciphertext is shorter than minimum required length.

**Expected Output**: Test passes silently.

**Interpretation**: Confirms input validation is working correctly.


#### 5. `test_different_keys_produce_different_ciphertexts()`
**Purpose**: Ensures that different keys produce different ciphertexts for the same plaintext.

**Expected Output**: Test passes silently.

**Interpretation**: Verifies that the encryption is key-dependent.


#### 6. `test_ctr_malleability_attack()` ⚠️
**Purpose**: Demonstrates CTR mode's vulnerability to malleability attacks.

**Expected Output**: 

```
CTR Malleability Attack Successful!
Original:  "Transfer $100.00"
Modified:  "Transfer $999.00"
```
**Interpretation**: This test **should pass** and demonstrates that an attacker can predictably modify ciphertexts to change specific parts of the plaintext. This is expected behavior showing CTR mode's lack of IND-CCA security.


### Example Complete Test Run

Expected output from `cargo test -- --show-output`:

```rust
   Compiling code v0.1.0 (/path/to/code)
    Finished test [unoptimized + debuginfo] target(s) in X.XXs
     Running unittests src/lib.rs (target/debug/deps/code-XXXXXXXXX)

running 6 tests
test tests::test_ciphertext_too_short ... ok
test tests::test_different_keys_produce_different_ciphertexts ... ok
test tests::test_encrypt_decrypt ... ok
test tests::test_encrypt_decrypt_large ... ok
test tests::test_ctr_malleability_attack ... ok
test tests::test_keygen ... ok

---- tests::test_ctr_malleability_attack stdout ----
CTR Malleability Attack Successful!
Original:  "Transfer $100.00"
Modified:  "Transfer $999.00"

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```
