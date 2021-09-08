use ferris_says::say;
use std::io::{ stdout, BufWriter, stdin };
use std::cmp::Ordering;
use rand::Rng;

fn main() 
{
    ferris_says();
    guessing_game();
}

/// Give a name to ferris.
///
/// So Ferris is a crate given in the book's exemple.
/// The function needs a word to work.
/// 
/// # Variables' creations
///
/// The name variable is just gonna store a new created `String`, it has to be mutable for the prompt : 
/// ```
/// let mut name = String::new();
/// ```
/// The width of the text :
/// ```
/// let width = 24;
/// ```
/// And the buffer :
/// ```
/// let mut writer = BufWriter::new(stdout());
/// ```
/// 
/// # User's input
/// 
/// To read the user's input the program is gonna use stdin(), like this
/// ```
/// stdin()
/// .read_line(&mut name)
/// .expect("Erreur");
/// ```
/// 
/// # Ferris_says
/// 
/// Then the program is gonna say "Salut + the name" with : 
/// ```
/// say(out.as_bytes(), width, &mut writer).unwrap();
/// ```
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

/// "I count gates and numbers, then play the guessing game"
/// 
/// The function will creates a random number using `rand`, then you'll have to guess the number.
/// 
/// # Variables' creation
/// 
/// The `=` in the `gen_range` means that the external born is include and the limit :
/// ```
/// let secret_number = rand::thread_rng().gen_range(1..=100);
/// ```
/// 
/// The guessed number will be stored in a String, then we're gonna use stdin to ask it to the user and trim it to get a number : 
/// ```
/// let mut guess = String::new();
/// 
/// stdin()
/// .read_line(&mut guess)
/// .expect("Erreur");
///
/// let guess : u32 = guess.trim().parse().expect("Please type a number !");
/// ```
/// 
/// # The guess
/// 
/// Then since we got the 2 numbers, we now have to say to the user if he is correct or not : 
/// So we're gonna use `match guess.cmp(&parameter) {}`, to match the `guess` with the parameter
/// 
/// ```
/// match guess.cmp(&secret_number) 
/// {
///     Ordering::Less => println!("Too small"),
///     Ordering::Equal => println!("You win !"),
///     Ordering::Greater => println!("Too big")
/// }
/// ```
fn guessing_game()
{
    let secret_number = rand::thread_rng().gen_range(1..=100);

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