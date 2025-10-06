
use std::env;
use std::io::{Read, Write};
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: lego-preview <part_id>");
        std::process::exit(1);
    }

    let part_id = &args[1];

    let mut stream = TcpStream::connect("127.0.0.1:4000")?;
    stream.write_all(part_id.as_bytes())?;
    stream.write_all(b"\n")?;

    let mut response = String::new();
    stream.read_to_string(&mut response)?;

    println!("{}", response);
    Ok(())
}
