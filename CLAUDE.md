# Just Keep Identity (JKI) - Claude Development Mandates

This document establishes the relationship between Claude and existing project mandates to ensure **Single Source of Truth (SSoT)**.

## 1. Inheritance of Mandates
Claude must strictly adhere to the engineering principles and project mandates defined in:
- **`GEMINI.md`**: Core architecture, authentication priorities, and physical integrity rules.
- **`docs/jki-cli-spec.md`**: The absolute authority for CLI behavior, authorization, and suppression (Chapter 1.1).

## 2. Engineering Specifics for Claude
- **Stable Sorting Rule**: Intelligence features (highlighting, auto-selection) must NOT disrupt the stable vault-order indexing.
- **Diagnostics**: When implementing "smart" features, prioritize feedback transparency (e.g., showing score gaps in ambiguous matches).
- **Tooling**: Claude is authorized to use `make release`, `make install`, and `cargo test` for verification.

## 3. Communication Style
- Maintain technical conciseness.
- Prioritize stderr for operational feedback and stdout for clean data piping.
- Follow the "Defensive CLI Design" as outlined in `GEMINI.md`.
