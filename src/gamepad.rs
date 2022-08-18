use crate::wasm4::{BUTTON_1, BUTTON_2, BUTTON_LEFT, BUTTON_RIGHT, BUTTON_UP, BUTTON_DOWN, GAMEPAD1, GAMEPAD2, GAMEPAD3, GAMEPAD4};

pub struct Gamepad {
    previous: [u8; 4],
    this_frame: [u8; 4],
}

#[repr(u8)]
pub enum Button {
    Button1 = BUTTON_1,
    Button2 = BUTTON_2,
    Left = BUTTON_LEFT,
    Right = BUTTON_RIGHT,
    Up = BUTTON_UP,
    Down = BUTTON_DOWN,
}

impl Gamepad {
    pub fn new() -> Self {
        Self {
            previous: [0; 4],
            this_frame: [0; 4],
        }
    }

    pub fn update_state(&mut self) {
        unsafe {
            self.this_frame = [
                *GAMEPAD1 & (*GAMEPAD1 ^ self.previous[0]),
                *GAMEPAD2 & (*GAMEPAD2 ^ self.previous[1]),
                *GAMEPAD3 & (*GAMEPAD3 ^ self.previous[2]),
                *GAMEPAD4 & (*GAMEPAD4 ^ self.previous[3]),
            ];
            self.previous = [*GAMEPAD1, *GAMEPAD2, *GAMEPAD3, *GAMEPAD4];
        }
    }

    pub fn pressed(&self, player_index: u8, button: Button) -> bool {
        self.previous[player_index as usize] & (button as u8) != 0
    }

    pub fn pressed_this_frame(&self, player_index: u8, button: Button) -> bool {
        self.this_frame[player_index as usize] & (button as u8) != 0
    }
}
