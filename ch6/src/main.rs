#[derive(Debug)]
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
    
}
#[derive(Debug)]
struct three(i32,i32,i32);
fn main() {
    println!("Hello, world!");
    let m = Message::ChangeColor(1,2,3);
    dbg!(three(1,2,3));
    dbg!(m);
    
}
