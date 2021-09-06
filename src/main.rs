use ferris_says::say;
use std::io::{ stdout, BufWriter, stdin };
use std::cmp::Ordering;
use rand::Rng;

fn main() 
{
    ferris_says();
    guessing_game();
}

fn ferris_says()
{
    let mut name = String::new();

    println!("Enter the player's name :");

    stdin()
    .read_line(&mut name)
    .expect("Erreur");

    let mut out = String::from("Salut ");

    out.push_str(&name.to_string());
    
    let width = 24;

    let mut writer = BufWriter::new(stdout());

    say(out.as_bytes(), width, &mut writer).unwrap();
}

fn guessing_game()
{
    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("{}", secret_number);
    println!("Guess the number\nEnter your number");

    let mut guess = String::new();
    
    stdin()
    .read_line(&mut guess)
    .expect("Erreur");

    let guess : u32 = guess.trim().parse().expect("Please type a number !");

    println!("{}", guess);

    match guess.cmp(&secret_number) 
    {
        Ordering::Less => println!("Too small"),
        Ordering::Equal => println!("You win !"),
        Ordering::Greater => println!("Too big")
    }
}