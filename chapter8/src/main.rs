fn main() {
    let mut v :Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("There are {} elements", v.len());
    println!("the first element is {}", v[0]);
}
