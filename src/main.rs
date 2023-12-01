mod letters;
mod sound;

use iced::{Application, Command, Element, Length, Renderer, Settings};
use iced::widget::{Column, Row, Text, Button};

fn random_letter() {
    println!("Hello, world!");
    let letter: letters::Letter = rand::random();
    println!("Random letter: {:?}", letter);
    sound::play_sound(&format!("{:?}.mp3", letter));
}

pub fn main() -> iced::Result {
    Hello::run(Settings::default())
}

struct Hello {
    addressee: String,
}
 
#[derive(Clone, Debug)]
enum HelloMessage {
    TextBoxChange(String),
    ButtonPressed
}

impl iced::Application for Hello {
    type Executor = iced::executor::Default;
    type Flags = ();
    type Message = HelloMessage;
    type Theme = iced::Theme;

    fn new(_flags: ()) -> (Hello, Command<Self::Message>) {
        ( Hello { addressee: String::from("world"), }, Command::none() )
    }

    fn title(&self) -> String {
        String::from("Dual-N-Back")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            HelloMessage::TextBoxChange(string) => {
                self.addressee = string;
                Command::none()
            }
            HelloMessage::ButtonPressed => {
                random_letter();
                Command::none() 
            }
        }
    }
        
    fn view(&self) -> Element<Self::Message> {
        let button = Button::new("Random Letter").on_press(HelloMessage::ButtonPressed);
        let row1 = Row::new().push(button);
        return Column::new().push(row1).into();
    }
}