# Part I – Select all that apply (36 points)

You get 2 points for every correct selection and -2 points for every incorrect selection. As for any multiple choice quiz, here only the selection counts. There is no need to describe your reasoning. The lowest number of points that you can get for any of the 5 multiple choice problems is 0.

1. Which of the following statements about unconditionally secure and pseudorandom cryptographic primitives are true? Select all that apply.

- A. A real-or-random perfectly secure symmetric key encryption scheme can only be constructed using the One-time Pad.
  - **False** $\rarr$  
    Perfect secrecy is not unique to the One-time Pad (OTP). Any scheme where the ciphertext reveals no information about the message is perfectly secure. 
  
    *Reasoning:* A scheme that encrypts using OTP and then appends a dummy bit (e.g., '0') is still perfectly secure, but it is not the OTP algorithm itself.

- B. Encrypting a message using the One-time Pad symmetric key encryption algorithm using key $k_1$ and encrypting the resulting ciphertext using another instance of the One-time Pad with key $k_2$ is equivalent to applying the One-time Pad once (using another key).
  - **True** $\rarr$ 
    Double OTP encryption is mathematically equivalent to single OTP encryption.
  
    *Reasoning:* $c = (m \oplus k_1) \oplus k_2 = m \oplus (k_1 \oplus k_2)$. Since the XOR sum of two random keys ($k_1 \oplus k_2$) is itself a random string, this acts exactly like a single OTP with a new key.


- C. If one encrypts a message using the One-time Pad symmetric key encryption algorithm and then encrypting the resulting ciphertext using the AES-128 symmetric key encryption scheme with a different key, then the resulting symmetric key encryption scheme is not perfectly secure.
  - **False** $\rarr$  
    Post-processing a perfectly secure ciphertext does not break its security.
  
    *Reasoning:* OTP produces a ciphertext that is statistically independent of the message (random noise). Encrypting this noise with AES cannot recover the original message or leak information. The perfect secrecy established in the first step is preserved.


- D. Let $P$ be a Pseudorandom Permutation of block-size $B$ and key-length $\lambda$, and let $k_1, k_2$ be keys. Let $P(k, x)$ denote applying $P$ to input $x \in \{0, 1\}^B$ with key $k$. Then $P(k_2, P(k_1, x))$ is a PRP with key-length $2\lambda$ and block-size $B$ (the key is $k_1 \| k_2$).
  - **True** $\rarr$ 
    Double encryption constitutes a Pseudorandom Permutation (PRP).
  
    *Reasoning:* A PRP is defined as a permutation indistinguishable from random by a polynomial-time adversary. While "Meet-in-the-Middle" attacks exist, they require exponential time ($O(2^\lambda)$). Since no efficient (polynomial-time) attack exists, the scheme satisfies the definition of a PRP.

- E. Due to the birthday paradox, Pseudorandom permutations of block-size $B$ can be distinguished from Pseudorandom Functions of input and output length $B$ by an attack that obtains around $2^{B/2}$ input/output samples.
  - **False** $\rarr$ 
    The sample size listed is incorrect.
  
    *Reasoning:* The Birthday Paradox states that collisions become likely after the square root of the domain size ($2^B$). The required number of samples is $\sqrt{2^B} = 2^{B/2}$, which is exponentially larger than the $B/2$ samples claimed in the statement.

- F. Pseudorandom Permutations are an abstraction of Public-key Encryption.
  -  **False** $\rarr$
    PRPs model symmetric encryption, not public-key encryption.
    
    *Reasoning:* Pseudorandom Permutations are the theoretical abstraction for Block Ciphers (like AES). Public-key encryption is abstracted by Trapdoor One-Way Permutations.

- G. The AES block cipher first expands the encryption key $k$ into multiple round keys. Then, during encryption, it applies the same function (in almost every round) repeatedly on the input, each time using a different round key. The input of the first round is the message, while every other round takes the previous round’s output as input.
  - **True** $\rarr$
    
     AES is an "iterated block cipher." It expands the main key into multiple round keys and applies the same round function repeatedly, where the output of one round serves as the input for the next.
    
- H. The AES block cipher follows the Feistel design pattern.
  - **False** $\rarr$
    AES does not use the Feistel structure.
    
    *Reasoning:* AES is built on a Substitution-Permutation Network (SPN). The Feistel structure (used in DES) is characterized by splitting the block into halves, which AES does not do.