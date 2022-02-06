// modules_demo/main.rs

mod foo;
use foo::Bar;

fn main() {
  foo::do_foo();
  Bar::hello();
}
// Hi from foo!
// Hello from Bar!
