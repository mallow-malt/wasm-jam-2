use crate::{game::{Game, GameState}, wasm4::{text, NETPLAY}, gamepad::Button};

pub fn main_menu(game: &mut Game) {
    text("Game Title", 40,10);
    text("Press X to Start", 20,50);
    if game.gamepad.pressed_this_frame(0, Button::Button1) {
        game.state = GameState::JoinMenu
    }
}

pub fn join_menu(game: &mut Game) {
    if unsafe { *NETPLAY & 0b100 == 0b100 } { // If netplay is active
        let player_index = unsafe { *NETPLAY & 0b011 };
        render_screen(game, player_index);
        // Enable start game for host if at least one other active player
        if game.active_players.iter().skip(1).any(|x| *x) && game.gamepad.pressed_this_frame(0, Button::Button1) {
            game.state = GameState::GameStart
        }
        game.run(&check_join_inputs, false, false);
    } else {
        text("Use return to",0,0);
        text("activate Netplay and ",0,10);
        text("share the link",0,20);
        text("with up to 3 people.", 0, 30);
    }
}

fn render_screen(game: &mut Game, player_index: u8) {
    if player_index == 0 {
        game.run(&print_player_status, false, false);

        if game.active_players.iter().skip(1).any(|x| *x) {
            text("Press X to start.", 0, 40);
        }
    } else {
        text("You are ".to_owned() + &format!("Player {}", player_index + 1), 0,0);
        if game.active_players[player_index as usize] {
            text("Waiting for host", 0, 10);
        } else {
            text("Press X to ready up", 0, 10);
        }
    }
}

fn check_join_inputs(game: &mut Game, player_index: u8) {
    if game.gamepad.pressed_this_frame(player_index, Button::Button1) {
        game.active_players[player_index as usize] = true
    }
}

fn print_player_status(game: &mut Game, player_index: u8) {
    text(format!("Player {}", player_index + 1) + if game.active_players[player_index as usize] {" Active"} else {" Inactive"}, 0, i32::from((player_index - 1) * 10));
}
