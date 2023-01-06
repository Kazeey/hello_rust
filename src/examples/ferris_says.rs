use std::{io::stdin, io::stdout, io::BufWriter};
use ferris_says::say;

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
/// say(message, width, &mut writer).unwrap();
/// ```
unsafe fn ferris_says()
{
    let mut name = String::new();

    println!("Enter the player's name :");

    stdin()
    .read_line(&mut name)
    .expect("Erreur");

    let out = "Salut ";

    let message = out.to_string() + &name.to_string();
    
    let width = 24;

    let mut writer = BufWriter::new(stdout());

    say(message.as_str(), width, &mut writer).unwrap();
}