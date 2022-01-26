pub mod front_of_house;
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // 从 crate 根开始，以 crate 名或者字面值 crate 开头
    // Absolute path,官方推荐绝对路径
    // crate::front_of_house::hosting::add_to_waitlist();
    //---等价于下面一行
    hosting::add_to_waitlist(); //类似于创建软链接
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    // 以 self 、 super 或当前模块的标识符开头
    // Relative path
    // front_of_house::hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

// back_of_house 模块和 serve_order 函数之间可能具有某种关联关系
//使用绝对路径
fn serve_order() {}
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }
    fn cook_order() {}
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}
