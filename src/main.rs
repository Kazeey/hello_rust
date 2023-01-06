mod lib
{
    pub mod screen;
    pub mod button;
}

mod examples 
{
    pub mod ferris_says;
    pub mod boucle;
    pub mod guessing_game;
}

use lib::button;
use lib::screen;


fn main() 
{
    screen::Draw::draw(&button::Button{width: 10, height: 10, label: String::from("Button")});
    examples::guessing_game::guessing_game();
}
