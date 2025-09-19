use iced::widget::{button, column, row, text, Column, Row};

#[derive(Default)]
struct Counter {
    value: i32,
}
#[derive(Debug, Clone, Copy)]
pub enum Message {
    Increment,
    Decrement,
}
mod elevator;

use elevator::elevator::Elevator;
use elevator::states;
use iced::Element;

impl Counter {
    pub fn view(&self) -> Element<Message> {
        // We use a column: a simple vertical layout
        column![
            // The increment button. We tell it to produce an
            // `Increment` message when pressed
            button("+").on_press(Message::Increment),
            // We show the value of the counter here
            text(self.value).size(50),
            // The decrement button. We tell it to produce a
            // `Decrement` message when pressed
            button("-").on_press(Message::Decrement),
        ]
        .into()
    }
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
        }
    }
}
fn main() -> iced::Result {
    let mut elevator = Elevator::new(10);
    elevator.go_to_floor(5);
    println!("Elevator is now on floor {}", elevator.status());
    iced::run("A cool counter", Counter::update, Counter::view)
}
