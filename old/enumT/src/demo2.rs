#[derive(Debug)]

enum Book {
    Papery(u32),
    Electronic(String),
}


fn main() {
    let book = Book::Papery(1001);
    let ebook = Book::Electronic(String::from("url://..."));
    println!("{:?}", book);
}
