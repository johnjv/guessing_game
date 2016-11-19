use std::io;

fn main() {
    println!("A GUESSING GAME");
    println!("please input whatever:");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read input");
    
    println!("your input: {}", guess);
}
