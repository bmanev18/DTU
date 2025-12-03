# Exercise 1

> Given:
> block size = 64 bits <br>
> Key length $\lambda$ = 56 bits
>
> $3DES.Keygen()$:
> 1. Compute $k_1, k_2, k_3 \larr DES.Keygen()$  
> 2. Output $k = (k_1, k_2, k_3)$
>
> $3DES.Enc(k, m)$:
> 1. Check that $k = (k_1, k_2, k_3)$
> 2. Output $DES.Enc(k_3, DES.Dec(k_2, DES.Enc(k_1, m)))$   

### 1. Find the missing $3DES.Dec$ algorithm. 

The algorithm for $3DES.Dec$ is as follows:
$3DES.Dec(k, c)$:
1. Check that $k = (k_1, k_2, k_3)
2. Output $DES.Dec(k_1, DES.Enc(k_2, DES.Dec(k_3, c)))$

### 2. Brute-force attack

- (a) How many days does it take to recover one key:
  - Since the key length for DES is 56 bits, the total number of possible keys is $2^{56}$. 
  - If an attacker can test one $DES.Enc$ or $DES.Dec$ operation in 1 microsecond ($10^{-6}$ seconds), the time required to brute-force one key is: 
   
  $$
    |K| = 2^{56} 
  $$
  
# Exercise 2
**Goal:** Show that $G(k, x) = F(k, x) \oplus \mathbf{r}$ is a PRF, given that $F$ is a PRF and $\mathbf{r}$ is a fixed public string.

**1. Write down the Lookup functions in the libraries $L^G_{PRF-Real}$ and $L^G_{PRF-Rand}$.**

*   **$L^G_{PRF-Real}$**: This library holds the secret key $k$.
    ```text
    Oracle Lookup(x):
        y_raw = F(k, x)
        y = y_raw XOR r
        return y
    ```

*   **$L^G_{PRF-Rand}$**: This library implements a truly random function. It maintains a table $T$ to ensure consistency (if the same $x$ is queried twice, the same $y$ is returned).
    ```text
    Oracle Lookup(x):
        If x is in T:
            return T[x]
        Else:
            y <- {0, 1}^out  // Sample uniformly at random
            T[x] = y
            return y
    ```

**2. Find a way to rewrite $L^G_{PRF-Real}$ into another library $\overline{L}^G_{PRF-Real}$ which uses $L^F_{PRF-Real}$.**

We can construct a library that acts as a "wrapper" around the oracle provided by the underlying PRF $F$. Let's call the oracle provided by the sub-library $\mathcal{O}_F$.

*   **$\overline{L}^G_{PRF-Real}$**:
    ```text
    Oracle Lookup(x):
        // Call the oracle of the underlying library (which is $L^F_Real$)
        y_raw = O_F.Lookup(x) 
        y = y_raw XOR r
        return y
    ```
    *Justification:* If the underlying oracle $\mathcal{O}_F$ is $L^F_{PRF-Real}$, then `y_raw` is exactly $F(k, x)$. Consequently, the output $y$ is $F(k, x) \oplus \mathbf{r}$, which matches the definition of $G(k, x)$ exactly.

**3. Apply the hybrid technique to switch out $L^F_{PRF-Real}$ with $L^F_{PRF-Rand}$.**

Since $F$ is a secure PRF, the library $L^F_{PRF-Real}$ is computationally indistinguishable from $L^F_{PRF-Rand}$. We can replace the underlying library in our construction from Step 2. Let's call this new hybrid library $L^{Hybrid}$.

*   **$L^{Hybrid}$**: Uses $L^F_{PRF-Rand}$ as the underlying oracle $\mathcal{O}_F$.
    ```text
    Oracle Lookup(x):
        // Call the oracle of the underlying library (which is L^F_Rand)
        y_raw = O_F.Lookup(x)
        y = y_raw XOR r
        return y
    ```
    *Justification:* Because $L^F_{PRF-Real} \approx L^F_{PRF-Rand}$, any adversary distinguishing $\overline{L}^G_{PRF-Real}$ from $L^{Hybrid}$ could be used to distinguish $F$ from a random function. Since $F$ is a PRF, this is impossible (negligible probability).

