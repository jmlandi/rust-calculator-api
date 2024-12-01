use actix_web:: {web, HttpResponse, Scope};
use crate::calculator::{Calculator as Calc, CalculatorInput as CalcInput};

/// Main routes configuration
/// Returns a Scope containing the health check endpoint
pub fn main_routes() -> Scope {
    web::scope("/")
        .route("", web::get().to(health_check))
}

/// Health check endpoint handler
/// Returns a simple JSON response indicating API status
async fn health_check() -> HttpResponse {
    println!("[MAIN] Someone health checked the API.");
    HttpResponse::Ok().json("API is running")
}

/// Calculator routes configuration
/// Returns a Scope containing all calculator endpoints
pub fn calculator_routes() -> Scope {
    web::scope("/calculator")
        .route("/add", web::post().to(add))
        .route("/sub", web::post().to(sub))
        .route("/mul", web::post().to(mul))
        .route("/div", web::post().to(div))
        .route("/pow", web::post().to(pow))
}

async fn add(input: web::Json<CalcInput>) -> HttpResponse {
    println!("[CALCULATOR] Add operation requested.");
    let result = Calc::add(input.into_inner());
    HttpResponse::Ok().json(result)
}

async fn sub(input: web::Json<CalcInput>) -> HttpResponse {
    println!("[CALCULATOR] Sub operation requested.");
    let result = Calc::sub(input.into_inner());
    HttpResponse::Ok().json(result)
}

async fn mul(input: web::Json<CalcInput>) -> HttpResponse {
    println!("[CALCULATOR] Mul operation requested.");
    let result = Calc::mul(input.into_inner());
    HttpResponse::Ok().json(result)
}

async fn div(input: web::Json<CalcInput>) -> HttpResponse {
    println!("[CALCULATOR] Div operation requested.");
    let result = Calc::div(input.into_inner());
    HttpResponse::Ok().json(result)
}

async fn pow(input: web::Json<CalcInput>) -> HttpResponse {
    println!("[CALCULATOR] Pow operation requested.");
    let result = Calc::pow(input.into_inner());
    HttpResponse::Ok().json(result)
}