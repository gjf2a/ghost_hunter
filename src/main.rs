#![no_std]
#![no_main]

use lazy_static::lazy_static;
use spin::Mutex;
use ghost_hunter_core::GhostHunterGame;
use ghost_hunter::MainGame;
use pluggable_interrupt_os::HandlerTable;

use pc_keyboard::DecodedKey;

lazy_static! {
    static ref GAME: Mutex<MainGame> = Mutex::new(GhostHunterGame::new());
}

fn tick() {
    ghost_hunter::tick(&mut GAME.lock());
}

fn key(key: DecodedKey) {
    GAME.lock().key(key);
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    HandlerTable::new()
        .keyboard(key)
        .timer(tick)
        .start()
}
