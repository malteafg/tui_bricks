
use tokio::net::TcpListener;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4000").await?;
    let db = build_db(); // Simulated DB

    loop {
        let (socket, _) = listener.accept().await?;
        let db = db.clone();
        tokio::spawn(async move {
            let mut reader = BufReader::new(socket);
            let mut line = String::new();

            if reader.read_line(&mut line).await.is_ok() {
                let part_id = line.trim();
                let response = db.get(part_id)
                    .map(|info| info.clone())
                    .unwrap_or_else(|| "Part not found\n".to_string());

                let _ = reader.get_mut().write_all(response.as_bytes()).await;
            }
        });
    }
}

fn build_db() -> std::sync::Arc<HashMap<String, String>> {
    let mut map = HashMap::new();
    map.insert("3001".into(), "Name: Brick 2x4\nColor: Red\nCategory: Bricks\nID: 3001\n".into());
    map.insert("3022".into(), "Name: Plate 2x2\nColor: Blue\nCategory: Plates\nID: 3022\n".into());
    std::sync::Arc::new(map)
}
