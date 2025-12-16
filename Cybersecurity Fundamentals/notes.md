# Core Principles of Cybersecurity

## Security Goals (CIA Triad and Extensions)
Computer security, cyber security, and information security focus on protecting resources (hardware, software, data, etc.) relative to specified objectives and an adversary’s capabilities. The primary goals are the CIA triad:

- **Confidentiality (C)**: Preventing unauthorized observation or disclosure of information ("keeping secrets secret").
- **Integrity (I)**: Preventing unauthorized modification of information or resources. This includes both prevention mechanisms (stopping unauthorized changes) and detection mechanisms (finding and correcting changes). Integrity is often paramount in commercial systems.
- **Availability (A)**: Ensuring systems and resources are available to authorized users when needed. Ensuring Availability is "devilishly difficult" because attacks (DoS) can be caused by non-security factors, such as infrastructure failure (e.g., a backhoe cutting a cable), and distinguishing malicious activity from legitimate high traffic loads is difficult.
- **Other Essential Goals (AAA)**: Accountability (actions traceable to a single entity), Authenticity (statements/resources are genuine), and Assurance (management of trust).

## Threats and Vulnerabilities
An attack requires four elements: a threat exploits a vulnerability, providing an opportunity for the attacker.

- **Threat Classes**: Disclosure (unauthorized access), Deception (acceptance of false data), Disruption (interruption, DoS), and Usurpation (unauthorized control).
- **Attackers & Motives**: Attackers range from Insiders (who account for >50% of the threat and know system topology) to external agents like Crackers/Hackers, Script-Kiddies, Spies, Criminals, and Hacktivists/Terrorists. Motivations include financial gain, curiosity, fame, and ideology. Spies and Criminals commonly rely on non-technical means like Beatings, Bribery, and Blackmail ("3 Bs").
- **Response Strategies**: Prevention (stopping attacks before they succeed), Deterrence (discouraging attackers via high risk or effort), Deflection (diverting attackers to false targets), Detection (identifying attacks as they happen), and Recovery (restoring systems after an incident).

# Security Administration and Risk Management

## Risk Management Cycle
Risk management involves:

1. **Risk Identification**: Identifying potential risks and their causes. Assets include hardware, software, data, people, documentation, and supplies.
2. **Risk Analysis/Assessment**: Estimating the likelihood and impact (cost/consequence) of threats. Risk is calculated as the cost of an event multiplied by its likelihood. Precise monetary quantification is challenging due to unpredictable attacks and the difficulty of valuing intangible assets (goodwill, data value), making categorical risk levels (High/Medium/Low) practical.
3. **Risk Treatment**: Implementing actions or mechanisms to mitigate the risk (e.g., Risk Reduction, Risk Transfer via insurance, or Risk Acceptance).
4. **Monitoring/Review**: Continuous oversight to control risks and verify compliance.

## Security Planning and Policy
Security Administration is based on Security Planning, Policy Definition/Enforcement, Physical Security, and Risk Analysis.

- **Policy Requirements**: Policies define what is and is not allowed. They require continuous support and funding from top management (CEO). Policies must balance security needs with the inconvenience to users, as mechanisms that are not understood or used are ineffective.
- **Security Plan**: Must define overall goals, establish a baseline (current system state), identify requirements, recommend controls, delegate accountability, and define continuous vigilance/evolution. Systems and their security policies must be capable of evolving to address new vulnerabilities and changes in the environment.

## Physical Security and Availability
Physical controls protect against environmental threats, disasters, and human interference.

- **Availability Measures**: Uninterruptible Power Supplies (UPS) allow clean shutdowns, complemented by backup generators. Surge suppressors protect equipment from electrical variations. Dependability for data centers is measured in Tiers (Tier I: single, non-redundant path; Tier IV: multiple active paths, compartmentalization, continuous cooling, autonomy against single faults).
- **Data Security**: Physical media containing sensitive information (paper, hard disks) must be properly disposed of via shredding, overwriting, or physical destruction (smashing/burning) to prevent interception.

# Authentication and Access Control

## Authentication (Who are you?)
Authentication verifies a claimed identity to grant system access and associate actions with real-world identities (Accountability).

- **Factors**: Authentication relies on: something known (password/PIN), something possessed (token/smart card), something inherent (biometrics), or something performed/done (signature, behavior). Multi-Factor Authentication (MFA) combines these factors.
- **Methods**:
    - **Passwords/Passphrases**: Most common and cheap. Passphrases (longer, multi-word sequences) are preferable as length increases entropy, making them harder to brute force while potentially being easier to remember than complex, short passwords.
    - **Tokens (OTP)**: Can be synchronized (time-based) or challenge-response.
    - **Biometrics**: Measures anatomical (fingerprint, iris, retina, face) or behavioral characteristics (voice, signature). Systems perform Enrollment, Verification (1:1), and Identification (1:N). System quality is measured by error rates (False Positives/Accepts and False Negatives/Rejects).
