use std::env;

fn main() {
    let _ = dotenv::dotenv();
    let foo = env::var("FOO").unwrap();
    println!("FOO={foo}");
}
