mod lib
{
    pub mod Screen;
    pub mod Button;
}

use lib::Button;
use lib::Screen;

fn main() 
{
    Screen::Draw::draw(&Button::Button{width: 10, height: 10, label: String::from("Button")});
}
