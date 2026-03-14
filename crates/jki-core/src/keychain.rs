#[cfg(feature = "keychain")]
use keyring::{Entry, Error as KeyringError};
use secrecy::SecretString;

/// Trait representing a secure storage for secrets.
pub trait SecretStore {
    fn set_secret(&self, service: &str, user: &str, secret: &str) -> Result<(), String>;
    fn get_secret(&self, service: &str, user: &str) -> Result<SecretString, String>;
    fn delete_secret(&self, service: &str, user: &str) -> Result<(), String>;
}

/// Internal trait for low-level keyring operations to allow mocking.
trait RawKeyring {
    fn set(&self, service: &str, user: &str, secret: &str) -> Result<(), String>;
    fn get(&self, service: &str, user: &str) -> Result<String, String>;
    fn delete(&self, service: &str, user: &str) -> Result<(), String>;
}

/// Real OS implementation of RawKeyring.
#[cfg(feature = "keychain")]
struct OsKeyring;

#[cfg(all(feature = "keychain", target_os = "macos"))]
trait CommandRunner {
    fn run(&self, program: &str, args: &[&str]) -> Result<std::process::Output, std::io::Error>;
}

#[cfg(all(feature = "keychain", target_os = "macos"))]
struct RealRunner;

#[cfg(all(feature = "keychain", target_os = "macos"))]
impl CommandRunner for RealRunner {
    fn run(&self, program: &str, args: &[&str]) -> Result<std::process::Output, std::io::Error> {
        std::process::Command::new(program).args(args).output()
    }
}

#[cfg(feature = "keychain")]
impl RawKeyring for OsKeyring {
    fn set(&self, service: &str, user: &str, secret: &str) -> Result<(), String> {
        #[cfg(target_os = "macos")]
        return self.set_internal(service, user, secret, &RealRunner);
        #[cfg(not(target_os = "macos"))]
        return self.set_internal(service, user, secret, &());
    }

    fn get(&self, service: &str, user: &str) -> Result<String, String> {
        let entry = Entry::new(service, user).map_err(|e| e.to_string())?;
        entry.get_password().map_err(|e| match e {
            KeyringError::NoEntry => "Secret not found".to_string(),
            _ => e.to_string(),
        })
    }

    fn delete(&self, service: &str, user: &str) -> Result<(), String> {
        let entry = Entry::new(service, user).map_err(|e| e.to_string())?;
        entry.delete_credential().map_err(|e| e.to_string())
    }
}

#[cfg(feature = "keychain")]
impl OsKeyring {
    #[cfg(target_os = "macos")]
    fn set_internal(
        &self,
        service: &str,
        user: &str,
        secret: &str,
        runner: &dyn CommandRunner,
    ) -> Result<(), String> {
        let current_exe = std::env::current_exe().map_err(|e| e.to_string())?;
        let mut agent_exe = current_exe.clone();
        agent_exe.pop();
        agent_exe.push("jki-agent");

        let _ = runner.run(
            "security",
            &["delete-generic-password", "-a", user, "-s", service],
        );

        let output = runner
            .run(
                "security",
                &[
                    "add-generic-password",
                    "-a",
                    user,
                    "-s",
                    service,
                    "-w",
                    secret,
                    "-T",
                    &current_exe.to_string_lossy().to_string(),
                    "-T",
                    &agent_exe.to_string_lossy().to_string(),
                ],
            )
            .map_err(|e| e.to_string())?;

        if !output.status.success() {
            let err = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Security command failed: {}", err));
        }
        Ok(())
    }

    #[cfg(not(target_os = "macos"))]
    fn set_internal(
        &self,
        service: &str,
        user: &str,
        secret: &str,
        _runner: &(),
    ) -> Result<(), String> {
        let entry = Entry::new(service, user).map_err(|e| e.to_string())?;
        entry.set_password(secret).map_err(|e| e.to_string())
    }
}

/// Implementation of `SecretStore` using the system's native keychain.
pub struct KeyringStore;

#[cfg(feature = "keychain")]
impl SecretStore for KeyringStore {
    fn set_secret(&self, service: &str, user: &str, secret: &str) -> Result<(), String> {
        OsKeyring.set(service, user, secret)
    }

    fn get_secret(&self, service: &str, user: &str) -> Result<SecretString, String> {
        OsKeyring.get(service, user).map(SecretString::from)
    }

    fn delete_secret(&self, service: &str, user: &str) -> Result<(), String> {
        OsKeyring.delete(service, user)
    }
}

#[cfg(not(feature = "keychain"))]
impl SecretStore for KeyringStore {
    fn set_secret(&self, _service: &str, _user: &str, _secret: &str) -> Result<(), String> {
        Err("Keychain support not compiled in".to_string())
    }

    fn get_secret(&self, _service: &str, _user: &str) -> Result<SecretString, String> {
        Err("Keychain support not compiled in".to_string())
    }

