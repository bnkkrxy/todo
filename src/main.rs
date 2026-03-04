mod entities;
use sea_orm::{Database, DbErr, EntityTrait, ActiveModelTrait, ActiveValue::Set, DatabaseConnection};
use entities::{category, task};
use std::io;

async fn add_category(db: &DatabaseConnection, name: String) -> Result<category::Model, DbErr> {
    category::ActiveModel {
        name: Set(name),
        ..Default::default()
    }.insert(db).await
}

async fn enter_category (db: &DatabaseConnection) {
    let mut name = String::new();
}

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    let db_url = "postgres://myuser:5521@127.0.0.1:5433/tododb";
    let db = Database::connect(db_url).await?;
    println!("Соединение с базой установлено!");


    match add_category(&db, "Работа".to_owned()).await {
        Ok(cat) => println!("Успешно создана категория: {} с ID: {}", cat.name, cat.id),
        Err(e) => eprintln!("Ошибка при создании категории: {}", e),
    }

    Ok(())
}
