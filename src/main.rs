#![feature(ptr_internals)]

mod ffi;
mod foo;

use foo::Foo;
fn main() {
    let mut foo = Foo::new().unwrap();
    foo.set_a(1);
    println!("{}", foo.get_a());
    foo.set_a(4);

    println!("{}", foo.get_a());
    println!("main done");
}