**4. Find an argument how to complete indistinguishability of the resulting library with $L^G_{PRF-Rand}$.**

We must now compare $L^{Hybrid}$ with $L^G_{PRF-Rand}$.

*   In **$L^{Hybrid}$**: `y_raw` is the output of a truly random function. For any new input $x$, `y_raw` is a uniformly distributed random string from $\{0, 1\}^{out}$. The output is $y = \text{y\_raw} \oplus \mathbf{r}$.
*   In **$L^G_{PRF-Rand}$**: The output $y$ is sampled directly from a uniform distribution over $\{0, 1\}^{out}$.

**Argument:**
The One-Time Pad property (or properties of XOR) states that if $A$ is a uniformly random variable and $B$ is a constant (or a random variable independent of $A$), then $C = A \oplus B$ is a uniformly random variable.

In $L^{Hybrid}$, `y_raw` is uniform. $\mathbf{r}$ is fixed. Therefore, $y = \text{y\_raw} \oplus \mathbf{r}$ is uniformly distributed.
Since the outputs of $L^{Hybrid}$ are uniformly distributed and consistent (because the underlying $L^F_{PRF-Rand}$ is consistent), the behavior of $L^{Hybrid}$ is **identical** to $L^G_{PRF-Rand}$.

**Conclusion:**
$L^G_{PRF-Real} \equiv \overline{L}^G_{PRF-Real} \approx L^{Hybrid} \equiv L^G_{PRF-Rand}$.
Therefore, $G$ is a PRF.

---

### Exercise 3: Birthday bounds in practice
**2. Probability for students in this course**

Assuming a standard classroom size (since the exact number of students isn't provided in the prompt, I will calculate for a few likely class sizes). Let $N = 365$.

*   **If class size $q = 30$:**
    *   Exact Probability: $1 - \prod_{i=0}^{29} \frac{365-i}{365} \approx \mathbf{70.6\%}$
*   **If class size $q = 50$:**
    *   Exact Probability: $\approx \mathbf{97.0\%}$
*   **If class size $q = 100$:**
    *   Exact Probability: $\approx \mathbf{99.99997\%}$

**3. Distinguishing PRF from PRP**

We are distinguishing a PRF $F$ (outputs can collide) from a PRP $G$ (outputs are distinct permutations, cannot collide).
*   **Distinguisher Strategy:** Query the oracle $q$ times. If a collision occurs ($y_i = y_j$ for $x_i \neq x_j$), output "PRF". If no collision, output "PRP".
*   **Probability of Success:** This is roughly equal to the probability of finding a collision in the PRF (since the PRP collision probability is 0).
*   **Parameters:** Block size is 32 bits, so $N = 2^{32}$.
*   **Formula:** We use the bound $P \approx \frac{q^2}{2N}$.

**Calculations:**

1.  **$q = 2$**:
    $$P \approx \frac{2^2}{2 \cdot 2^{32}} = \frac{4}{2^{33}} = 2^{-31} \approx 4.66 \times 10^{-10}$$
    *Probability is negligible.*

2.  **$q = 2^8$ (256)**:
    $$P \approx \frac{(2^8)^2}{2 \cdot 2^{32}} = \frac{2^{16}}{2^{33}} = 2^{-17} \approx 0.0000076$$
    *Probability is very low.*

3.  **$q = 2^{12}$ (4096)**:
    $$P \approx \frac{(2^{12})^2}{2 \cdot 2^{32}} = \frac{2^{24}}{2^{33}} = 2^{-9} \approx 0.00195$$
    *Probability is small ($\approx 0.2\%$).*

4.  **$q = 2^{16}$ (65536)**:
    $$P \approx \frac{(2^{16})^2}{2 \cdot 2^{32}} = \frac{2^{32}}{2^{33}} = 2^{-1} = 0.5$$
    *Probability is **50\%**.*

**Conclusion:**
For $q = 2^{16}$ (which is $\sqrt{N}$), the probability of distinguishing $F$ from $G$ becomes significant (50%). This confirms the "Birthday Bound" rule of thumb: collisions become likely once the number of queries reaches the square root of the output space size.
    