fn main() {
    println!("Hello, world!");
    let a = Student { apple: 12 };
    println!("a={}", a.apple);
}

pub trait EatApplePlus {
    fn get_apple(&self) -> usize;
    fn set_apple(&mut self, num: usize);
    fn eat_apple(&mut self) {
        self.set_apple(self.get_apple() + 1);
        println!("{} apple eaten", self.get_apple());
    }
}

pub struct Student {
    apple: usize,
}
impl EatApplePlus for Student {
    fn get_apple(&self) -> usize {
        self.apple
    }
    fn set_apple(&mut self, num: usize) {
        self.apple = num;
    }
}
