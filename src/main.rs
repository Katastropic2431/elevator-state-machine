use iced::widget::{button, column, row, text, Column, Row};
use std::thread::sleep;
use std::time::Duration;

#[derive(Default)]
struct Counter {
    value: i32,
}
#[derive(Debug, Clone, Copy)]
pub enum Message {
    RequestFloor(u8),
    Tick,
    OpenDoor,
    CloseDoor,
}
mod elevator;

use elevator::elevator::Elevator;
use iced::Subscription;
use iced::{time, Element};
struct App {
    elevator: Elevator,
    max_floor: u8,
}

impl App {
    fn new() -> Self {
        let max_floor = 10;
        Self {
            elevator: Elevator::new(max_floor),
            max_floor,
        }
    }

    // simulate a timer tick every second
    fn subscription(&self) -> Subscription<Message> {
        iced::time::every(Duration::from_millis(1000)).map(|_| Message::Tick)
    }
    // Iced calls this whenever a Message is produced
    fn update(&mut self, message: Message) {
        match message {
            Message::RequestFloor(f) => self.elevator.go_to_floor(f),
            Message::Tick => self.elevator.tick(),
            Message::OpenDoor => self.elevator.open_door(),
            Message::CloseDoor => self.elevator.close_door(),
        }
    }

    // builds the ui based on current state
    fn view(&self) -> Element<Message> {
        // floor selection buttons 0..=max_floor
        let mut floors: Row<Message> = row![];
        for f in 0..=self.max_floor {
            floors = floors.push(button(text(f.to_string())).on_press(Message::RequestFloor(f)));
        }

        // controls
        let controls: Row<Message> = row![
            button("Tick").on_press(Message::Tick),
            button("Open").on_press(Message::OpenDoor),
            button("Close").on_press(Message::CloseDoor),
        ];

        // status
        let status = text(self.elevator.status()).center();

        column![
            text("Elevator Simulator").size(24),
            status,
            floors,
            controls,
        ]
        .into()
    }
}

pub fn main() -> iced::Result {
    // run the application
    iced::application("Elevator", App::update, App::view)
        .subscription(App::subscription)
        .centered()
        .run_with(|| (App::new(), iced::Task::none()))
}
