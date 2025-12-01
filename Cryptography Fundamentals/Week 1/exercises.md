# Exercise 1.1

## a) 

Let $V = \{1, 2, 3, 4, 5, 6\}$ be the set of outcomes for a single die throw.

Let $X_1, X_2, X_3, X_4 \leftarrow V$ be four independent throws of a fair die. This means they are sampled from the uniform distribution over $V$.

We want to compute the probability that $X_i = 6$ for at least one $i \in \{1, 2, 3, 4\}$.

Let $A$ be the event that at least one throw is a 6.

$$A = \exists i \in \{1, 2, 3, 4\} : X_i = 6$$

It is easier to compute the probability of the complement event, $\bar{A}$, which is that *none* of the throws are a 6.

$$\bar{A} = \forall i \in \{1, 2, 3, 4\} : X_i \neq 6$$

For a single throw $X_i$, the probability of not getting a 6 is:

$$P(X_i \neq 6) = 1 - P(X_i = 6) = 1 - \frac{1}{6} = \frac{5}{6}$$

Since the four throws are independent events, the probability that none of them are 6 is the product of their individual probabilities:

$$P(\bar{A}) = P(X_1 \neq 6 \text{ and } X_2 \neq 6 \text{ and } X_3 \neq 6 \text{ and } X_4 \neq 6)$$

$$P(\bar{A}) = \prod_{i=1}^{4} P(X_i \neq 6) = \left(\frac{5}{6}\right)^4 = \frac{625}{1296}$$

The probability of event $A$ is then:

$$P(A) = 1 - P(\bar{A}) = 1 - \frac{625}{1296} = \frac{1296 - 625}{1296} = \frac{671}{1296}$$

So, the probability of getting at least one 6 in four throws of a fair die is $\frac{671}{1296}$.

## b)

Let the 10 coin tosses be a sequence $C = (C_1, C_2, \dots, C_{10})$, where each $C_i \in \{0, 1\}$ is chosen uniformly at random.

We have two betting options:

**Option 1: Guess the first 6 coin tosses.**

There are $2^6 = 64$ possible sequences for the first 6 tosses. We make one guess. The probability of this guess being correct is:

$$P(\text{Win Option 1}) = \frac{1}{2^6} = \frac{1}{64}$$

**Option 2: Guess a sequence of 7 coin tosses that will appear as a sub-sequence.**

Let our guess be a sequence $G = (g_1, \dots, g_7)$. We win if this sequence appears as a contiguous sub-sequence in the 10 tosses. There are 4 possible starting positions for a 7-toss sub-sequence in a 10-toss sequence:

- $S_1 = (C_1, \dots, C_7)$
- $S_2 = (C_2, \dots, C_8)$
- $S_3 = (C_3, \dots, C_9)$
- $S_4 = (C_4, \dots, C_{10})$

We win if our guess $G$ is equal to at least one of these sub-sequences $S_j$. Let $A_j$ be the event that $G = S_j$. The probability of any single $A_j$ is $P(A_j) = (\frac{1}{2})^7 = \frac{1}{128}$.

The events $A_j$ are not disjoint. For example, if $S_1 = S_2$, matching one means matching the other. To find the total winning probability, we can use the Principle of Inclusion-Exclusion. However, a simpler approach is to sum the probabilities of winning at each possible starting position for the first time.

Let's choose a non-periodic guess sequence, for example, $G = (0, 0, 0, 0, 0, 0, 1)$. This sequence does not overlap with shifted versions of itself.

- The probability of winning at the first position is $P(A_1) = 1/128$.
- The probability of losing at the first position but winning at the second is $P(\bar{A_1} \cap A_2) = P(\bar{A_1}) \times P(A_2) = (1 - 1/128) \times (1/128)$. This is because the events are independent due to the non-overlapping nature of the coin tosses involved.
- Similarly, $P(\bar{A_1} \cap \bar{A_2} \cap A_3) = (1 - 1/128)^2 \times (1/128)$.
- And $P(\bar{A_1} \cap \bar{A_2} \cap \bar{A_3} \cap A_4) = (1 - 1/128)^3 \times (1/128)$.

The total probability of winning is the sum of these probabilities:

$$P(\text{Win Option 2}) = \frac{1}{128} + \left(1-\frac{1}{128}\right)\frac{1}{128} + \left(1-\frac{1}{128}\right)^2\frac{1}{128} + \left(1-\frac{1}{128}\right)^3\frac{1}{128}$$

