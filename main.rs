use std::io::{self, Write};
//use rand::Rng;

fn main() {
    //масиви
    println!("Введіть своє імя: "); 

    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Не вдалося прочитати рядок");

    println!("Чудово привіт {}", name);

    print!("Тепер введи свій вік");

    let mut age = String::new();
    
    io::stdin().read_line(&mut age).expect("Не вдалося прочитати рядок");
}
