fn main() {
    println!("Hello, world!");
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    let mut v1 = vec![1, 2, 3];
    let third: &i32 = &v[2];
    println!("The third element is {}", third);
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
    let does_not_exist = &v[100];
    let does_not_exist = v.get(100);
}
