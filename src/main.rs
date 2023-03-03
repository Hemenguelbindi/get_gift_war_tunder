use inputbot::{MouseButton::*, KeybdKey::*, MouseCursor};

use std::thread::sleep;
use std::time::Duration;

fn open_war_thunder() {
    // Клик на координатах (1121, 699)
    LeftButton.bind(||{
        while LeftButton.is_pressed() {
            MouseCursor::move_rel(1121, 699);
            LeftButton.press();
        }
    });
    sleep(Duration::from_millis(100));
    // Клик на координатах (1155, 580)
    LeftButton.bind(||{
        while LeftButton.is_pressed() {
            MouseCursor::move_rel(1155, 580);
            LeftButton.press();
        }
    });
    sleep(Duration::from_millis(100));
    // Клик на координатах (283, 46)
    LeftButton.bind(||{
        while LeftButton.is_pressed() {
            MouseCursor::move_rel(283, 46);
            LeftButton.press();
        }
    });
    sleep(Duration::from_millis(100));
    // Клик на координатах (250, 83)
    LeftButton.bind(||{
        while LeftButton.is_pressed() {
            MouseCursor::move_rel(283, 46);
            LeftButton.press();
        }
    });
    sleep(Duration::from_millis(100));
    // Клик на координатах (174, 283)
    LeftButton.bind(||{
        while LeftButton.is_pressed() {
            MouseCursor::move_rel(174, 283);
            LeftButton.press();
        }
    });
    sleep(Duration::from_millis(100));
    // Клик на координатах (535, 413)
    LeftButton.bind(||{
        while LeftButton.is_pressed() {
            MouseCursor::move_rel(535, 413);
            LeftButton.press();
        }
    });
    sleep(Duration::from_millis(100));
    // Нажатие клавиши Enter
    EnterKey.bind(||EnterKey.press());
}

fn get_gif() {
    // клик в координатах 670, 330
    LeftButton.bind(||{
        while LeftButton.is_pressed() {
            MouseCursor::move_rel(670, 330);
            LeftButton.press();
        }
    });
    sleep(Duration::from_millis(100));
    EnterKey.bind(||EnterKey.press());
}

fn main() {
    open_war_thunder();
    sleep(Duration::from_secs(120));
    get_gif();
}
