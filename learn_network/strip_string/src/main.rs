fn main() {
    let a = "/good/fdfei023re32qr32afddddddddddd";
    let b = "/world";
    let c = (&a[a.rfind('/').unwrap() + 1..a.len()]).len();
    let d = a.len() - a.rfind('/').unwrap() - 1;
    // let d = b.trim__matches('/');
    println!("c={:?},d={:?}\n", c, d);
}
