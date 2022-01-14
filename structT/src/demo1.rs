#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}
struct Site {
    d: String,
    name: String,
    found: u32
}
struct Color(u8, u8, u8);
struct Point(f64, f64);

fn main() {
	println!("Hello, world!");
	let r = Site {
		d: String::from("sdfdf"),
		name: String::from("owee"),
		found: 232
	};
	let b = Site {
		d: String::from("aaa"),
		name: String::from("bbb"),
		..r
	};

	let black = Color(0, 0, 0);
	let origin = Point(0.0, 0.0);
	    struct Color(u8, u8, u8);
    struct Point(f64, f64);

    let black = Color(0, 0, 0);
    let origin = Point(0.0, 0.0);

    println!("black = ({}, {}, {})", black.0, black.1, black.2);
    println!("origin = ({}, {})", origin.0, origin.1);
	let rect1 = Rectangle { width: 30, height: 50 };
    println!("rect1 is {:?}", rect1);

}	
