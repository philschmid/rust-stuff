use futures::stream::{StreamExt, TryStreamExt};
use mongodb::{
    bson::doc,
    options::{ClientOptions, FindOptions},
    results::InsertManyResult,
    Client, Database,
};
use serde::{Deserialize, Serialize};

static MONGODB_URL: &str = "mongodb://localhost:27017";

#[derive(Debug, Serialize, Deserialize)]
struct Book {
    title: String,
    author: String,
}

async fn insert_books(db: Database) -> Result<InsertManyResult, mongodb::error::Error> {
    // Get a handle to a collection of `Book`.
    let typed_collection = db.collection::<Book>("books");

    let books = vec![
        Book {
            title: "The Grapes of Wrath".to_string(),
            author: "John Steinbeck".to_string(),
        },
        Book {
            title: "To Kill a Mockingbird".to_string(),
            author: "Harper Lee".to_string(),
        },
        Book {
            title: "To Kill a Mockingbird".to_string(),
            author: "Adam Lee".to_string(),
        },
        Book {
            title: "To Kill Bill".to_string(),
            author: "John Lee".to_string(),
        },
    ];

    // Insert the books into "mydb.books" collection, no manual conversion to BSON necessary.
    typed_collection.insert_many(books, None).await
}

async fn get_books_by_title(db: Database, title: &str) -> Result<&str, mongodb::error::Error> {
    // Get a handle to a collection of `Book`.
    let typed_collection = db.collection::<Book>("books");

    // options to filter/sort by author desc
    let find_options = FindOptions::builder().sort(doc! { "author": 1 }).build();

    let cursor = typed_collection
        .find(doc! {"title":title}, find_options)
        .await?;

    let v: Vec<_> = cursor.try_collect().await?;

    println!("{:?}", v);
    Ok("ja")
}

async fn get_one_book_by_title(
    db: Database,
    title: &str,
) -> Result<Option<Book>, mongodb::error::Error> {
    // Get a handle to a collection of `Book`.
    let typed_collection = db.collection::<Book>("books");

    let result = typed_collection
        .find_one(doc! {"title":title}, None)
        .await?;
    Ok(result)
}

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    // Parse your connection string into an options struct
    let mut client_options = ClientOptions::parse(MONGODB_URL).await?;

    // Manually set an option
    client_options.app_name = Some("Rust Demo".to_string());

    // Get a handle to the cluster
    let client = Client::with_options(client_options)?;

    // Ping the server to see if you can connect to the cluster
    client
        .database("admin")
        .run_command(doc! {"ping": 1}, None)
        .await?;
    println!("Connected successfully.");

    // List the names of the databases in that cluster
    for db_name in client.list_database_names(None, None).await? {
        println!("{}", db_name);
    }
    let db = client.database("mydb");

    match insert_books(db.clone()).await {
        Ok(r) => println!("{:?}", r),
        Err(e) => panic!("{}", e),
    }
    get_books_by_title(db.clone(), "To Kill a Mockingbird").await?;

    get_one_book_by_title(db.clone(), "To Kill a Mockingbird").await?;

    // delete every element
    db.drop(None).await?;

    Ok(())
}
