fn boucle()
{
    let a = [10, 20, 30, 40, 50];

    for element in a.iter()
    {
        println!("{}", element);
    }

    for number in (1..4).rev()
    {
        println!("{}", number);
    }
}