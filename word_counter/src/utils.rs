pub fn add(a: i32, b: i32) -> i32 {
  a + b
}

pub fn say_hello(name: &str) -> String {
  format!("Hello, {}!", name)
}

fn private_function() {
  println!("This is private");
}