


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
/// let guess : u32 = match guess.trim().parse() {
///     Ok(num) => num,
///     Err(_) => continue
/// };
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
///    Ordering::Less => println!("Too small"),
///    Ordering::Equal => {
///        println!("You win !");
///        break;
///    },
///    Ordering::Greater => println!("Too big")
/// }
/// ```
fn guessing_game()
{
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is : {}", secret_number);

    loop 
    {
        println!("Guess the number");

        let mut guess = String::new();
        
        stdin()
        .read_line(&mut guess)
        .expect("Erreur");

        let guess : u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
    
        match guess.cmp(&secret_number) 
        {
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("You win !");
                break;
            },
            Ordering::Greater => println!("Too big")
        }
    }
}