//! Test file for CodeQL queries
//! This file contains examples of all the features we want to detect with our CodeQL queries

use chrono::{Utc, DateTime, NaiveDateTime};

// Primitive types
const BOOL_TRUE: bool = true;
const BOOL_FALSE: bool = false;

const INTEGER_VALUE: i32 = 42;
const FLOAT_VALUE: f64 = 3.14159;

const STRING_VALUE: &str = "Hello, World!";
const CHAR_VALUE: char = 'A';

// Date/time usage
fn get_current_time() -> DateTime<chrono::Utc> {
    Utc::now()
}

// Character operations
fn char_example() -> char {
    let ch = 'B';
    ch
}

// Struct definition (Class Type)
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

// Enum definition (Class Type)
#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

// Function definition (Class Type)
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

// Trait definition (Class Type)
trait Drawable {
    fn draw(&self);
}

// Algebraic Data Types
#[derive(Debug)]
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Triangle(f64, f64, f64), // Using tuple style
}

// Option pattern (Algebraic Data Type)
fn find_value(key: &str) -> Option<i32> {
    if key == "valid" {
        Some(42)
    } else {
        None
    }
}

// Result pattern (Algebraic Data Type)
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

// Type union through enum
#[derive(Debug)]
enum Value {
    Int(i32),
    Float(f64),
    Text(String),
}

// Dynamic dispatch pattern
struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing circle with radius {}", self.radius);
    }
}

impl Drawable for Rectangle {
    fn draw(&self) {
        println!("Drawing rectangle {}x{}", self.width, self.height);
    }
}

fn render_object(object: Box<dyn Drawable>) {
    object.draw();
}

fn main() {
    // Using primitive types
    let is_active = true;
    let count = 100;
    let price = 99.99;
    let message = "Welcome to our application!";
    let letter = 'X';
    
    // Using date/time
    let now = Utc::now();
    let timestamp = NaiveDateTime::from_timestamp_opt(1609459200, 0).unwrap();
    
    // Using character
    let character = char_example();
    
    // Using structs
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
    };
    
    // Using enums
    let favorite_color = Color::Blue;
    
    // Using functions
    let greeting = greet("Bob");
    
    // Using algebraic data types
    let circle = Shape::Circle { radius: 5.0 };
    let rectangle = Shape::Rectangle { width: 10.0, height: 20.0 };
    let triangle = Shape::Triangle(3.0, 4.0, 5.0);
    
    // Using Option pattern
    let some_value = find_value("valid");
    let none_value = find_value("invalid");
    
    // Using Result pattern
    let division_result = divide(10.0, 2.0);
    let error_result = divide(10.0, 0.0);
    
    // Using type union
    let int_value = Value::Int(42);
    let float_value = Value::Float(3.14);
    let text_value = Value::Text("Hello".to_string());
    
    // Using dynamic dispatch
    let circle_obj = Box::new(Circle { radius: 5.0 });
    let rect_obj = Box::new(Rectangle { width: 10.0, height: 20.0 });
    
    render_object(circle_obj);
    render_object(rect_obj);
    
    println!("Test completed successfully!");
}