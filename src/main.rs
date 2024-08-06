/*

use actix_web::{ delete, get, patch, post, web::{Json, Path}, App, HttpResponse, HttpServer, Responder};
use serde::{ Deserialize, Serialize};
use validator::Validate;


#[get("/pizzas")]
async fn get_pizzas() -> impl Responder {
    HttpResponse::Ok().body("Pizzas Available")
}

#[post("/buypizza")]
async fn buy_pizza(body: Json<BuyPizzaRequest>) -> impl Responder {
    let is_valid = body.validate();

    match is_valid {
        Ok(_) => {
            let pizza_name = body.pizza_name.clone();
            HttpResponse::Ok().body(format!("Pizza entered is {pizza_name}"))
        }
        Err(_) => HttpResponse::Ok().body("Pizza name is requied!")
    }
    
}

#[patch("/updatepizzas/{uuid}")]
async fn update_pizza(update_pizza_url: Path<UpdatePizzaURL>) -> impl Responder {
    let uuid = update_pizza_url.into_inner().uuid;
    HttpResponse::Ok().body(format!("Updating pizza with id of {uuid}"))
}

#[delete("/delete/{uuid}")]
async fn delete_pizza() -> impl Responder {
    HttpResponse::Ok().body("Delete pizza")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new( || App::new()
        .service(get_pizzas)
        .service(buy_pizza)
        .service(update_pizza)
        .service(delete_pizza)
    )
            .bind("127.0.0.1:8080")?
            .run()
            .await
}

#[derive(Validate, Deserialize, Serialize)]
pub struct BuyPizzaRequest {
    #[validate(length(min = 1, message = "pizza name required"))]
    pub pizza_name: String,
}


#[derive(Validate, Deserialize, Serialize)]
pub struct UpdatePizzaURL {
    pub uuid: String,
}

*/




//Implementing database connection --with auto increasing int id
//================================



use actix_web::{ delete, get, patch, post, web::{Json, Path, Data}, App, HttpResponse, HttpServer, Responder};
use serde::{ Deserialize, Serialize};
use validator::Validate;
use dotenv::dotenv;
use sqlx::{self, postgres::PgPoolOptions, Pool, Postgres, FromRow};

pub struct AppState {
    db: Pool<Postgres>
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Error building a connection pool");

    HttpServer::new( move || App::new()
        .app_data(Data::new(AppState { db: pool.clone() }))
        .service(get_users)
        .service(update_user)
        .service(add_user)
        .service(get_user)
        .service(delete_user)
    )
            .bind("127.0.0.1:8080")?
            .run()
            .await
}

#[derive(Serialize, Deserialize, Validate, FromRow)]
struct User {
    #[validate(length(min = 1, message = "user name required"))]
    firstname: String,
    lastname: String,
}

#[get("/users")]
async fn get_users(state: Data<AppState>) -> impl Responder {
    match sqlx::query_as::<_, User>(
        "SELECT *FROM users"
    )
        .fetch_all(&state.db)
        .await
    {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().json("Failed to get users"),
    }
}

#[get("/users/{id}")]
async fn get_user(state: Data<AppState>, id: Path<i32>) -> impl Responder {
    let id: i32 = id.into_inner();
    
    match sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE id = $1"
    )
        .bind(id)
        .fetch_one(&state.db)
        .await
    {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().json("Failed to get user"),
    }
}

#[post("/users")]
async fn add_user(state: Data<AppState>, body: Json<User>) -> impl Responder {
    let is_valid = body.validate();


    match is_valid {
        Ok(_) => {
            match sqlx::query_as::<_, User>(
                "INSERT INTO users (uuid, firstname, lastname) VALUES ($1, $2, $3) RETURNING uuid, firstname, lastname"//
            )
                .bind(body.firstname.to_string())
                .bind(body.lastname.to_string())
                .fetch_one(&state.db)
                .await
            {
                Ok(user) => HttpResponse::Ok().json(user),
                Err(_) => HttpResponse::InternalServerError().json("Failed to create user"),
            }
        }
        Err(_) => HttpResponse::Ok().body("User name is requied!")
    }
    
}

#[patch("/updateuser/{id}")]
async fn update_user(state: Data<AppState>, body: Json<User>, id: Path<i32>) -> impl Responder {
    //let id = id.into_inner().id;
    let id: i32 = id.into_inner();

    let is_valid = body.validate();

    match is_valid {
        Ok(_) => {
            match sqlx::query_as::<_, User>(
                "UPDATE users SET firstname = $1, lastname= $2 WHERE id = $3 RETURNING firstname, lastname"
            )
                .bind(body.firstname.to_string())
                .bind(body.lastname.to_string())
                .bind(id)
                .fetch_all(&state.db)
                .await
            {
                Ok(user) => HttpResponse::Ok().json(user),
                Err(_) => HttpResponse::InternalServerError().json("Failed to update user"),
            }
        }
        Err(_) => HttpResponse::Ok().body("User name is requied!")
    }
}

