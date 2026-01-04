use std::sync::{Arc, atomic::AtomicBool};

fn main() -> std::io::Result<()> {
    let running = Arc::new(AtomicBool::new(true));
    let _ = rebrickable_server::RebrickableServer::start_with_arc(Arc::clone(&running))?;
    ctrlc::set_handler(move || {
        println!("Ctrl+C received, shutting down...");
        running.store(true, std::sync::atomic::Ordering::SeqCst);
        std::process::exit(0);
    })
    .expect("failed to set Ctrl+C handler");

    println!("Press Ctrl+C to stop");
    loop {
        std::thread::park();
    }
}
