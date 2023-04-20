use std::env::var;

fn main() {
    let secret_name = var("secret_user").expect("`secret_user` environment variable not set");
    println!("\"Hello, {}\"", secret_name);
}
