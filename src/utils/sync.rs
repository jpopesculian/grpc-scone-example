use std::io::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub fn block_until_user_exit() -> Result<(), Error> {
    let term = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::SIGINT, Arc::clone(&term))?;
    signal_hook::flag::register(signal_hook::SIGTERM, Arc::clone(&term))?;
    while !term.load(Ordering::Relaxed) {
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
    Ok(())
}
