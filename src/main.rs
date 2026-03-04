mod entities;
use sea_orm::{Database, DbErr, EntityTrait, ActiveModelTrait, ActiveValue::Set, DatabaseConnection};
use entities::{category, task};
use std::io::{self, Write};

fn input_str(output: &str) -> String{
    print!("{}", output);
    io::stdout().flush().unwrap();
    let mut input_str = String::new();
    io::stdin()
        .read_line(&mut input_str)
        .unwrap();
    let input_str_trimmed = input_str.trim().to_string();
    input_str_trimmed
}

fn input_i32(output: &str) -> i32 {
    let input_i32: i32 = loop {
    print!("{}", output);
    io::stdout().flush().unwrap();
    let mut input_str = String::new();
    io::stdin()
        .read_line(&mut input_str)
        .unwrap();
    match input_str.trim().parse::<i32>() {
        Ok(num) => break num,
        Err(_) => println!("error"),

    };
    };
    input_i32
}

async fn add_category(db: &DatabaseConnection, name: String) -> Result<category::Model, DbErr> {
    category::ActiveModel {
        name: Set(name),
        ..Default::default()
    }.insert(db).await
}

async fn enter_category (db: &DatabaseConnection) {
    let name_trimmed = input_str("category: ");
    if !name_trimmed.is_empty() {
        match add_category(&db, name_trimmed.to_owned()).await {
            Ok(cat) => println!("Успешно создана категория: {} с ID: {}", cat.name, cat.id),
            Err(e) => eprintln!("Ошибка при создании категории: {}", e),
        }
    }
}

async fn delete_category(db: &DatabaseConnection) -> Result<(), DbErr> {
    let categ_id = input_i32("id_category: ");
    category::Entity::delete_by_id(categ_id).exec(db).await?;
    println!("Категория с ID {} была удалена", categ_id);
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

async fn get_tasks_by_category() {}

async fn mark_as_done() {}

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    let db_url = "postgres://myuser:5521@127.0.0.1:5433/tododb";
    let db = Database::connect(db_url).await?;
    println!("Соединение с базой установлено!");
    
    //match для взаимодействия
    show_all_categories(&db).await;
    delete_category(&db).await;

    show_all_categories(&db).await;
    Ok(())
}
