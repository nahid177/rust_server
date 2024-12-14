use warp::{Reply, http::StatusCode};
use mongodb::Collection;
use futures::TryStreamExt;
use serde_json::json;
use std::convert::Infallible;
use crate::models::book::Book;
use chrono::Utc;

pub async fn get_books(collection: Collection<Book>) -> Result<impl Reply, Infallible> {
    let mut cursor = collection.find(None, None).await.expect("Failed to fetch books");

    let mut books: Vec<Book> = vec![];
    while let Some(book) = cursor.try_next().await.expect("Error getting next book") {
        books.push(book);
    }

    Ok(warp::reply::json(&books))
}

pub async fn create_book(collection: Collection<Book>, new_book: Book) -> Result<impl Reply, Infallible> {
    let new_book_with_defaults = Book {
        id: None,
        created_at: Some(Utc::now()),
        ..new_book
    };

    let insert_result = match collection.insert_one(new_book_with_defaults, None).await {
        Ok(result) => result,
        Err(err) => {
            eprintln!("Failed to insert book: {}", err);
            return Ok(warp::reply::with_status(
                warp::reply::json(&json!({"error": format!("Failed to insert book: {}", err)})),
                StatusCode::INTERNAL_SERVER_ERROR,
            ));
        }
    };

    let response = warp::reply::json(&json!({ "success": true, "inserted_id": insert_result.inserted_id }));
    Ok(warp::reply::with_status(response, StatusCode::CREATED))
}
