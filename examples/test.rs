// This is really test code for the library, but split out
// as a separate file to ensure we are started without threads;
// the cargo test runtime may spawn threads.

#[cfg(any(target_os = "linux", target_os = "android"))]
fn test_fork() -> std::io::Result<()> {
    use nix::sys::wait::WaitStatus;

    unsafe {
        let p = nix::unistd::fork()?;
        match p {
            nix::unistd::ForkResult::Parent { child } => {
                match nix::sys::wait::waitpid(child, None)? {
                    WaitStatus::Exited(p, status) => {
                        assert_eq!(p, child);
                        assert_eq!(status, 0);
                    }
                    o => {
                        panic!("child process failed: {:?}", o)
                    }
                }
                Ok(())
            }
            nix::unistd::ForkResult::Child => {
                try_setenv::try_set_env_var("foo", "bar").unwrap();
                std::process::exit(0);
            }
        }
    }
}

fn main() {
    try_setenv::try_set_env_var("foo", "bar").unwrap();
    assert_eq!(std::env::var_os("foo").unwrap(), "bar");
    std::thread::spawn(|| {
        let r = std::panic::catch_unwind(|| {
            // Silence the panic info
            let h = std::panic::take_hook();
            std::panic::set_hook(Box::new(|_| {}));
            try_setenv::try_set_env_var("somekey", "someval").unwrap();
            std::panic::set_hook(h)
        });
        if cfg!(any(target_os = "linux", target_os = "android")) {
            assert!(r.is_err());
        }
    })
    .join()
    .unwrap();

    if cfg!(any(target_os = "linux", target_os = "android")) {
        test_fork().unwrap();
        println!("Validated fork()");
    }

    println!("Validated try_set_env_var with and without threads.")
}
