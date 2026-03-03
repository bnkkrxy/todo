mod entities;
use sea_orm::{Database, DbErr, EntityTrait, ActiveModelTrait, ActiveValue::Set};
use entities::{category, task};

fn add_category() {

}

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    let db_url = "postgres://myuser:5521@127.0.0.1:5433/tododb";
    let db = Database::connect(db_url).await?;
    println!("Соединение с базой установлено!");
    Ok(())
}
