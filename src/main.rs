mod entities;
use dotenvy::dotenv;
use std::env;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, Database, DatabaseConnection, DbErr, EntityTrait, sea_query::ExprTrait};
use entities::{category, task};
use std::{collections::HashMap, io::{self, Write}};

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
        Err(_) => println!("Необходимо вводить только цифры."),
        }
    };
    input_i32
}

async fn add_category(db: &DatabaseConnection, name: String) -> Result<category::Model, DbErr> {
    category::ActiveModel {
        name: Set(name),
        ..Default::default()
    }.insert(db).await
}

async fn create_category (db: &DatabaseConnection) -> Result<category::Model, DbErr> {
    let mut name_trimmed = input_str("Введите нахвание новой категории: ");
    if name_trimmed.is_empty() {
        name_trimmed = "Новая категория".to_string();
    }
    match add_category(&db, name_trimmed.to_owned()).await {
        Ok(category) => {
            println!("Успешно создана категория: {} с ID: {}", category.name, category.id);
            Ok(category)
        }
        Err(e) => {
            println!("Ошибка при создании категории: {}", e);
            Err(e)
        }
    }
}

async fn delete_category(db: &DatabaseConnection) -> Result<(), DbErr> {
    show_all_categories(db).await?;
    let categ_id = input_i32("Введите ID категории: ");
    category::Entity::delete_by_id(categ_id).exec(db).await?;
    println!("Категория с ID {} была удалена", categ_id);
    Ok(())
}

async fn show_all_categories(db: &DatabaseConnection) -> Result<(), DbErr> {
    let categories: Vec<category::Model> = category::Entity::find().all(db).await?;
    for category in categories {
        println!("Категория: {}, ID: {}", category.id, category.name);
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

async fn create_task(db: &DatabaseConnection) -> Result<(), DbErr>{
    let title_trimmed = input_str("Введите название: ");
    if title_trimmed.is_empty() {
        println!("Название не может быть пустым.");
        return Ok(());
    }
    let desc_trimmed = input_str("Введите описание (если не хотите, нажмите enter): ");

    show_all_categories(db).await?;
    let mut categ_id = input_i32("Выберите категорию по ID или введите 0 для создания новой: ");
    
    if categ_id == 0 {
        let new_categ = create_category(db).await?;
        categ_id = new_categ.id;
    }
        
    match add_task(&db, title_trimmed, desc_trimmed, categ_id).await {
        Ok(task) => {
            println!("Задача успешно создана с заголовком {} и ID {} и категорией {}", 
            task.title, task.id, task.category_id);
            Ok(())
        },
        Err(e) => {
            println!("Ошибка при создании: {}", e);
            Err(e)
        } 
    }   
}

async fn delete_task(db: &DatabaseConnection) -> Result<(), DbErr> {
    println!("Вывести все задачи или по категории?");
    loop {
        let choice = input_i32("Введите 1, если все, и 2, если по категории: ");
        match choice {
            1 => {
                if let Err(e) = show_all_tasks(db).await {
                    println!("Ошибка при выводе задач.");
                }
                break;
            },
            2 => {
                if let Err(e) = print_tasks_by_category(db).await {
                    println!("Ошибка при выводе задач по категории.");
                }
                break;
            }
            _ => println!("Нет такого варианта."),
        }
    }
    let task_id = input_i32("Введите ID задачи: ");
    task::Entity::delete_by_id(task_id).exec(db).await?;
    println!("Задача с ID {} была удалена.", task_id);
    Ok(())
}

async fn show_all_tasks(db: &DatabaseConnection) -> Result<(), DbErr> {
    let categories: Vec<category::Model> = category::Entity::find().all(db).await?;
    let tasks_categories: HashMap<i32, category::Model> = categories
            .into_iter()
            .map(|categ| (categ.id, categ))
            .collect();
    let tasks: Vec<task::Model> = task::Entity::find().all(db).await?;

    for t in tasks {
        let category_name_for_task = match tasks_categories.get(&t.category_id) {
            Some(categ) => &categ.name,
            None => "without category",
        };
        println!("ID: {}. Задача: [{}] | Описание: {} | Категория: {} | Отметка о выполнении: {}", 
            t.id, t.title, t.description, category_name_for_task, t.is_done);
    }

    Ok(())
} 

async fn print_tasks_by_category(db: &DatabaseConnection) -> Result<(), DbErr>{
    let tasks: Vec<task::Model> = task::Entity::find().all(db).await?;
    show_all_categories(db).await?;
    let categ_id = input_i32("Введите ID категории для вывода задач: ");
        
    for t in tasks {
        if t.category_id == categ_id {
            println!("Задача: [{}] | Описание: {} | Отметка о выполнении: {}", t.title, t.description, t.is_done);
        }
    } 
    
    Ok(())
}

async fn mark_as_done(db: &DatabaseConnection) -> Result<(), DbErr> {
    show_all_tasks(db).await?;
    let task_id = input_i32("Введите ID задачи, которую хотите отметить выполненной: ");
    match task::Entity::find_by_id(task_id).one(db).await? {
        Some(task) => {
            let mut active_task: task::ActiveModel = task.into();
            active_task.is_done = Set(true);
            active_task.update(db).await?;
            Ok(())
        },
        None => {
            println!("Задачи с ID {} не существует.", task_id);
            Ok(())
        },
    }
}

async fn main_menu(db: &DatabaseConnection) -> Result<(), DbErr>{
    loop {
    println!("---ЗАМЕТКИ---");
    println!("Меню навигации: ");
    println!("1. Добавить задачу");
    println!("2. Вывести все задачи");
    println!("3. Вывести все задачи из категории");
    println!("4. Удалить задачу");
    println!("5. Вывести все категории");
    println!("6. Добавить новую категорию");
    println!("7. Удалить категорию");
    println!("8. Отметить задачу выполненной");
    println!("0. Завершить работу");

    let mut user_input = input_i32("Выберите пункт меню: ");
        
    match user_input {
        1 => {
            if let Err(e) = create_task(db).await {
                println!("Ошибка при создании задачи.");
            }
        },
        2 => {
            if let Err(e) = show_all_tasks(db).await {
                println!("Ошибка при выводе задач.");
            }
        },
        3 => {
            if let Err(e) = print_tasks_by_category(db).await {
                println!("Ошибка при выводе задач по категории.");
            }
        },
        4 => {
            if let Err(e) = delete_task(db).await {
                println!("Ошибка при удалении задачи.");
            }
        }
        5 => {
            if let Err(e) = show_all_categories(db).await {
                println!("Ошибка при выводе категорий.");
            }
        },
        6 => {
            if let Err(e) = create_category(db).await {
                println!("Ошибка при создании категории.");
            }
        },
        7 => {
            if let Err(e) = delete_category(db).await {
                println!("Ошибка при удалении категории.");
            }
        }
        8 => {
            if let Err(e) = mark_as_done(db).await {
                println!("Ошибка при отметки выолненной.");
            }
        },
        0 => {
            println!("Выход из программы...");
            return Ok(());
        },
    
        _ => println!("Такого пункта не существует!"),
    }
}
}

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").unwrap();
    let db = Database::connect(db_url).await?;
    println!("Соединение с базой установлено!");
    
    main_menu(&db).await?;

    Ok(())
}
