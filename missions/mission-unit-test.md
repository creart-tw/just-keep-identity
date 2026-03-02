# Mission: Just Keep Identity (jki) Unit Testing - Phase 3 (Automation & Agency)

## 1. Context Update
Phase 3 has introduced:
- **Git Automation**: `jkim sync` for automated vault synchronization.
- **Agent Service**: `jki-agent` as a background service with an IPC protocol (JSON over Local Sockets).
- **TUI Management**: `jkim edit` for an interactive, searchable account list using `ratatui`.
- **Integrated CLI**: `jki` now supports an `agent` subcommand for IPC interaction.

## 2. Objective
Extend coverage to the new automation logic and agent-based workflows, maintaining a workspace-wide coverage of >80%.

## 3. Key Logic to Test
### jki-core (Sync & IPC)
- **Git Utilities**: 
    - Test `git::add_all`, `git::commit`, `git::pull_rebase`, and `git::push` (mocking the git command if possible, or using temp repos).
    - Ensure `GitRepoStatus` correctly identifies clean vs. modified states.
- **Agent IPC**:
    - Verify `agent::Request` and `agent::Response` serialization/deserialization.

### jki-agent (Service Logic)
- **Request Handling**:
    - Test `handle_client` logic (mocking the `LocalSocketStream` if possible, or using an actual socket in a temporary directory).
    - Ensure `Ping` and `GetOTP` (placeholder) return the expected responses.

### jkim (Automation & TUI)
- **Sync Command**:
    - Verify `handle_sync` performs the correct sequence of Git operations.
    - Test edge cases where Git is not initialized or a remote is missing.
- **TUI Filter Logic**:
    - Extract the filtering logic from `handle_edit` to a testable function to ensure search/filtering works correctly without needing a full terminal environment.

### jki (Subcommand Dispatch)
- **Agent Client**:
    - Test `handle_agent` by mocking the agent response (using a local socket in a temp directory).

## 4. Technical Requirements
- **Async Testing**: Since `jki-agent` and IPC might eventually move to async (Tokio), be prepared to use `#[tokio::test]`.
- **Interprocess Simulation**: Use `JkiPath::agent_socket_path()` overrides to test IPC without affecting the user's running agent.
- **Concurrency**: Continue using `serial_test` for any tests that modify environment variables or global states (like `JKI_HOME`).

## 5. Handover Note
Current coverage is healthy (~80%). Focus on covering the new `git` module in `jki-core` and the IPC dispatcher in `jki`.

---
*Updated by Gemini-CLI for Phase 3 Unit Test Mission.*
