# Just Keep Identity (jki)
> **Extreme speed MFA & Identity Session Manager for CLI Power Users.**

![JKI Demo](docs/assets/demo.gif)

[繁體中文](README.zh-TW.md)

`jki` is an identity authorization tool designed specifically for engineers. It's not just about managing TOTP; it's about completing authentication at "millisecond" speeds without ever leaving your terminal.

## 🚀 Core Philosophy

*   **Extreme Velocity**: Search and copy in < 3ms. By the time you need the OTP, it's already in your clipboard.
*   **Fuzzy Intelligence**: Advanced fuzzy search with character highlighting. Locate accounts instantly even if you don't remember the exact name.
*   **Smart Agent**: Intelligent background agent supporting auto-unlock for plaintext vaults and active disk synchronization (Active Reload).
*   **Physical Isolation**: Built on `age` encryption. All secrets stay on your local disk or your private Git repo—zero cloud dependency.
*   **CLI Ergonomics**: Optimized Micro-Roll command set (`j-k-i`), allowing for one-handed operation.

## 🧬 Technical DNA

Built with Rust for extreme stability and security:

*   **Intelligent Agent**: `jki-agent` manages decrypted memory cache. It's the secure gateway to OS Keyring integration.
*   **Dual-Mode Vault**:
    *   `Plaintext Mode`: Maximum speed, reads local plaintext cache in secure environments.
    *   `Encrypted Mode`: AES-GCM encryption via `age`, perfect for Git synchronization and long-term storage.
*   **Unix-Friendly**: Perfect pipe support (`stdout -`), easily integrates with `ssh`, `git`, `kubectl`, and other CLI tools.

## 🛠 Quick Start

```bash
# Query and copy OTP (Priority: Agent -> Keyfile -> Password Prompt)
jki github

# Smart Filtering: Search for "google" and select the 2nd result
jki google 2

# Force List Mode: View matches without executing
jki google -l

# Fast Vault Sync (Git commit/pull/push)
jkim git sync
```

### Smart Filtering & Selection

`jki` follows a "Filter -> Action" logic chain, making it effortless to navigate complex account lists:

1.  **Multi-Pattern Filtering**: `jki [PATTERNS]... [INDEX]`
    *   `jki u`: Lists all accounts matching `u` (e.g., Uber, Uplay).
    *   `jki u 2`: Directly acquires the OTP for the 2nd item in the results.
2.  **List Mode (`-l, --list`)**:
    *   Appended `-l` switches `jki` to "View Only" mode.
    *   Extremely useful for verifying index numbers in large result sets.
3.  **Graceful Feedback**: Ambiguous results are no longer errors; JKI elegantly lists candidates with score gaps to guide your next keystroke.

---

## 📦 Installation (macOS)

```bash
# Clone and Install
git clone https://github.com/creart-tw/just-keep-identity.git
cd just-keep-identity
make install
```

---

*Built with ❤️ for those who live in the terminal.*
