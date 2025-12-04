# Part I – Select all that apply (36 points)

## 1. Which of the following statements about unconditionally secure and pseudorandom cryptographic primitives are true?

- **A. A real-or-random perfectly secure symmetric key encryption scheme can only be constructed using the One-time Pad.**
  - **False** $\rarr$  
    Perfect secrecy is not unique to the One-time Pad (OTP). Any scheme where the ciphertext reveals no information about the message is perfectly secure. 
  
    *Reasoning:* A scheme that encrypts using OTP and then appends a dummy bit (e.g., '0') is still perfectly secure, but it is not the OTP algorithm itself.

- **B. Encrypting a message using the One-time Pad symmetric key encryption algorithm using key $k_1$ and encrypting the resulting ciphertext using another instance of the One-time Pad with key $k_2$ is equivalent to applying the One-time Pad once (using another key).**
  - **True** $\rarr$ 
    Double OTP encryption is mathematically equivalent to single OTP encryption.
  
    *Reasoning:* $c = (m \oplus k_1) \oplus k_2 = m \oplus (k_1 \oplus k_2)$. Since the XOR sum of two random keys ($k_1 \oplus k_2$) is itself a random string, this acts exactly like a single OTP with a new key.


- **C. If one encrypts a message using the One-time Pad symmetric key encryption algorithm and then encrypting the resulting ciphertext using the AES-128 symmetric key encryption scheme with a different key, then the resulting symmetric key encryption scheme is not perfectly secure.**
  - **False** $\rarr$  
    Post-processing a perfectly secure ciphertext does not break its security.
  
    *Reasoning:* OTP produces a ciphertext that is statistically independent of the message (random noise). Encrypting this noise with AES cannot recover the original message or leak information. The perfect secrecy established in the first step is preserved.


- **D. Let $P$ be a Pseudorandom Permutation of block-size $B$ and key-length $\lambda$, and let $k_1, k_2$ be keys. Let $P(k, x)$ denote applying $P$ to input $x \in \{0, 1\}^B$ with key $k$. Then $P(k_2, P(k_1, x))$ is a PRP with key-length $2\lambda$ and block-size $B$ (the key is $k_1 \| k_2$).**
  - **True** $\rarr$ 
    Double encryption constitutes a Pseudorandom Permutation (PRP).
  
    *Reasoning:* A PRP is defined as a permutation indistinguishable from random by a polynomial-time adversary. While "Meet-in-the-Middle" attacks exist, they require exponential time ($O(2^\lambda)$). Since no efficient (polynomial-time) attack exists, the scheme satisfies the definition of a PRP.

- **E. Due to the birthday paradox, Pseudorandom permutations of block-size $B$ can be distinguished from Pseudorandom Functions of input and output length $B$ by an attack that obtains around $2^{B/2}$ input/output samples.**
  - **False** $\rarr$ 
    The sample size listed is incorrect.
  
    *Reasoning:* The Birthday Paradox states that collisions become likely after the square root of the domain size ($2^B$). The required number of samples is $\sqrt{2^B} = 2^{B/2}$, which is exponentially larger than the $B/2$ samples claimed in the statement.

- **F. Pseudorandom Permutations are an abstraction of Public-key Encryption.**
  -  **False** $\rarr$
    PRPs model symmetric encryption, not public-key encryption.
    
    *Reasoning:* Pseudorandom Permutations are the theoretical abstraction for Block Ciphers (like AES). Public-key encryption is abstracted by Trapdoor One-Way Permutations.

- **G. The AES block cipher first expands the encryption key $k$ into multiple round keys. Then, during encryption, it applies the same function (in almost every round) repeatedly on the input, each time using a different round key. The input of the first round is the message, while every other round takes the previous round’s output as input.**
  - **True** $\rarr$
    
     AES is an "iterated block cipher." It expands the main key into multiple round keys and applies the same round function repeatedly, where the output of one round serves as the input for the next.
    
- **H. The AES block cipher follows the Feistel design pattern.**
  - **False** $\rarr$
    AES does not use the Feistel structure.
    
    *Reasoning:* AES is built on a Substitution-Permutation Network (SPN). The Feistel structure (used in DES) is characterized by splitting the block into halves, which AES does not do.

## 2. Which of the following statements about Modes of Operation, Hash functions and MACs are true?


