use std::sync::atomic::Ordering;

fn main() -> std::io::Result<()> {
    let mut server = rebrickable_server::RebrickableServer::start()?;
    let running = server.clone_stop_handle();
    ctrlc::set_handler(move || {
        println!("Ctrl+C received, shutting down...");
        running.store(false, Ordering::SeqCst);
    })
    .expect("failed to set Ctrl+C handler");

    println!("Press Ctrl+C to stop the server");

    server.join();
    Ok(())
}