This is a geometric series sum. Let $p = 1/128$.

$$P(\text{Win Option 2}) = p \sum_{k=0}^{3} (1-p)^k = p \frac{1 - (1-p)^4}{1 - (1-p)} = 1 - (1-p)^4 = 1 - \left(\frac{127}{128}\right)^4$$

$$P(\text{Win Option 2}) = 1 - \frac{127^4}{128^4} = 1 - \frac{259694081}{268435456} = \frac{8741375}{268435456} \approx 0.0309$$

This strategy of choosing a non-periodic sequence is optimal because it minimizes the overlaps between the winning opportunities, thus maximizing the total probability of a win.

**Comparison and Decision:**

- $P(\text{Win Option 1}) = 1/64 = 0.015625$
- $P(\text{Win Option 2}) \approx 0.0309$

The probability of winning with Option 2 is almost double that of Option 1. Therefore, the best strategy is to choose Option 2 and guess a non-periodic sequence of length 7.

**Would you take the bet?**

To decide, we can calculate the expected value (EV) of the bet for the chosen strategy (Option 2).

- Win: +50 DKK
- Lose: -1 DKK
- $P(\text{Win}) \approx 0.0309$
- $P(\text{Lose}) = 1 - P(\text{Win}) \approx 0.9691$

$$EV = (50 \times P(\text{Win})) + (-1 \times P(\text{Lose}))$$

$$EV \approx (50 \times 0.0309) - (1 \times 0.9691) = 1.545 - 0.9691 = 0.5759 \text{ DKK}$$

Since the expected value is positive, on average, you would make money by taking this bet repeatedly. Therefore, yes, I would take the bet.

---

# Exercise 1.2

To determine if a pair of random variables (X,Y) is independent, you must check if their joint probability distribution P(X=x,Y=y) is equal to the product of their marginal probability distributions P(X=x)⋅P(Y=y) for all possible values of x and y.

The marginal probabilities are the sums of the rows and columns of the joint probability matrix.

### For $p_1$:

$p_1 = \begin{pmatrix} 1/2 & 0 \\ 0 & 1/2 \end{pmatrix}$

Marginal Probabilities:

- $P(X_1=0) = \frac{1}{2} + 0 = \frac{1}{2}$
- $P(X_1=1) = 0 + \frac{1}{2} = \frac{1}{2}$
- $P(Y_1=0) = \frac{1}{2} + 0 = \frac{1}{2}$
- $P(Y_1=1) = 0 + \frac{1}{2} = \frac{1}{2}$

Independence Check:

Let's check the entry $p_1(0,0)$.

- $p_1(0,0) = \frac{1}{2}$
- $P(X_1=0) \cdot P(Y_1=0) = \frac{1}{2} \cdot \frac{1}{2} = \frac{1}{4}$

Since $\frac{1}{2} \neq \frac{1}{4}$, the pair $(X_1, Y_1)$ is **dependent**.

### For $p_2$:

$p_2 = \begin{pmatrix} 1/3 & 1/3 \\ 0 & 1/3 \end{pmatrix}$

Marginal Probabilities:

- $P(X_2=0) = \frac{1}{3} + \frac{1}{3} = \frac{2}{3}$
- $P(X_2=1) = 0 + \frac{1}{3} = \frac{1}{3}$
- $P(Y_2=0) = \frac{1}{3} + 0 = \frac{1}{3}$
- $P(Y_2=1) = \frac{1}{3} + \frac{1}{3} = \frac{2}{3}$

Independence Check:

Let's check the entry $p_2(0,0)$.

- $p_2(0,0) = \frac{1}{3}$
- $P(X_2=0) \cdot P(Y_2=0) = \frac{2}{3} \cdot \frac{1}{3} = \frac{2}{9}$

Since $\frac{1}{3} \neq \frac{2}{9}$, the pair $(X_2, Y_2)$ is **dependent**.

### For $p_3$:

$p_3 = \begin{pmatrix} 1/4 & 1/4 \\ 1/4 & 1/4 \end{pmatrix}$

Marginal Probabilities:

- $P(X_3=0) = \frac{1}{4} + \frac{1}{4} = \frac{1}{2}$
- $P(X_3=1) = \frac{1}{4} + \frac{1}{4} = \frac{1}{2}$
- $P(Y_3=0) = \frac{1}{4} + \frac{1}{4} = \frac{1}{2}$
- $P(Y_3=1) = \frac{1}{4} + \frac{1}{4} = \frac{1}{2}$

