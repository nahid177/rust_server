use warp::Filter;
use warp::http::Method;

mod handlers;
mod models;
mod db;

use handlers::book_handler::{get_books, create_book};
use models::book::Book;
use db::mongodb::connect_db;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();  // Load environment variables

    let db = connect_db().await;  // Connect to MongoDB
    let books_collection = db.collection::<Book>("books");

    // Clone the collection for get and post routes
    let books_collection_clone_for_get = books_collection.clone();
    let get_books_route = warp::path("books")
        .and(warp::get())
        .and_then(move || get_books(books_collection_clone_for_get.clone()));

    let books_collection_clone_for_post = books_collection.clone();
    let create_book_route = warp::path("books")
        .and(warp::post())
        .and(warp::body::json())  // Ensure body is handled as JSON
        .and_then(move |new_book: Book| create_book(books_collection_clone_for_post.clone(), new_book));

    // Define CORS filter
    let cors = warp::cors()
        .allow_any_origin()  // Allow requests from any origin
        .allow_methods(&[Method::GET, Method::POST])
        .allow_headers(vec!["Content-Type", "Authorization"]);

    // Combine routes with CORS filter
    let routes = get_books_route.or(create_book_route).with(cors);

    println!("Server started on http://localhost:3030");
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
