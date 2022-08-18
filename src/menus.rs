use crate::*;

pub unsafe fn main_menu() {
    text("Game Title", 40,10);
    text("Press X to Start", 20,50);
    if pressed_this_frame(0) & BUTTON_1 != 0 {
        GAME_STATE = 1;
    }
}

pub unsafe fn join_menu() {
    if *NETPLAY & 0b100 == 0b100 { // If netplay is active
        let player_index = *NETPLAY & 0b011;
        match player_index {
            0 => host_screen(),
            _ => join_screen(player_index)
        }
        // Enable start game for host if at least one other active player
        if ACTIVE_PLAYERS.iter().skip(1).any(|x| *x) && pressed_this_frame(0) & BUTTON_1 != 0 {
            GAME_STATE = 2;
        }
        run_for_non_host(&check_join_inputs)
    } else {
        text("Use return to",0,0);
        text("activate Netplay and ",0,10);
        text("share the link",0,20);
        text("with up to 3 people.", 0, 30);
    }
}

unsafe fn host_screen() {
    print_player_status(1);
    print_player_status(2);
    print_player_status(3);
    if ACTIVE_PLAYERS.iter().skip(1).any(|x| *x) {
        text("Press X to start.", 0, 40);
    }
}

unsafe fn print_player_status(player_index: u8) {
    text(format!("Player {}", player_index + 1) + if ACTIVE_PLAYERS[player_index as usize] {" Active"} else {" Inactive"}, 0, i32::from((player_index - 1) * 10));
}

unsafe fn join_screen(player_index: u8) {
    text("You are ".to_owned() + &format!("Player {}", player_index + 1), 0,0);
    if ACTIVE_PLAYERS[player_index as usize] {
        text("Waiting for host", 0, 10);
    } else {
        text("Press X to ready up", 0, 10);
    }
}

fn check_join_inputs(player_index: u8) {
    unsafe {if pressed_this_frame(player_index) & BUTTON_1 != 0 {
        ACTIVE_PLAYERS[player_index as usize] = true
    }}
}