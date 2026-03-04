mod entities;
use sea_orm::{Database, DbErr, EntityTrait, ActiveModelTrait, ActiveValue::Set, DatabaseConnection};
use entities::{category, task};
use std::io::{self, Write};

async fn add_category(db: &DatabaseConnection, name: String) -> Result<category::Model, DbErr> {
    category::ActiveModel {
        name: Set(name),
        ..Default::default()
    }.insert(db).await
}

async fn enter_category (db: &DatabaseConnection) {
    print!("category: ");
    io::stdout().flush().unwrap();
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .unwrap();
    let name_trimmed = name.trim().to_string();
    if !name_trimmed.is_empty() {
        match add_category(&db, name_trimmed.to_owned()).await {
            Ok(cat) => println!("Успешно создана категория: {} с ID: {}", cat.name, cat.id),
            Err(e) => eprintln!("Ошибка при создании категории: {}", e),
    }
    }
}

async fn delete_category(db: &DatabaseConnection, categ_id: i32) -> Result<(), DbErr> {
    category::Entity::delete_by_id(categ_id).exec(db).await?;
    println!("Категория с ID {} была удалена", categ_id);
    Ok(())
}

async fn enter_delete_category(db: &DatabaseConnection) -> Result<(), DbErr> {
    let id: i32 = loop {
    print!("category_id: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .unwrap();
    match input.trim().parse::<i32>() {
        Ok(num) => break num,
        Err(_) => println!("error"),

    };
    };
    delete_category(&db, id).await?;
    Ok(())
}

async fn show_all_categories(db: &DatabaseConnection) -> Result<(), DbErr> {
    let categories: Vec<category::Model> = category::Entity::find().all(db).await?;
    for category in categories {
        println!("Category: {}, ID: {}", category.id, category.name);
    }
    Ok(())
}

async fn add_task(db: &DatabaseConnection, title: String, desc: String, categ_id: i32) -> Result<task::Model, DbErr> {
    task::ActiveModel {
        title: Set(title),
        description: Set(desc),
        category_id: Set(categ_id),
        ..Default::default()
    }.insert(db).await
}

async fn enter_tasks() {}

async fn show_all_tasks() {} //задачи+категории

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    let db_url = "postgres://myuser:5521@127.0.0.1:5433/tododb";
    let db = Database::connect(db_url).await?;
    println!("Соединение с базой установлено!");
    
    //match для взаимодействия
    show_all_categories(&db).await;
    enter_delete_category(&db).await;
    show_all_categories(&db).await;

    Ok(())
}
