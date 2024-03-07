use std::mem;


// Copy types
// https://dhghomon.github.io/easy_rust/Chapter_19.html


#[derive(Debug,Clone)]
struct Foo {
    a: u8,
    b: u32,
    c: bool,
}

impl Drop for Foo {
    fn drop(&mut self) {
        mem::forget(self.a);
        mem::forget(self.b);
        // mem::forget(self.c);
        println!("drop Foo");
    }
}

impl Foo {
    fn print_anything(&self) {
        println!("out of print anything => Hello from here ");
    }
}

fn main() {
    // test constructors
    let foo = Foo {
        a: 0,
        b: 1,
        c: false,
    };
    println!("foo => {:#?} {:#?} {:#?}", foo.a, foo.b, foo.c);
    foo.print_anything();
    drop(foo);
}
