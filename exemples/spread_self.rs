#[derive(Debug)]
struct Foo {
    test: i32,
    test_2: i32,
}

impl Foo {
    pub fn new() -> Foo {
        Foo {
            test: 0,
            test_2: 300,
        }
    }

    pub fn set_test_100(&mut self) -> &Self {
        self.test = 100;
        self
    }
}

pub fn main() {
    let mut foo = Foo::new();
    println!("{:?}", foo);
    let bar = foo.set_test_100();
    println!("{:?}", bar);
    println!("{:?}", foo);
}
