use ferris_says::say;
use std::io::{ stdout, BufWriter, stdin };

fn main() 
{
    ferris_says("Maxime");
    guessing_game();
}

fn ferris_says(name: &str)
{
    let mut out = String::from("Salut");
    out.push_str(name);
    let width = 24;

    let mut writer = BufWriter::new(stdout());

    say(out.as_bytes(), width, &mut writer).unwrap();
}

fn guessing_game()
{
    println!("Guess the number\nEnter your number");

    let mut guess = String::new();

    stdin()
    .read_line(&mut guess)
    .expect("Erreur");

    println!("{}", guess);
}