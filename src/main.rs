#![no_std]
#![no_main]

use crossbeam::atomic::AtomicCell;

use ghost_hunter::MainGame;
use pluggable_interrupt_os::HandlerTable;
use pc_keyboard::DecodedKey;

static TICKED: AtomicCell<bool> = AtomicCell::new(false);
static KEY: AtomicCell<Option<DecodedKey>> = AtomicCell::new(None);

fn cpu_loop() -> ! {
    let mut game = MainGame::new();
    loop {
        if let Ok(_) = TICKED.compare_exchange(true, false) {
            ghost_hunter::tick(&mut game);
        }
        
        if let Ok(k) = KEY.fetch_update(|k| if k.is_some() {Some(None)} else {None}) {
            if let Some(k) = k {
                game.key(k);
            }
        }
    }
}

fn tick() {
    TICKED.store(true);
}

fn key(key: DecodedKey) {
    KEY.store(Some(key));
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    HandlerTable::new()
        .keyboard(key)
        .timer(tick)
        .cpu_loop(cpu_loop)
        .start()
}
