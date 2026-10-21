#[cfg(windows)] mod windows;
#[cfg(windows)] pub use windows::*;

#[cfg(target_vendor = "apple")] mod apple;
#[cfg(target_vendor = "apple")] pub use apply::*;

#[cfg(all(unix, not(target_vendor = "apple")))] mod unix;
#[cfg(all(unix, not(target_vendor = "apple")))] pub use unix::*;

#[cfg(not(any(windows, target_vendor = "apple", unix)))] compile_error!("rngms-memory: unsupported platform");
