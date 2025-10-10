pub mod states {

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum ElevatorState {
        Idle,
        Moving,
        DoorOpen,
    }
}

pub mod elevator {
    use super::states::ElevatorState;
    use std::collections::VecDeque;
    #[derive(Debug)]
    pub struct Elevator {
        current_floor: u8,
        max_floor: u8,
        door_open: bool,
        target_floor: VecDeque<u8>,
        state: ElevatorState,
        timer: u8,
    }

    impl Elevator { 
        pub fn new(max_floor: u8) -> Self {
            Self {
                current_floor: 0,
                max_floor,
                door_open: false,
                target_floor: VecDeque::new(),
                state: ElevatorState::Idle,
                timer: 0,
            }
        }

        pub fn current_floor(&self) -> u8 {
            self.current_floor
        }

        pub fn state(&self) -> ElevatorState {
            self.state
        }

        pub fn go_to_floor(&mut self, floor: u8) {
            if self.target_floor.contains(&floor) {
                return;
            }
            match self.state {
                ElevatorState::Idle => {
                    if floor <= self.max_floor && floor != self.current_floor {
                        self.target_floor.push_back(floor);
                        self.state = ElevatorState::Moving;
                    }
                }
                ElevatorState::Moving => {
                    if floor <= self.max_floor {
                        self.target_floor.push_back(floor);
                    }
                }
                ElevatorState::DoorOpen => {
                    if floor <= self.max_floor {
                        self.target_floor.push_back(floor);
                    }
                }
            }
        }

        pub fn close_door(&mut self) {
            if self.state == ElevatorState::DoorOpen {
                self.timer = 0;
                self.door_open = false;
                self.state = ElevatorState::Idle;
            }
        }

        pub fn open_door(&mut self) {
            if self.state == ElevatorState::Idle {
                self.door_open = true;
                self.state = ElevatorState::DoorOpen;
                self.timer = 10; // door stays open for 10 ticks
            }
        }

        pub fn tick(&mut self) {
            // if we are idle, check if there are target floors to go to
            match self.state {
                ElevatorState::Idle => {
                    if let Some(&next_floor) = self.target_floor.front() {
                        if next_floor != self.current_floor {
                            self.state = ElevatorState::Moving;
                        }
                    }
                }
                // if moving, move one floor towards the target if we reach target, open door
                ElevatorState::Moving => {
                    if let Some(&next_floor) = self.target_floor.front() {
                        if self.current_floor < next_floor {
                            self.current_floor += 1;
                        } else if self.current_floor > next_floor {
                            self.current_floor -= 1;
                        }

                        if self.current_floor == next_floor {
                            self.target_floor.pop_front();
                            self.state = ElevatorState::DoorOpen;
                            self.door_open = true;
                            self.timer = 10; // door stays open for 10 ticks
                        }
                    } else {
                        self.state = ElevatorState::Idle;
                    }
                }
                // if door is opened, count down the timer
                ElevatorState::DoorOpen => {
                    if self.timer > 0 {
                        self.timer -= 1;
                    } else {
                        self.door_open = false;
                        self.state = ElevatorState::Idle;
                    }
                }
            }
        }
        pub fn status(&self) -> String {
            format!(
                "Floor: {}, State: {:?}, Door Open: {}, Targets: {:?}, Timer: {}",
                self.current_floor, self.state, self.door_open, self.target_floor, self.timer
            )
        }
    }
}
