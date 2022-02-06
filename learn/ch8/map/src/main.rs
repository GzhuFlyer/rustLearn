fn main() {
    println!("Hello, world!");
    map_1();
    map_2();
    map_3();
    map_4();
    map_5();
    update_map();
    entry_map();
    count_map();
}

fn map_1() {
    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
}
// 另一个构建哈希 map 的方法是使用一个元组的 vector 的 collect 方法，其中每个元组包含一个
// 键值对。 collect 方法可以将数据收集进一系列的集合类型，包括 HashMap 。例如，如果队伍的
// 名字和初始分数分别在两个 vector 中，可以使用 zip 方法来创建一个元组的 vector，其中
// “Blue” 与 10 是一对，依此类推。接着就可以使用 collect 方法将这个元组 vector 转换成一个
// HashMap ，
fn map_2() {
    use std::collections::HashMap;
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
}

fn map_3() {
    use std::collections::HashMap;
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value); // 这里 field_name 和 field_value 不再有效，
                                         // 尝试使用它们看看会出现什么编译错误！
}
// 哈希 map 和所有权
fn map_4() {
    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
}
// 访问哈希 map 中的值
fn map_5() {
    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}
// 覆盖一个值
fn update_map() {
    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);
}
// 只在键没有对应值时插入
fn entry_map() {
    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);
}
fn count_map() {
    use std::collections::HashMap;
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
