fn main() {
    let v = vec![1, 2, 3, 4, 5];

    // let third:&i32 = &v[100]; //thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 100'
    let gthird: Option<&i32> = v.get(2);
    println!("{:?}", gthird);
    println!("{}", v.len());
}
