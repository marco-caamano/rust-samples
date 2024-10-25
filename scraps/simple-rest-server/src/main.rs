#[macro_use]
extern crate lazy_static;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
}

// In-memory database simulation
lazy_static! {
    static ref USERS: Mutex<Vec<User>> = Mutex::new(Vec::new());
}

// Handler for the GET request
async fn get_users() -> impl Responder {
    // Lock the Mutex to access the vector
    let vec = USERS.lock().unwrap();
    // Dereference all users contained in the "database"
    let users_json = serde_json::to_string(&*vec).unwrap();
    HttpResponse::Ok().body(format!("{}\n", users_json))
}

// Handler for the POST request
async fn add_user(user: web::Json<User>) -> impl Responder {
    let mut vec = USERS.lock().unwrap();
    // Modify the vector safely
    vec.push(user.into_inner());
    HttpResponse::Ok().body("User added\n")
}

// Handler to PATCH (update) an item by id
async fn update_user(item: web::Json<User>) -> impl Responder {
    let mut vec = USERS.lock().unwrap();

    // Find the item by its id
    if let Some(existing_item) = vec.iter_mut().find(|i| i.id == item.id) {
        existing_item.name = item.name.clone();
        return HttpResponse::Ok().body("Item updated\n");
    }

    HttpResponse::NotFound().body("Item not found\n")
}

// Handler to DELETE an item by id
async fn delete_user(path: web::Path<u32>) -> impl Responder {
    let mut vec = USERS.lock().unwrap();
    let id = path.into_inner();

    // Find the index of the item to delete
    if let Some(index) = vec.iter().position(|i| i.id == id) {
        vec.remove(index);
        return HttpResponse::Ok().body("Item deleted\n");
    }

    HttpResponse::NotFound().body("Item not found\n")
}

// Main entry point to start the server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/users", web::get().to(get_users)) // GET /items
            .route("/users", web::post().to(add_user)) // POST /items
            .route("/users", web::patch().to(update_user)) // PATCH /items
            .route("/users/{id}", web::delete().to(delete_user)) // DELETE /items/{id}
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