Independence Check:

Now, we must check all four entries.

- $p_3(0,0) = \frac{1}{4}$. Product: $P(X_3=0) \cdot P(Y_3=0) = \frac{1}{2} \cdot \frac{1}{2} = \frac{1}{4}$. (Match)
- $p_3(0,1) = \frac{1}{4}$. Product: $P(X_3=0) \cdot P(Y_3=1) = \frac{1}{2} \cdot \frac{1}{2} = \frac{1}{4}$. (Match)
- $p_3(1,0) = \frac{1}{4}$. Product: $P(X_3=1) \cdot P(Y_3=0) = \frac{1}{2} \cdot \frac{1}{2} = \frac{1}{4}$. (Match)
- $p_3(1,1) = \frac{1}{4}$. Product: $P(X_3=1) \cdot P(Y_3=1) = \frac{1}{2} \cdot \frac{1}{2} = \frac{1}{4}$. (Match)

Since all four entries satisfy the condition, the pair $(X_3, Y_3)$ is **independent**.

### Exercise 1.3

Cyphertext: ";\r6TXfTe~r[bjrTeXrlbhrWb\aZ2rHf\aZrUeb^XarVelcgb~r[h[2r;TccXafrgbrg[XrUXfgrbYrhf!!!rAXkgrg\‘XrTebhaW~rgelr48F $%+r\ar:T_b\fr6bhagXer@bWXsss"

*   **Plaintext ($m$):**
    `Hi Caesar, how are you doing? Using broken crypto, huh? Happens to the best of us... Next time around, try AES-128 in Galois Counter Mode!!!`
*   **Parameters:**
    *   $a = 32$
    *   $b = 126$
    *   $k = 82$

**Justification:**

1.  **Analyzing the Alphabet Range ($a, b$):**
    By inspecting the ciphertext, we observe that the characters fall within the range of printable ASCII characters.
    *   The minimum ASCII value observed is 32 (the space character `' '`, which appears in the segment `F $%`).
    *   The maximum ASCII value observed is 126 (the tilde `~`).
    *   Therefore, we can hypothesize that the alphabet $\mathcal{X}_{a,b}$ corresponds to the standard printable ASCII range, where **$a = 32$** and **$b = 126$**.
    *   The size of this alphabet is $\ell = b - a + 1 = 126 - 32 + 1 = 95$.

2.  **Finding the Key ($k$):**
    We can use frequency analysis to find the key. In standard English sentences, the **space** character is typically the most frequent.
    *   In the ciphertext, the character **`r`** (ASCII 114) appears most frequently (16 times).
    *   We hypothesize that `r` in the ciphertext corresponds to `space` (ASCII 32) in the plaintext.
    *   Using the encryption formula $e_k(x) = (x - a + k) \pmod \ell + a$, the decryption formula is $d_k(c) = (c - a - k) \pmod \ell + a$.
    *   Setting $c = 114$ (`r`) and $d_k(c) = 32$ (`space`):
        $$32 = (114 - 32 - k) \pmod{95} + 32$$
        $$0 = (82 - k) \pmod{95}$$
    *   This implies **$k = 82$**.

3.  **Decryption Verification:**
    Using the parameters $a=32, b=126, k=82$, we can decrypt the first few characters of the ciphertext `;\r6TXfTe~r`:
    *   `;` (59) $\rightarrow$ $(59 - 32 - 82) \pmod{95} + 32 = (-55 \pmod{95}) + 32 = 40 + 32 = 72$ ('H')
    *   `\` (92) $\rightarrow$ $(92 - 32 - 82) \pmod{95} + 32 = (-22 \pmod{95}) + 32 = 73 + 32 = 105$ ('i')
    *   `r` (114) $\rightarrow$ $(114 - 32 - 82) \pmod{95} + 32 = 0 + 32 = 32$ (' ')
    *   `6` (54) $\rightarrow$ $(54 - 32 - 82) \pmod{95} + 32 = (-60 \pmod{95}) + 32 = 35 + 32 = 67$ ('C')
    *   `T` (84) $\rightarrow$ $(84 - 32 - 82) \pmod{95} + 32 = (-30 \pmod{95}) + 32 = 65 + 32 = 97$ ('a')
    
    This yields "Hi Ca...", confirming the hypothesis. Decrypting the full string results in the English text provided above.



