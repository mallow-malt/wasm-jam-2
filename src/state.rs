pub static mut GAME_STATE: u8 = 0;
pub static mut ACTIVE_PLAYERS: [bool; 4] = [true, false, false, false];

pub unsafe fn run_for_non_host<F>(mut fun: F) 
    where F: FnMut(u8)
{
    for (i, x) in ACTIVE_PLAYERS.iter().skip(1).enumerate() {
        if *x {
            fun(i as u8);
        }
    }
}