- **Distributed Identity**: Identity models include Identity Silos (separate credentials per service), Single Sign-On (SSO) (one credential provider for multiple services, e.g., Kerberos), and Federated Identity (circle of trust allowing identity sharing across domains).
- **Session Management**: Once authenticated, a server generates a secure session ID (using secure random numbers) often stored in cookies, which must be fresh and invalidated upon termination. Threats include session hijacking (via XSS or MitM) and Cross-Site Request Forgery (CSRF).

## Access Control and Authorization (Are you allowed?)
Authorization enforces policies governing access to resources. The Reference Monitor acts as a centralized guard mediating all access by subjects (active entities) to objects (passive resources).

- **Access Control Architectures**: Decisions are made by the Policy Decision Point (PDP), enforced by the Policy Enforcement Point (PEP), administered by the Policy Administration Point (PAP), and based on information from the Policy Information Point (PIP).
- **Access Control Models**:
    - **Identity-Based Access Control (IBAC)**: Permissions tied directly to unique user IDs (difficult to scale).
    - **Role-Based Access Control (RBAC)**: Permissions linked to organizational roles (e.g., Professor, Student). RBAC96 expanded on this to include hierarchies and constraints.
    - **Attribute-Based Access Control (ABAC)**: Access relies on attributes of the subject, object, action, and environmental context (like time/location). ABAC models typically use digitally signed assertions (credentials) and are considered favorable for distributed systems due to their non-conflicting (monotonous) nature. JWT tokens often carry ABAC attributes.
    - **Mandatory Access Control (MAC)**: System-enforced policies (often military focused). Bell-LaPadula focuses on Confidentiality (No Read Up, No Write Down), while Biba focuses on Integrity (No Read Down, No Write Up).
- **Implementation Methods**: Access Control Matrix is implemented via Access Control Lists (ACLs) (per object, defines who can access it) or Capabilities (per subject, defines what it can access).

# Cryptography and Protocols

