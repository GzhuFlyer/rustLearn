fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    // let score = scores.get(&team_name);
    // let ttt = String::from("s: &str")
    let mut score = scores.get_mut("Blue").unwrap();
    let a = score;
    println!("{}", a);

    let score1 = scores.get_mut("Blue1");
    match score1 {
        None => None,
        Some(a) => Some(a),
    };
    //   match coin {
    //     Coin::Penny => 1,
    //     Coin::Nickel => 5,
    //     Coin::Dime => 10,
    //     Coin::Quarter(state) => {
    //         println!("State quarter from {:?}!", state);
    //         25
    //     }
    // }
    // let coin = Coin::Quarter(UsState::Alaska);
    // let mut count = 0;
    // // match coin {
    // //     Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    // //     _ => count += 1,
    // // }
    // //等价于
    // if let Coin::Quarter(state) = coin {
    //     println!("State quarter from {:?}!", state);
    // } else {
    //     count += 1;
    // }

    // match x {
    //     None => None,
    //     Some(i) => Some(i + 1),
    // }
    // let b = score1;
    // println!("{:?}", b);
    // println!("{:?}", score);
    // println!("{}", scores.get("Blue").unwrap());
}