#[delete("/delete/{id}")]
async fn delete_user(state: Data<AppState>, id: Path<i32>) -> impl Responder {
    let id: i32 = id.into_inner();

    match sqlx::query_as::<_, User>(
        "DELETE FROM users WHERE id = $1 RETURNING firstname, lastname"
    )
        .bind(id)
        .fetch_one(&state.db)
        .await
    {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().json("Failed to delete user"),
    }
}


/* 
//Implementing database connection with uuid, and basic login



==SQL COMMAND WITH UUID FIELD==

CREATE TABLE users2 (
    uuid UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
	email VARCHAR NOT NULL,
	password1 VARCHAR NOT NULL,
	password2 VARCHAR NOT NULL
);




use actix_web::{ delete, get, patch, post, web::{Json, Path, Data}, App, HttpResponse, HttpServer, Responder};
use serde::{ Deserialize, Serialize};
use validator::Validate;
use dotenv::dotenv;
use sqlx::{self, postgres::PgPoolOptions, Pool, Postgres, FromRow};

pub struct AppState {
    db: Pool<Postgres>
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Error building a connection pool");

    HttpServer::new( move || App::new()
        .app_data(Data::new(AppState { db: pool.clone() }))
        .service(get_users)
        .service(update_user)
        .service(add_user)
        .service(get_user)
        .service(delete_user)
    )
            .bind("127.0.0.1:8080")?
            .run()
            .await
}

#[derive(Serialize, Deserialize, Validate, FromRow)]
struct User {
    #[validate(length(min = 1, message = "user name required"))]
    //uuid: String,
    name: String,
    email: String,
    password1: String,
    password2: String
}

#[get("/users")]
async fn get_users(state: Data<AppState>) -> impl Responder {
    match sqlx::query_as::<_, User>(
        "SELECT * FROM users2"
    )
        .fetch_all(&state.db)
        .await
    {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().json("Failed to get users"),
    }
}

#[get("/users/{uuid}")]
async fn get_user(state: Data<AppState>, uuid: Path<String>) -> impl Responder {
    let uuid = uuid.into_inner();
    
    match sqlx::query_as::<_, User>(
        "SELECT * FROM users2 WHERE id = $1"
    )
        .bind(uuid)
        .fetch_one(&state.db)
        .await
    {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().json("Failed to get user"),
    }
}

#[post("/users")]
async fn add_user(state: Data<AppState>, body: Json<User>) -> impl Responder {
    let is_valid = body.validate();


    match is_valid {
        Ok(_) => {
            match sqlx::query_as::<_, User>(
                "INSERT INTO users2 (name, email, password1, password2) VALUES ($1, $2, $3, $4) RETURNING name, email"//
            )
                .bind(body.name.to_string())
                .bind(body.email.to_string())
                .bind(body.password1.to_string())
                .bind(body.password2.to_string())
                .fetch_one(&state.db)
                .await
            {
                Ok(user) => HttpResponse::Ok().json(user),
                Err(_) => HttpResponse::InternalServerError().json("Failed to create user"),
            }
        }
        Err(_) => HttpResponse::Ok().body("User name is requied!")
    }
    
}

#[patch("/updateuser/{uuid}")]
async fn update_user(state: Data<AppState>, body: Json<User>, uuid: Path<String>) -> impl Responder {
    //let id = id.into_inner().id;
    let uuid= uuid.into_inner();

    let is_valid = body.validate();

    match is_valid {
        Ok(_) => {
            match sqlx::query_as::<_, User>(
                "UPDATE users2 SET name = $1, email= $2, password1 = $3, password2 = $4 WHERE id = $5 RETURNING name, email"
            )
                .bind(body.name.to_string())
                .bind(body.email.to_string())
                .bind(body.password1.to_string())
                .bind(body.password2.to_string())
                .bind(uuid)
                .fetch_all(&state.db)
                .await
            {
                Ok(user) => HttpResponse::Ok().json(user),
                Err(_) => HttpResponse::InternalServerError().json("Failed to update user"),
            }
        }
        Err(_) => HttpResponse::Ok().body("User name is requied!")
    }
}

#[delete("/delete/{uuid}")]
async fn delete_user(state: Data<AppState>, uuid: Path<String>) -> impl Responder {
    let uuid= uuid.into_inner();

    match sqlx::query_as::<_, User>(
        "DELETE FROM users2 WHERE id = $1 RETURNING firstname, lastname"
    )
        .bind(uuid)
        .fetch_one(&state.db)
        .await
    {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().json("Failed to delete user"),
    }
}

*/
