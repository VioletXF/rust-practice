use std::mem::take;


fn main() {
    let s = String::from("asdf");

    takes_ownership(s);
    println!("{}", s);

}

fn takes_ownership(s: String){

}