    fn delete_secret(&self, _service: &str, _user: &str) -> Result<(), String> {
        Err("Keychain support not compiled in".to_string())
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use secrecy::ExposeSecret;
    use std::collections::HashMap;
    use std::sync::Mutex;

    /// A simple mock store for unit testing components that depend on `SecretStore`.
    pub struct MockSecretStore {
        storage: Mutex<HashMap<String, String>>,
    }

    impl MockSecretStore {
        pub fn new() -> Self {
            Self {
                storage: Mutex::new(HashMap::new()),
            }
        }

        fn key(service: &str, user: &str) -> String {
            format!("{}:{}", service, user)
        }
    }

    impl SecretStore for MockSecretStore {
        fn set_secret(&self, service: &str, user: &str, secret: &str) -> Result<(), String> {
            self.storage
                .lock()
                .unwrap()
                .insert(Self::key(service, user), secret.to_string());
            Ok(())
        }

        fn get_secret(&self, service: &str, user: &str) -> Result<SecretString, String> {
            self.storage
                .lock()
                .unwrap()
                .get(&Self::key(service, user))
                .cloned()
                .map(SecretString::from)
                .ok_or_else(|| "Secret not found".to_string())
        }

        fn delete_secret(&self, service: &str, user: &str) -> Result<(), String> {
            self.storage
                .lock()
                .unwrap()
                .remove(&Self::key(service, user))
                .map(|_| ())
                .ok_or_else(|| "Secret not found".to_string())
        }
    }

    #[test]
    fn test_mock_secret_store() {
        let store = MockSecretStore::new();
        let service = "test-service";
        let user = "test-user";
        let secret = "test-secret";

        store.set_secret(service, user, secret).unwrap();
        let retrieved = store.get_secret(service, user).unwrap();
        assert_eq!(retrieved.expose_secret(), secret);

        store.delete_secret(service, user).unwrap();
        let result = store.get_secret(service, user);
        assert!(result.is_err());
    }

    #[test]
    #[cfg(not(feature = "keychain"))]
    fn test_keyring_store_fallback() {
        let store = KeyringStore;
        assert!(store.set_secret("s", "u", "p").is_err());
        assert!(store.get_secret("s", "u").is_err());
        assert!(store.delete_secret("s", "u").is_err());
    }

    // --- New Tests for RawKeyring logic ---

    struct MemoryBackend {
        storage: Mutex<HashMap<String, String>>,
    }

    impl RawKeyring for MemoryBackend {
        fn set(&self, service: &str, user: &str, secret: &str) -> Result<(), String> {
            self.storage
                .lock()
                .unwrap()
                .insert(format!("{}:{}", service, user), secret.to_string());
            Ok(())
        }
        fn get(&self, service: &str, user: &str) -> Result<String, String> {
            self.storage
                .lock()
                .unwrap()
                .get(&format!("{}:{}", service, user))
                .cloned()
                .ok_or_else(|| "Not found".to_string())
        }
        fn delete(&self, service: &str, user: &str) -> Result<(), String> {
            self.storage
                .lock()
                .unwrap()
                .remove(&format!("{}:{}", service, user))
                .map(|_| ())
                .ok_or_else(|| "Not found".to_string())
        }
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn test_macos_keychain_acl_logic() {
        use std::sync::Arc;

        struct MockRunner {
            calls: Arc<Mutex<Vec<String>>>,
            should_fail: bool,
        }
        impl CommandRunner for MockRunner {
            fn run(
                &self,
                program: &str,
                args: &[&str],
            ) -> Result<std::process::Output, std::io::Error> {
                let mut calls = self.calls.lock().unwrap();
                calls.push(format!("{} {}", program, args.join(" ")));

                // Use a proper ExitStatus creation for testing
                use std::os::unix::process::ExitStatusExt;
                use std::process::ExitStatus;

                Ok(std::process::Output {
                    status: if self.should_fail {
                        ExitStatus::from_raw(256) // Error code
                    } else {
                        ExitStatus::from_raw(0)
                    },
                    stdout: vec![],
                    stderr: if self.should_fail {
                        b"Access denied".to_vec()
                    } else {
                        vec![]
                    },
                })
            }
        }

        let calls = Arc::new(Mutex::new(vec![]));
        let runner = MockRunner {
            calls: calls.clone(),
            should_fail: false,
        };
        let kr = OsKeyring;

        // Test successful path
        kr.set_internal("mysvc", "myusr", "mypwd", &runner).unwrap();

        let history = calls.lock().unwrap();
        assert_eq!(history.len(), 2);
        assert!(history[0].contains("delete-generic-password"));
        assert!(history[1].contains("add-generic-password"));
        assert!(history[1].contains("-T")); // Verify ACL flags are present
        assert!(history[1].contains("jki-agent")); // Verify agent path logic

        // Test failure path
        let calls_err = Arc::new(Mutex::new(vec![]));
        let runner_err = MockRunner {
            calls: calls_err.clone(),
            should_fail: true,
        };
        let result = kr.set_internal("mysvc", "myusr", "mypwd", &runner_err);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Security command failed"));
    }

    #[test]
    fn test_os_keyring_cross_platform_stub() {
        let kr = OsKeyring;
        let _ = kr.get("non-existent-svc", "user");
        let _ = kr.delete("non-existent-svc", "user");
    }

    #[test]
    fn test_raw_keyring_interface() {
        let backend = MemoryBackend {
            storage: Mutex::new(HashMap::new()),
        };
        backend.set("svc", "usr", "pwd").unwrap();
        assert_eq!(backend.get("svc", "usr").unwrap(), "pwd");
        backend.delete("svc", "usr").unwrap();
        assert!(backend.get("svc", "usr").is_err());
    }
}
