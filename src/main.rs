use iced::{
    widget::{button, column, row, text, Row, Space}, Length, Rectangle, Theme,
    mouse, Point, Size,
    Element, Subscription, Renderer, Color,
};
use iced::widget::canvas::{self, Canvas, Frame, Path, Stroke, Geometry};

use std::time::{Duration, Instant};

use elevator::elevator::Elevator;


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
    Frame,
}
mod elevator;


const TOTAL_FLOORS: u8 = 8;
const FLOOR_HEIGHT: f32 = 60.0;
const CAR_SIZE: Size = Size { width: 40.0, height: 40.0 };
const CAR_SPEED: f32 = 220.0;
struct App {
    elevator: Elevator,
    max_floor: u8,
    car_pos_px: f32,      
    last_tick: Instant,
}

#[derive(Debug, Default)]
struct ElevatorShaft {
    floors: u8,
    floor_h: f32,
    car_pos_px_from_bottom: f32,
}

impl<Message> canvas::Program<Message> for ElevatorShaft {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());

        // shaft dimensions — centered horizontally
        let shaft_width = 100.0;
        let shaft_height = self.floor_h * self.floors as f32;
        let shaft_x = (bounds.width - shaft_width) / 2.0;
        let shaft_y = (bounds.height - shaft_height) / 2.0;

        // shaft outline
        let shaft_rect = Path::rectangle(
            Point::new(shaft_x, shaft_y),
            Size::new(shaft_width, shaft_height),
        );
        frame.stroke(&shaft_rect, Stroke::default());

        // floor lines
        for i in 0..=self.floors+1 {
            let y = shaft_y + shaft_height - (i as f32) * self.floor_h;
            let line = Path::line(
                Point::new(shaft_x, y),
                Point::new(shaft_x + shaft_width, y),
            );
            frame.stroke(&line, Stroke::default());
        }

        // elevator car
        let car_center_from_bottom = self.car_pos_px_from_bottom;
        let car_bottom_y = shaft_y + shaft_height - car_center_from_bottom;
        let car_top_left = Point::new(
            shaft_x + (shaft_width - 40.0) / 2.0,
            car_bottom_y - 40.0,                  
        );

        let car = Path::rectangle(car_top_left, CAR_SIZE);
        frame.fill(&car, Color::from_rgb(0.1, 0.9, 0.5));

        vec![frame.into_geometry()]
    }
}


fn approach(current: f32, target: f32, max_delta: f32) -> f32 {
    let diff = target - current;
    if diff.abs() <= max_delta { target } else { current + diff.signum() * max_delta }
}

impl App {
    fn new() -> Self {
    let max_floor = TOTAL_FLOORS;
        let mut app =Self {
            elevator: Elevator::new(max_floor),
            max_floor,
            car_pos_px: 0.0,
            last_tick: Instant::now(),
        };
        app.elevator.enable_audio_from_files(
            "sounds/jazz-lounge-elevator-music-332339.mp3",
            "sounds/elevator-ding-at-arenco-tower-dubai-38520.mp3",
        );
        app
    }

    // simulate a timer tick every second
    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([
            iced::time::every(Duration::from_secs(1)).map(|_| Message::Tick),
            iced::time::every(Duration::from_millis(16)).map(|_| Message::Frame),
        ])
    }
    // Iced calls this whenever a Message is produced
    fn update(&mut self, message: Message) {
        match message {
            Message::RequestFloor(f) => self.elevator.go_to_floor(f),
            Message::OpenDoor => self.elevator.open_door(),
            Message::CloseDoor => self.elevator.close_door(),
            Message::Tick => {
                self.elevator.tick();
            }
            Message::Frame => {
                let now = Instant::now();
                let dt = now.duration_since(self.last_tick).as_secs_f32();
                self.last_tick = now;

                let target_px = (self.elevator.current_floor() as f32) * FLOOR_HEIGHT;
                self.car_pos_px = approach(self.car_pos_px, target_px, CAR_SPEED * dt);
            }
        }
    }

    // builds the ui based on current state
    fn view(&self) -> Element<Message> {
        // floor selection buttons 0..=max_floor
        let mut floors: Row<Message> = row![];
        for f in 0..=self.max_floor {
            floors = floors.push(button(text(f.to_string())).on_press(Message::RequestFloor(f)));
        }

        let floors_centered = row![Space::with_width(Length::Fill),floors.spacing(5),Space::with_width(Length::Fill)];

        // controls
        let controls: Row<Message> = row![
            Space::with_width(Length::Fill),
            button("Tick").on_press(Message::Tick),
            button("Open").on_press(Message::OpenDoor),
            button("Close").on_press(Message::CloseDoor),
            Space::with_width(Length::Fill),
        ].spacing(10);

        // status
        let status: Element<Message> = text(self.elevator.status()).center().into();

        let shaft_widget = Canvas::new(ElevatorShaft {
                floors: TOTAL_FLOORS+1,
                floor_h: FLOOR_HEIGHT,
                car_pos_px_from_bottom: self.car_pos_px,
            })
            .width(Length::Fixed(160.0))
            .height(Length::Fixed(FLOOR_HEIGHT * (TOTAL_FLOORS + 1) as f32 + 20.0));

        let shaft_centered = row![Space::with_width(Length::Fill), shaft_widget, Space::with_width(Length::Fill)];

        column![
            text("Elevator Simulator").size(24),
            status,
            shaft_centered,
            floors_centered,
            controls,
        ]
        .spacing(12).into()
    }
}

fn main() -> iced::Result {
    // run the application
    iced::application("Elevator", App::update, App::view)
        .subscription(App::subscription)
        .centered()
        .run_with(|| (App::new(), iced::Task::none()))
}

