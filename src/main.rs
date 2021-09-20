mod user;

fn main() {
    println!("Hello, world!");
    let user = user::User::new();
    println!("{:?}", user);
}