## Cryptographic Building Blocks
Cryptography provides tools to ensure confidentiality, integrity, authenticity, and non-repudiation. Security should rely on the secrecy of the key, not the secrecy of the algorithm (Kerckhoffs's Principle).

- **Symmetric Cryptography (SKC)**: Uses one key for encryption (E(m,k)) and decryption (D(c,k)). It is fast and efficient for large data. SKC algorithms include DES, 3DES, and AES (the current standard).
    - **Modes of Operation**: Block Ciphers operate on fixed-size blocks. Simple modes like Electronic Codebook (ECB) are vulnerable to structural analysis and replay attacks. Modes like Cipher Block Chaining (CBC) and Counter (CTR) introduce feedback/IVs to ensure ciphertext uniqueness.
- **Asymmetric Cryptography (PKC)**: Uses a public/private key pair ($K_{pub}$, $K_{priv}$). It is slow compared to SKC. Examples: RSA (based on factorization difficulty) and ElGamal (based on discrete logarithms difficulty).
- **Cryptographic Hash Functions**: Produce fixed-length output (digest/fingerprint) from variable input. They must be computationally difficult to invert (one-way property) and collision-resistant. Current standards are SHA-2 and SHA-3.
- **Integrity Tools**: Message Authentication Codes (MAC) combine a hash function with a shared key for message integrity and authenticity. Digital Signatures use the private key to sign a message hash, ensuring integrity, authenticity, and non-repudiation.
- **Hybrid Cryptosystems**: Use PKC to securely exchange a fast symmetric session key (K), which is then used by SKC to encrypt the bulk message data.

## Key Management and Protocols
- **Key Properties**: Keys must be generated to be intractable to guess (using true random number generators or correctly seeded PRNGs).
- **Key Distribution**: Keys can be distributed via Key Exchange (one party generates the key) or Key Agreement (all parties influence generation, e.g., Diffie-Hellman).
- **Perfect Forward Secrecy (PFS)**: A crucial property ensuring that compromise of long-term keys does not compromise past session keys.
- **Public Key Infrastructure (PKI)**: A hierarchy of Certification Authorities (CAs) binds public keys to identities using digital certificates. Certificates must be revoked quickly upon key compromise, either automatically (short expiration) or manually (Certificate Revocation Lists/CRLs).
- **Protocol Security**: Cryptographic protocols define how parties employ cryptography. Protocols should be formally analyzed, as informal arguments often overlook vulnerabilities (e.g., the original Needham-Schroeder attack exploited lack of explicit identity). Protocol attacks often involve replay (accepting a message more often than intended) or confusing agents about their roles.

# Network Security and Intrusion Detection

## Communication Security
Networks are defined by boundaries, ownership (Autonomous Systems/Administrative Domains), and control.

- **Link Encryption**: Encryption performed at the Data Link Layer, independently encrypting/decrypting at every intermediate node. Data is exposed (in plaintext) at internal routers.
- **End-to-End Encryption**: Encryption handled by end hosts (application layer), protecting data across all intermediate nodes. Requires user/application management.
- **Virtual Private Networks (VPNs)**: Use Tunneling to encapsulate packets in a secure protocol over a public network, creating a secure private connection.
- **Anonymity in Networks**: Achieved by mixing communication flows, such as in The Onion Router (TOR), where messages are layered with encryption intended for multiple mix-nodes; this hides the source and destination from any single intermediate node.

## Network Defenses and Monitoring
- **Perimeter Security**: Firewalls filter traffic at the network boundary (corporate) or individual nodes (personal) based on packet headers, ports, applications, and sometimes content (Deep Packet Inspection).
- **Intrusion Detection Systems (IDS)**: The process of identifying and responding to malicious activity.
    - **Host-Based IDS**: Inspects local information (logs, running applications, system usage).
    - **Network-Based IDS**: Inspects network traffic using Signature-Based Systems (known attack patterns) or Anomaly Detection Systems (detecting deviations from baseline traffic patterns).
- **Honeypots/Honeynets**: Trap systems designed to attract and retain attackers away from production environments to study their motives and techniques.
- **Network Segmentation**: Dividing the network into smaller, isolated sections with internal firewalls prevents attackers who compromise one host from moving rapidly across the entire organization.

# Social Engineering and Malware

## Social Engineering
Social engineering is the non-technical act of manipulating users to compromise security.

- **Principles of Manipulation**: Authority, Intimidation, Urgency, Scarcity, Familiarity, Consensus, and Trust.
- **Attack Methods (Online/Offline)**:
    - **Phishing/Spear Phishing/Whaling**: Using mass emails, or highly targeted, tailored messages (Whaling targets high-value individuals like CEOs) to extract sensitive information.
    - **Smishing/Vishing**: Phishing via text message (SMS) or voice communication, often exploiting urgency or spoofing legitimate numbers.
    - **Pharming**: Misdirecting users to fake websites (often via DNS server poisoning or malware).
    - **Physical**: Dumpster diving, Shoulder Surfing, and Tailgating (following an authorized person through an access point).
- **Human Role**: Humans are the greatest vulnerability, but trained users are also the best defense against social engineering.

## Malware
Malicious software designed to cause harm or grant unauthorized access.

- **Types**:
    - **Ransomware/Crypto-Malware**: Encrypts files for ransom (e.g., WannaCry, NotPetya) or steals computing resources for cryptocurrency mining.
    - **Trojan Horse**: Appears legitimate but harbors hidden malicious functionality.
    - **Worm**: Self-propagating code that installs copies on penetrated systems without human assistance.
    - **Logic Bomb**: Malicious code inserted by an insider that executes only when a specific condition is met (e.g., time or termination).
    - **Rootkit/Privileged Trojan**: Malware installed to gain privileged access (kernel mode) and hide its presence (rootkit functionality).
    - **Botnet**: A network of compromised "zombie" machines controlled remotely, commonly used for Distributed Denial of Service (DDoS) attacks.
- **Software Vulnerabilities**: Developers must address common software weaknesses (CWE Top 25). Critical flaws include SQL Injection (improper neutralization of input allows database command manipulation) and Buffer Overflow (writing data beyond allocated memory bounds, allowing code execution/reuse). Input validation must occur on the server side (backend), as client-side checks are insufficient.

# Specialized Hardware and Concepts
- **Trusted Platform Module (TPM)**: A secure processor (hardware) supporting cryptographic operations and tamper-resistant storage for keys and measurements. TPM uses Platform Configuration Registers (PCRs) to securely hash and verify software components during a trusted/secure boot process.
- **Zero-Knowledge Proofs (ZKP)**: Cryptographic protocols allowing a prover (Peggy) to convince a verifier (Victor) that she knows a secret or a property without revealing any information beyond the validity of the statement itself. ZKPs are used in anonymous credential systems (e.g., IBM's Idemix) and voting protocols.
- **Digital Fitness**: Users must cultivate "digital fitness," stressing awareness and caution, particularly when dealing with digital links, communication methods, and maintaining good security practices.