use actix_web::{post, web, App, HttpServer, HttpResponse};
use actix_cors::Cors;
use serde_json::json;
use std::sync::Mutex;

mod db;
mod types;

use db::{User, UserStore};
use types::CoverLetterInput;

struct MyData {
    users: Mutex<UserStore>
}

// API 端点:
// - POST /register: 用户注册
// - POST /login: 用户登录
// - POST /api/step1: 处理第一步

// handle register
/// 检查：
/// 1. 用户名不能为空
/// 2. 密码不能为空
/// 3. 用户名不能重复
#[post("/register")]

async fn register(user: web::Json<User>, data: web::Data<MyData>) -> HttpResponse {
    // check if username or password is empty
    if user.name.trim().is_empty() {
        return HttpResponse::BadRequest().json(json!({
            "message": "Username is empty"
        }));
    }
    if user.password.trim().is_empty() {
        return HttpResponse::BadRequest().json(json!({
            "message": "Password is empty"
        }));
    }

    let mut my_store = data.users.lock().unwrap();

    // try to add the user
    if my_store.add_user(user.into_inner()).unwrap_or(false) {
        HttpResponse::Ok().json(json!({
            "message": "Welcome!:D"
        }))
    } else {
        HttpResponse::BadRequest().json(json!({
            "message": "Username already exists"
        }))
    }
}


// login stuff
/// 检查：
/// 1. 用户名不能为空
/// 2. 密码不能为空
/// 3. 用户名和密码必须匹配

#[post("/login")]
async fn login(user: web::Json<User>, data: web::Data<MyData>) -> HttpResponse {
    if user.name.trim().is_empty() {
        return HttpResponse::BadRequest().json(json!({
            "message": "Username is empty"
        }));
    }
    if user.password.trim().is_empty() {
        return HttpResponse::BadRequest().json(json!({
            "message": "Password is empty"
        }));
    }
    let store = data.users.lock().unwrap();
    
    // check if user exists and pwd is ok
    if store.verify_login(&user.name, &user.password).unwrap_or(false) {
        HttpResponse::Ok().json(json!({
            "message": "Login success"
        }))
    } else {
        HttpResponse::BadRequest().json(json!({
            "message": "Wrong username or password"
        }))
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // init the user store
    let store = UserStore::new().expect("x-x");
    let my_data = web::Data::new(MyData {
        users: Mutex::new(store)
    });

    println!("Server is up at http://localhost:8081 :D");
    HttpServer::new(move || {
        // CORS 配置，前端地址：http://localhost:8080 或 http://127.0.0.1:8080，因为现在前端和后端用的两个terminal，所以用了cros
        let cors = Cors::default()
            .allowed_origin("http://127.0.0.1:8080")
            .allowed_origin("http://localhost:8080")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec!["Content-Type"])
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(my_data.clone())
            .service(register)
            .service(login)
            .route("/api/step1", web::post().to(handle_step1))
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}

/// 输入格式:
/// ```json
/// {
///     "step": 1,
///     "content": "用户输入的内容",
///     "user_id": "用户ID（没登录的时候是Null）"
/// }
/// ```
/// 
/// 返回格式:
/// ```json
/// {
///     "message": "成功",
///     "content": "用户输入的内容"
/// }
/// ```

// 处理step 1
async fn handle_step1(input: web::Json<CoverLetterInput>) -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "message": "Step 1 received successfully",
        "content": input.content
    }))
} 