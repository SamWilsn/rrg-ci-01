struct Bar(i32);

impl Bar {
    fn as_i32(self) -> i32 {
        self.0
    }
}

fn main() {
    let bar = Bar(3).as_i32();
    println!("Hello, world! {}", bar);
}
