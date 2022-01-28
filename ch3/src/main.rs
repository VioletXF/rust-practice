fn main() {
    let a = [1,2,3,4,5];
    println!(
        "{}",
        match a.get(5) {
            Some(num) => num,
            None => {
                println!("out of bounds!");
                &(12&6)
            }
        }
    );

    println!("{}", func());

    let a = 'outer: loop {
        loop{
            break 'outer 1;
        }
    };
    for number in (1..4).rev(){
        println!("{}!", number);
    }
    println!("{}", a);
    

}

fn func() -> u32 {
    9
}