- **A. The IND-CPA security of CTR mode fails if the initially chosen random value $r$ is reused.**
  - **True** $\rarr$
    CTR (Counter) mode turns a block cipher into a stream cipher. It generates a "keystream" by encrypting a series of counters: $E_k(r || 1), E_k(r || 2), \dots$. The ciphertext is created by XORing the message with this keystream: $C = M \oplus \text{Keystream}$.
    
    *The Flaw:* If you reuse the random value $r$ (the nonce) with the same key $k$, you generate the **exact same keystream**.
    
    *The Attack:* If an attacker sees two ciphertexts encrypted with the same nonce:
    *   $C_1 = M_1 \oplus \text{Keystream}$
    *   $C_2 = M_2 \oplus \text{Keystream}$
    *   The attacker can compute $C_1 \oplus C_2 = (M_1 \oplus \text{Keystream}) \oplus (M_2 \oplus \text{Keystream}) = M_1 \oplus M_2$.
    *   This reveals the XOR difference of the two plaintexts, completely breaking confidentiality (IND-CPA). This is known as the "Two-Time Pad" problem.

- **B. In the ECB mode of operation, each block of the input message of length $B$ is encrypted by applying an arbitrary secure Pseudorandom Function on the block. All PRF calls use the same key, and the construction always works as long as the PRF has an output length of at least $B$ bits.** 
  - **False** $\rarr$
    The key issue here is **Decryption**.
    
    *PRF vs. PRP:* A Pseudorandom Function (PRF) maps an input to a random-looking output, but it is not necessarily reversible (invertible). A Pseudorandom Permutation (PRP), also known as a Block Cipher, is a bijection—it maps every input to a unique output and **can be reversed**.
    
    *Why it fails:* To decrypt a message in ECB mode, you must be able to take the ciphertext block and reverse the operation to get the plaintext block ($M = D_k(C)$). If you use a generic PRF that is not a permutation, you cannot guarantee that decryption is possible (two different messages might map to the same output, or the function might simply be one-way). Therefore, ECB requires a PRP (Block Cipher), not just a PRF.

- **C. Any Pseudorandom function of output length $\lambda$ and key-length $\lambda$ can be used to implement a MAC scheme that is EUF-CMA secure against attackers with runtime polynomial in $\lambda$.**
  - **True** $\rarr$
    This refers to the canonical definition of a MAC.
    
    *The Construction:* A standard way to build a MAC for fixed-length messages is to simply define the tag as $t = F_k(m)$, where $F$ is a secure PRF.
    
    *Why it works:* The definition of a secure PRF is that its output is indistinguishable from a truly random function to any efficient adversary. If the output is random, an attacker cannot predict the tag $t$ for a new message $m$ without knowing the key $k$. This satisfies the requirement for EUF-CMA (Existential Unforgeability under Chosen Message Attack).

- **D. The length of a CBC MAC tag scales with the length of the message that is protected.**
  - **False** $\rarr$
    CBC-MAC uses the Chaining Block Cipher mode, but it does not output the whole chain.
    
    *The Process:* It processes the message block by block, chaining the result of the previous block into the next. However, the **Tag** is defined only as the **output of the final block**.
    
    *Result:* Regardless of whether the message is 1 block long or 1000 blocks long, the resulting tag is exactly the size of one block (e.g., 128 bits for AES). It is a fixed-length authenticator.

- **E. Secure MAC schemes protect both the confidentiality and integrity of a message.**
  - **False** $\rarr$
    This is a fundamental distinction in cryptography primitives.
    *   **Encryption** (e.g., AES-CBC, CTR) protects **Confidentiality** (secrecy).
    *   **MACs** (e.g., HMAC, CBC-MAC) protect **Integrity** and **Authenticity** (detecting tampering).
    
    *The Reality:* A MAC tag is usually sent alongside the plaintext message: $(m, \text{Tag})$. An attacker can read $m$ perfectly fine (no confidentiality), but they cannot modify $m$ without invalidating the tag. To get both protections, you must use Authenticated Encryption (AE).

- **F. A cryptographic hash function is a family of deterministic functions that maps an input of variable length to a fixed output length.**
  - **True** $\rarr$
    This is the standard definition.
    1.  **Deterministic:** If you hash the same file twice, you get the exact same hash every time.
    2.  **Variable Input:** It can accept a single letter or a 4GB video file.
    3.  **Fixed Output:** The result is always a bit string of a specific size (e.g., 256 bits for SHA-256), regardless of input size.

