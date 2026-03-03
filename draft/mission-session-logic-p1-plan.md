# Objective
Implement Agent Session Logic Integration (Phase 1) to enable "One-time Authentication, Global Availability" and "Automatic Data Refresh" across the JKI toolsuite.

# Key Files & Context
- `crates/jki-core/Cargo.toml`: Add `interprocess` dependency.
- `crates/jki-core/src/lib.rs`: 
    - Update `agent::Request` and `agent::Response` enums.
    - Implement `AgentClient` for IPC communication.
    - Move `ensure_agent_running` from `jki` to `jki_core`.
    - Update `acquire_master_key` to prioritize Agent-stored keys.
- `crates/jki-agent/src/main.rs`: 
    - Reinforce `State` with persistent `master_key`.
    - Handle `GetMasterKey` and `Reload` requests.
    - Implement `Reload` logic to clear cached secrets.
- `crates/jkim/src/main.rs`: 
    - Trigger agent `Reload` after data modifications (`edit`, `import`, `change`, `sync`).
    - Utilize updated `acquire_master_key` for session-aware authentication.
- `crates/jki/src/main.rs`: 
    - Implement "Lazy Unlock": Synchronize authenticated master key to Agent after successful local authentication.

# Implementation Steps

## 1. Core Expansion (jki-core)
1. Add `interprocess` to `jki-core/Cargo.toml`.
2. Update `agent` module in `jki-core/src/lib.rs`:
    - `Request`: Add `GetMasterKey`, `Reload`.
    - `Response`: Add `MasterKey(String)`, `Success`.
3. Implement `AgentClient` struct in `jki-core/src/lib.rs` with methods:
    - `connect()`, `ping()`, `unlock(key)`, `get_otp(id)`, `get_master_key()`, `reload()`.
4. Move `ensure_agent_running` to `jki-core/src/lib.rs` and make it public.
5. Update `acquire_master_key` in `jki-core/src/lib.rs`:
    - Before checking Keychain/File, try `AgentClient::get_master_key()`.

## 2. Agent Session Management (jki-agent)
1. Update `State` in `jki-agent/src/main.rs`:
    - Add `master_key: Option<secrecy::SecretString>`.
2. Modify `State::unlock`:
    - Store the `master_key` upon successful unlock.
3. Modify `State::get_otp`:
    - If `secrets` is `None` but `master_key` is `Some`, perform a transparent reload (re-decrypt files).
4. Update `handle_client_io`:
    - `Request::GetMasterKey`: Return `Response::MasterKey` if unlocked.
    - `Request::Reload`: Set `self.secrets = None` (triggering re-read on next access) and return `Response::Success`.

## 3. Toolsuite Integration (jkim & jki)
1. In `jkim/src/main.rs`:
    - Update `handle_edit`, `handle_import_winauth`, `handle_sync`, and `handle_master_key::Change` to call `AgentClient::reload()` at the end of successful operations.
2. In `jki/src/main.rs`:
    - Use `jki_core::ensure_agent_running`.
    - In `run()`, after falling back to local decryption and obtaining `master_key`, send an `Unlock` request to the agent if it's running (Lazy Unlock).

# Verification & Testing
- Start `jki-agent`.
- Run `jki -o <pattern>` to trigger local auth and verify Agent becomes unlocked (Lazy Unlock).
- Run `jkim edit` or `jkim import-winauth` and verify Agent reloads (clears cache).
- Run `cargo test` across all crates to ensure no regressions.
