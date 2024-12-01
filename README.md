# RUST Calculator API

This API was created for studying purposes. I was interested in learning Rust and started with a simple server to put into practice basic concepts that I learned. The primary goal was to understand how to structure a web service using Rust.

http
````
GET http://127.0.0.1:8080/
````

### Calculator Operations
All calculator endpoints accept POST requests with a JSON body:
json
```
{
"a": 5,
"b": 5
}
```

Available endpoints:
- `POST /calculator/add` - Performs addition
- `POST /calculator/sub` - Performs subtraction
- `POST /calculator/mul` - Performs multiplication
- `POST /calculator/div` - Performs division
- `POST /calculator/pow` - Performs power operation

## Project Structure
```
src/
├── main.rs # Application entry point
├── routes.rs # Route configurations
└── calculator/
└── mod.rs # Calculator logic
```

## Running the Project

1. Clone the repository
2. Use Cargo to build and run the project:

bash
```
cargo run
```

3. The server will start at `http://127.0.0.1:8080`

## Learning Notes

- **Actix Scopes**: Help organize routes logically, such as grouping calculator endpoints
- **Serde Magic**: Automatically converts between Rust structs and JSON
- **Handler Pattern**: Each endpoint has a dedicated async handler function
- **Type Safety**: Rust's type system ensures correct data handling

## May the Rust be with you! 🦀✨

Remember: Just as a Jedi builds their lightsaber, we build our APIs with precision, care, and proper error handling!

<img src="https://i.giphy.com/Dmydf2Zf2kOys.webp" style="width: 100%;">