- **G. A cryptographic hash function can be used to make RSA IND-CCA secure using the OAEP transform.**
  - **True** $\rarr$
    "Textbook" RSA is not secure (it is deterministic and malleable).
    
    *OAEP (Optimal Asymmetric Encryption Padding):* This is a padding scheme applied to the message before RSA encryption. It uses cryptographic hash functions (specifically as Mask Generation Functions, or MGFs) to introduce randomness and destroy the mathematical structure of the message.
    
    *Result:* This transform makes RSA secure against Chosen Ciphertext Attacks (IND-CCA).

- **H. The Merkle-Damgård construction for hash functions is prone to length-extension attacks.**
  - **False** $\rarr$
    (Based on the provided solution context).
    
    *Explanation of the Teacher's Logic:* The Merkle-Damgård construction relies on **Merkle Padding**. This padding scheme appends the **length of the message** to the end of the input before hashing.
    
    *Why "No":* The teacher is arguing that because the padding includes the message length, the construction is formally secure against trivial extension. The padding ensures that the input is "prefix-free" (a valid message cannot be a prefix of another valid message).
    
    *Note:* In practical implementations (like MD5, SHA-1, SHA-256), the state is output directly, which *does* allow length extension attacks. However, in the theoretical context of the course, the "padding scheme" (Merkle padding) is the theoretical fix that prevents the mathematical triviality of extending the chain.


## 3. Which of the following statements are true about Public key encryption and digital signatures?

- **A. 4 is often used as an RSA public exponent, due to its small size**
  - **False** $\rarr$
    *Requirement for $e$:* In RSA, the public exponent $e$ must be coprime to $\phi(N)$ (the Euler totient function) so that a modular inverse $d$ (the private key) exists.
    
    *The Math:* $N = p \cdot q$ where $p$ and $q$ are large primes. Since primes $>2$ are odd, $p-1$ and $q-1$ are both even numbers.
    
    *The Conflict:* $\phi(N) = (p-1)(q-1)$. Since both factors are even, $\phi(N)$ is even (divisible by 2). The number 4 is also even. Therefore, $\gcd(4, \phi(N)) \ge 2$. They are not coprime, so $d$ cannot be computed, and the encryption would not be reversible.

- **B. If one can efficiently compute the Euler totient function $\phi(N)$ of an arbitrary number $N$, then RSA is insecure.**
  - **True** $\rarr$
    *The Relationship:* The RSA private key $d$ is computed using the extended Euclidean algorithm to satisfy $d \equiv e^{-1} \pmod{\phi(N)}$.
    
    *The Break:* The public key consists of $(N, e)$. If an attacker can compute $\phi(N)$, they have all the inputs required to calculate $d$ exactly as the key owner did. This allows them to decrypt any message.

