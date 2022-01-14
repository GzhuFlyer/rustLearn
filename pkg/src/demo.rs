//Rust 中有两种简单的访问权：公共（public）和私有（private）。

//默认情况下，如果不加修饰符，模块中的成员访问权将是私有的。
/*
nation
 ├── government
 │ └── govern
 ├── congress
 │ └── legislate
 └── court
   └── judicial
*/
mod nation {
    pub mod government {
        pub fn govern() {}
    }

    mod congress {
        pub fn legislate() {}
    }
   
    mod court {
        fn judicial() {
            super::congress::legislate();
        }
    }
}

fn main() {
    nation::government::govern();
}

