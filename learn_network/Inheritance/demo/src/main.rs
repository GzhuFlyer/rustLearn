fn main() {
    let mut s = Student { eaten_apple: 10 };
    s.eat_apple();
    // println!("{}", s.eaten_apple);
}

pub trait EatApple {
    fn eat_apple(&mut self);
}

pub struct People {
    eaten_apple: u8,
}
impl EatApple for People {
    fn eat_apple(&mut self) {
        self.eaten_apple += 1;
        println!("{} apple eaten", self.eaten_apple);
    }
}

pub struct Student {
    eaten_apple: u8,
    // other fields
}
impl EatApple for Student {
    fn eat_apple(&mut self) {
        self.eaten_apple += 1;
        println!("{} apple eaten", self.eaten_apple);
    }
}
impl Student {
    //other methods
}

pub trait EatApplePlus {
    fn get_apple(&self) -> usize;
    fn set_apple(&mut self, num: usize);
    fn eat_apple(&mut self) {
        self.set_apple(self.get_apple() + 1);
        println!("{} apple eaten", self.get_apple());
    }
}