- **C. The ElGamal public key cryptosystem can only be constructed if discrete logarithms in $\Z^*_p$ are hard to compute**
  - **False** $\rarr$
    *Construction vs. Security:* You can *construct* (run the algorithm for) ElGamal on any cyclic group, regardless of whether the math is hard or easy. The algorithm functions, it just might not be secure.
    
    *Hardness Assumptions:* The security of ElGamal relies on the **Decisional Diffie-Hellman (DDH)** assumption, not just the Discrete Log (DL) assumption.
    *   It is theoretically possible for a group to exist where Discrete Log is hard (you can't find $x$), but DDH is easy (you can distinguish $g^{xy}$ from random). In such a group, ElGamal would be insecure even though DL is hard.

- **D. The ElGamal public key cryptosystem is IND-CPA secure.**
  - **True** $\rarr$
    *Assumption:* This assumes the underlying group satisfies the Decisional Diffie-Hellman (DDH) assumption.
    
    *Reasoning:* In ElGamal, the ciphertext includes a "mask" $g^{xy} \cdot m$. Under the DDH assumption, the shared secret $g^{xy}$ is indistinguishable from a truly random group element to an attacker. Masking a message with a truly random value (similar to a One-Time Pad) hides all information about the plaintext, satisfying IND-CPA (Indistinguishability under Chosen Plaintext Attack).

- **E. In a digital signature scheme, both the sender and the receiver share a secret key.**
  - **False** $\rarr$
    *Symmetric vs. Asymmetric:* Shared secret keys are a property of Symmetric Cryptography (like MACs).
    
    *Digital Signatures:* These are Asymmetric (Public-Key) primitives.
    *   **Sender:** Uses a **Private Key** (known only to them) to generate the signature.
    *   **Receiver:** Uses a **Public Key** (known to everyone) to verify the signature. No secret is shared between the parties.

- **F. If there exists an efficient attacker against the Computational Diffie-Hellman problem in a certain group $G$, then there exists an efficient attacker against Decisional Diffie-Hellman in $G$.**
  - **True** $\rarr$
    *The Hierarchy:*
    *   **CDH (Computational Diffie-Hellman):** Given $g, g^a, g^b$, compute $g^{ab}$.
    *   **DDH (Decisional Diffie-Hellman):** Given $g, g^a, g^b, Z$, determine if $Z = g^{ab}$ or if $Z$ is random.
    
    *The Reduction:* If you have an algorithm that solves CDH, you can easily solve DDH. You simply take the inputs $g^a, g^b$, run your CDH algorithm to compute the actual $g^{ab}$, and compare it to the candidate $Z$. If they match, $Z$ is valid; otherwise, it is random. Thus, if CDH is broken, DDH is automatically broken.

- **G. The X3DH (Signal) key exchange protocol is secure against attacks with quantum computers.**
  - **False** $\rarr$
    *Underlying Primitive:* X3DH relies on the hardness of the Discrete Logarithm problem (specifically Elliptic Curve Diffie-Hellman).
    
    *Shor's Algorithm:* A sufficiently powerful quantum computer running Shor's algorithm can solve the Discrete Logarithm problem in polynomial time. This breaks the Diffie-Hellman exchange, allowing a quantum attacker to recover the session keys.

<!--  -->

## 4. Which of the following statements are true about Post-Quantum Cryptography and LWE?

- **A. It is known how to break block ciphers with the help of a quantum algorithm for the period-finding problem.**
  - **False** $\rarr$
    *Explanation:* The "period-finding problem" is the specific mathematical task solved by **Shor's Algorithm**. This algorithm is effective against number-theoretic problems like Factoring (RSA) and Discrete Logarithms (Diffie-Hellman).
    
    *Block Ciphers (AES):* These do not rely on hidden periods or cyclic group structures. They rely on "confusion and diffusion."
    
    *The Real Threat:* Block ciphers are threatened by **Grover's Algorithm**, which is a quantum search algorithm.

- **B. It is known how to break RSA with the help of a quantum algorithm for the period-finding problem.**
  - **True** $\rarr$
    *Explanation:* The security of RSA relies on the difficulty of factoring a large number $N$.
    
    *The Reduction:* Factoring $N$ can be mathematically reduced to finding the period $r$ of the function $f(x) = a^x \mod N$.
    
    *Shor's Algorithm:* This is a quantum algorithm that can find this period $r$ in polynomial time. Once the period is found, extracting the factors of $N$ is computationally easy, thus breaking RSA.

- **C. Quantum computing attacks break the Diffie-Hellman key exchange protocol both in its original and elliptic curve variants.**
  - **True** $\rarr$
    *Explanation:* Both original Diffie-Hellman (finite fields) and Elliptic Curve Diffie-Hellman (ECDH) rely on the **Discrete Logarithm Problem** in cyclic groups.
    
    *Shor's Reach:* Shor's algorithm solves the "Hidden Subgroup Problem" for finite abelian groups. Since both the multiplicative group of integers and the group of points on an elliptic curve fall into this category, Shor's algorithm breaks both in polynomial time.

- **D. There are known polynomial-time quantum algorithms for breaking all practical cryptography.**
  - **False** $\rarr$
    *Explanation:* "All practical cryptography" includes Symmetric Encryption (AES) and Hash Functions (SHA-2/SHA-3).
    
    *Symmetric Crypto:* As mentioned in (A), symmetric primitives are only weakened by Grover's algorithm (requiring larger keys), not broken completely.
    
    *Post-Quantum Crypto:* There are also asymmetric schemes (like those based on Lattices, Isogenies, or Multivariate equations) for which no polynomial-time quantum attack is known.

- **E. There is a known quantum algorithm to solve the Learning With Errors problem (underlying Regev encryption) in polynomial time.**
  - **False** $\rarr$
    *Explanation:* This is the entire premise of **Lattice-based Cryptography**.
    
    *Hardness:* The Learning With Errors (LWE) problem involves solving linear equations that have been "polluted" with small amounts of noise.
    
    *Quantum Resistance:* Currently, the best known quantum algorithms for solving LWE (mostly based on sieving) still run in exponential time. Because Shor's algorithm (period finding) does not apply to the noisy lattice structure, LWE is considered "Quantum Safe."

- **F. In Regev’s encryption scheme, the message is used as the secret vector s in the Learning With Errors (LWE) problem (the decision LWE problem is the task of distinguishing $A$, $As + e$ from a uniformly random pair of data with the same sizes, where e is small).**
  - **False** $\rarr$
    *Explanation:* This statement confuses the roles of the variables in the scheme.
    *   **The Secret Key ($sk$):** This is the vector $\mathbf{s}$. It is static and used for decryption.
    *   **The Message ($m$):** This is the data we want to transmit.
  
<!--  -->

## 5.  Which of the following statements are true in basic cryptography?

- **A. Kerckhoffs’ principle states that the confidentiality of a cryptographic key should be at least one out of several secret data...**
  - **False** $\rarr$
    *Explanation:* Kerckhoffs' principle explicitly argues against relying on multiple secret components (like a secret algorithm).
    
    *The Principle:* A cryptosystem should remain secure even if everything about the system (the algorithm, the hardware, the source code) is public knowledge, **except for the key**.

- **B. A cryptographic algorithm that is evaluated in an attack model that gives the attacker more power enjoys stronger security guarantees.**
  - **True** $\rarr$
    *Explanation:* Security definitions are hierarchical.
    
    *The Logic:* Proving security in a stronger model (giving the attacker more capabilities) implies security in all weaker models.

- **C. Security definitions are necessary for mathematical proofs of cryptographic security.**
  - **True** $\rarr$
    *Explanation:* You cannot mathematically prove that a system is "secure" without a rigorous definition of what "secure" means.
    
    *The Framework:* A proof requires:
    1.  **Syntax:** How the algorithms work.
    2.  **Security Definition:** A "Game" involving an adversary and a challenger, defining exactly what the adversary must achieve to "win" (e.g., distinguish two messages).

- **D. If $f(n) > |p(n)|$ for some polynomial $p$ and some integer $n > 0$, $f$ is not negligible.**
  - **False** $\rarr$
    *Explanation:* The definition of a **negligible function** is asymptotic (it concerns the behavior as $n \to \infty$). The statement claims that if the function is large at **one specific point** (some integer $n$), it is not negligible. This is incorrect. A function can be huge for small $n$ (e.g., $n=1$ to $100$) but still drop off rapidly to zero as $n$ increases, satisfying the definition of negligible.
    
    *The Definition:* A function is negligible if it eventually becomes smaller than the inverse of *any* polynomial for all $n$ larger than some threshold $N$.

- **E. IND-CPA security ensures that a scheme is secure even against attackers who can request decryptions...**
  - **False** $\rarr$
    *Explanation:* The statement describes **IND-CCA** (Chosen Ciphertext Attack), not IND-CPA.

- **F. A (properly generated) one-time pad ciphertext is a uniformly random string from the attacker’s perspective.**
  - **True** $\rarr$
    *Explanation:* This is the property of **Perfect Secrecy**.
    
    *The Math:* The ciphertext is calculated as $C = M \oplus K$.
    
    *Reasoning:* If the key $K$ is chosen uniformly at random from $\{0,1\}^n$, then for any fixed message $M$, the resulting ciphertext $C$ is also uniformly distributed over $\{0,1\}^n$. The attacker sees only random noise, which is why the ciphertext reveals absolutely no information about the message.

- **G. It is fundamentally impossible to determine whether an attack against AES succeeds with non-negligible probability...**
  - **True** $\rarr$
    "Negligible probability" is a theoretical limit that describes what happens as a security parameter $n$ approaches infinity $n \to \infty$.
    
    However, AES uses fixed key sizes (e.g., 128 bits). Since the size is constant, you cannot calculate a limit. Therefore, AES relies on Concrete Security (calculating exact odds, like $2^{-128}$) rather than theoretical asymptotic definitions.