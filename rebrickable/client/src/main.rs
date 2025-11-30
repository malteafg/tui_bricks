use rebrickable_client::ClientDB;
use rebrickable_database_api::RebrickableDB;

fn main() -> std::io::Result<()> {
    let db = ClientDB::new();

    let part = db.part_from_id(&"4070".into());
    println!("Response: {:#?}", part);

    let color = db.color_from_name("Blue");
    println!("Response: {:#?}", color);

    Ok(())
}
