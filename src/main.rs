struct Bar(i32);

impl Bar {
    fn as_i32(self) -> i32 {
        self
        .0
    }
}

fn main() {
    println!("Hello, world!");
}
