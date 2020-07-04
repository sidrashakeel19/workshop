// Necessary imports here 
use actix_web::{App, get, post, put, HttpResponse, HttpServer};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {    
    // Initializing server
    HttpServer::new(|| {
        App::new()
        // Associating service(s)/route_handler(s)
         .service(find_all)
         .service(find)
         .service(create)
    })
    // Binding socket address server will receive requests on
    .bind("127.0.0.1:5000")?
    .run()
    .await
}
// ----------------------- Route Handlers ------------------------

// This route handler will list all the data available
#[get("/students")]
async fn find_all() -> HttpResponse {
    HttpResponse::Ok().body("List of students")
}

// This route handler will list data with specific id
#[get("/students/{id}")]
async fn find() -> HttpResponse {
    HttpResponse::Ok().body("Listing student with specific id")
}

// This route handler will create a new record
#[post("/students")]
async fn create() -> HttpResponse {
    HttpResponse::Ok().body("Creating a new record......")
}

// This route handler will update an existing record
#[put("/students/{id}")]
async fn update() -> HttpResponse {
    HttpResponse::Ok().body("Updating record......")
}