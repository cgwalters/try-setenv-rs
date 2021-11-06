//! This crate provides a single function [`try_set_env_var`] which panics if the process has multiple threads.
//!
//! # Implementation notes:
//!
//! At the current time, detection of multiple threads will only work on Linux kernels.

use std::ffi::OsStr;

/// Returned if the process has multiple threads
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Error;

fn process_is_threaded() -> bool {
    // TODO: cross-platform
    if cfg!(any(target_os = "linux", target_os = "android")) {
        if let Ok(r) = std::fs::read_dir("/proc/self/task") {
            return r.count() > 1;
        }
    }
    // Generic fall through
    false
}

fn try_set_env_var_impl(key: &OsStr, value: Option<&OsStr>) -> Result<(), Error> {
    if process_is_threaded() {
        return Err(Error);
    }
    if let Some(v) = value {
        std::env::set_var(key, v);
    } else {
        std::env::remove_var(key);
    }
    Ok(())
}

/// Wrapper for [`std::env::set_var`] which will panic if the process has multiple threads.
pub fn try_set_env_var<K: AsRef<OsStr>, V: AsRef<OsStr>>(key: K, value: V) -> Result<(), Error> {
    let key = key.as_ref();
    let value = value.as_ref();
    try_set_env_var_impl(key, Some(value))
}

/// Wrapper for [`std::env::remove_var`] which will panic if the process has multiple threads.
pub fn try_remove_env_var<K: AsRef<OsStr>>(key: K) -> Result<(), Error> {
    let key = key.as_ref();
    try_set_env_var_impl(key, None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(any(target_os = "linux", target_os = "android"))]
    fn threading() {
        std::thread::spawn(|| {
            assert!(process_is_threaded());
        })
        .join()
        .unwrap()
    }
}
