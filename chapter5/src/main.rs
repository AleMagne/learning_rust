fn main() {
    let user = String::from("Ciao");
    let user2 = &user;
    println!("{}, {}", user, user2);
    println!("{}, {}", user, user2);
}      