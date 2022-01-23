fn main() {
	let s1 = String::from("hello");
//	let s2 = s1;
	//println!("{},world",s1);//error
//	println!("{},world",s2);
	let s2 = s1.clone();
	println!("s1 = {},s2 = {}",s1,s2); 
	let s3 = String::from("Hello");
	takes_ownership(s3);
//	takes_ownership(s3);//error
	let x = 5;
	makes_copy(x);
	
}

fn takes_ownership(some_string: String){
	println!("{}",some_string);
}

fn makes_copy(some_integer: i32){
	println!("{}",some_integer);
}
