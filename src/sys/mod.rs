#[cfg(windows)] mod windows;
#[cfg(windows)] pub use windows::*;

#[cfg(unix)] mod unix;
#[cfg(unix)] pub use unix::*;

#[cfg(not(any(windows, unix)))] compile_error!("rngms-memory: unsupported platform");
