use std::ffi::CString;

// #[link="c"]
// extern {
//     fn snprintf(str: *mut const c_char, size: size_t, format: *const c_char, ...);
// }

// fn sprintf1(format: &str, value: ???) -> String {
//     const BUF_SIZE: usize = 256;
//     unsafe {
//         let format = CString::new(format).unwrap().into_raw();
//         let buffer = CString::new(&Vec::with_capacity(BUF_SIZE)).unwrap().into_raw();

//         snprintf(format, BUF_SIZE as size_t, buffer, value);

//         let result = CString::from_raw(buffer);
//         from_utf8(result).unwrap().to_string()
//     }
// }

extern "C" {
    fn printf(c: *const i8, ...);
}
#[macro_export(local_inner_macros)]
macro_rules! wrap_printf {
    // trace!(target: "my_target", key1 = 42, key2 = true; "a {} event", "log")
    // trace!(target: "my_target", "a {} event", "log")
    (target: $target:expr, $($arg:tt)+) => (printf(target: $target, $($arg)+));

    // trace!("a {} event", "log")
    ($($arg:tt)+) => (printf( $($arg)+))
}

macro_rules! hey {
    () => {};
}

//https://zhuanlan.zhihu.com/p/104469645
// macro_rules! test1 {
//     //表达式选择器， expr
//     ($name:expr) => {
//         println!("$name = {:?}", $name);
//     };
// }

// //https://www.cnblogs.com/praying/p/14457360.html
// macro_rules! test2 {
//     //表达式选择器， expr
//     ($name:expr,$name2:expr) => {
//         println!("$name = {:?}, $name2 = {:?}", $name, $name2);
//     };
//     ($name:expr) => {
//         println!("$name = {:?}", $name)
//     };
// }

// macro_rules! test3 {
//     //表达式选择器， expr
//    ($($name:expr),*) => {
//         $(println!("$name = {:?}",$name);)*
//     };
// }

macro_rules! test4 {
    ($name:expr) => {
        {
           println!("$name = {}",$name);
           $name
        }

    };
    ($name:expr,$name1:expr) => {
        {
            let hello = format!("{}{}",$name,$name1);
            println!("{}",hello);
            hello
        }
    };
    //表达式选择器， expr
    ($name:expr,$($name1:tt)*)=>{
        {
            let hello = format!("{}{}",$name,test4!($($name1)*));
            println!("{}",hello);
        }
     }
}

macro_rules! test5 {
    //表达式选择器， expr
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

// macro_rules! test6 {
//     ([ $( $element:tt ),* ]) => {
//         println!("{},{},{},{}",...)
//     };
// }

#[inline]
fn syscall_error_code(result: i64) -> i32 {
    if result < 0 && result >= -0x1000 {
        return -result as i32;
    }
    0
}

macro_rules! test7 {
        (target: $target:expr, $($arg:tt)+) =>

            (syscall_no_intercept(target: $target, $($arg)+));

        ($($arg:tt)+) => {
            {
                let result = unsafe{ 
                    (syscall_no_intercept( $($arg)+))
                 };
                let error = syscall_error_code(result);
                if(error != 0) {
                     -error as i64
                }else{
                    result
                }
            }
        }

}
// macro_rules! test8 {
//     // (target: $target:expr, $($arg:tt)+) => (syscall_no_intercept(target: $target, $($arg)+));
//     ($($arg:tt)+) =>  {
//        let x =  (test7!( $($arg)+))
//     }
// }

macro_rules! mixed_rules {
    () => {};
    (trace $name:ident; $($tail:tt)*) => {
        {
            println!(concat!(stringify!($name), " = {:?}"), $name);
            mixed_rules!($($tail)*);
        }
    };
    (trace $name:ident = $init:expr; $($tail:tt)*) => {
        {
            let $name = $init;
            println!(concat!(stringify!($name), " = {:?}"), $name);
            mixed_rules!($($tail)*);
        }
    };
}
#[link(name = "syscall_intercept")]
extern "C" {
    pub fn syscall_no_intercept(
        syscall_number: ::std::os::raw::c_long,
        ...
    ) -> ::std::os::raw::c_long;
}

fn main() {
    // println!("Hello, world!");
    // let x = 32;
    // let y = 14.00;
    // unsafe {
    //     let a = CString::new("hello: x = %d,y = %f\n").unwrap();
    //     printf(a.as_ptr(), x, y);
    //     wrap_printf!(a.as_ptr(), x, y);
    // }
    // hey!();
    // // test1!("good");
    // // test2!("hello");
    // // test2!("hello", "world");
    // // test3!("test3");
    // // test3!("test3", "morning");
    // test4!("test4");
    // test4!("test4", "test4...");
    // test4!("test4", "test4...", "llllll");
    // test4!("test4", "test4...", "llllll");
    // println!("test5,{:?}", test5!("test5", "good", "morning", "hello"));
    // println!("test6,{:?}",test6!(1));
    // println!(
    //     "file {} line {} module_path {}",
    //     file!(),
    //     line!(),
    //     module_path!()
    // );
    let a: i64 = 11;
    let b: i64 = 2;
    let c: i64 = 3;

    let x = test7!(a, b, c);
    let y = test7!(a);
    println!("x = {:?}", x);
    println!("a = {:?}", y);
    // let x = test7!(a);
    // println!("x = {:?}", x);

    // {
    //     let a = 42;
    //     let b = "Ho-dee-oh-di-oh-di-oh!";
    //     let c = (false, 2, 'c');
    //     mixed_rules!(
    //         trace a;
    //         trace b;
    //         trace c;
    //         trace b = "They took her where they put the crazies.";
    //         trace b;
    //     );
    // }

    // println!("show = {:?}",test4!("test4","helllo")) ;
    // println!("show = {:?}",test4!("test4","helllo","world")) ;
}
