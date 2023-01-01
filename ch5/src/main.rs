



fn main() {
    let mut rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    dbg!(&rect1);
    println!("{}", rect1.area());

}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&mut self) -> u32 {
        self.width = 1;
        self.width * self.height
    }
}