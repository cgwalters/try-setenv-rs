# try-setenv-rs

Trivial crate motivated by discussion in 
https://internals.rust-lang.org/t/synchronized-ffi-access-to-posix-environment-variable-functions/

This exposes a single `try_set_env_var` which ultimately wraps `setenv()` but
verifies that there are not multiple threads.
