//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

use wasm_game_of_life::Universe;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
pub fn test_tick() {
    // Fail if all universes are equal
    assert_ne!(glider_frame_0(), glider_frame_1());

    let mut test_universe = glider_frame_0();
    test_universe.tick();
    assert_eq!(test_universe, glider_frame_1());
    test_universe.tick();
    assert_eq!(test_universe, glider_frame_2());
    test_universe.tick();
    assert_eq!(test_universe, glider_frame_3());
    test_universe.tick();
    assert_eq!(test_universe, glider_frame_4());
}

#[cfg(test)]
pub fn glider_frame_0() -> Universe {
    Universe::from_str(
        "
        ◻◻◻◻◻◻
        ◻◻◼◻◻◻
        ◼◻◼◻◻◻
        ◻◼◼◻◻◻
        ◻◻◻◻◻◻
        ◻◻◻◻◻◻
        ",
    )
}

#[cfg(test)]
pub fn glider_frame_1() -> Universe {
    Universe::from_str(
        "
        ◻◻◻◻◻◻
        ◻◼◻◻◻◻
        ◻◻◼◼◻◻
        ◻◼◼◻◻◻
        ◻◻◻◻◻◻
        ◻◻◻◻◻◻
        ",
    )
}

#[cfg(test)]
pub fn glider_frame_2() -> Universe {
    Universe::from_str(
        "
        ◻◻◻◻◻◻
        ◻◻◼◻◻◻
        ◻◻◻◼◻◻
        ◻◼◼◼◻◻
        ◻◻◻◻◻◻
        ◻◻◻◻◻◻
        ",
    )
}

#[cfg(test)]
pub fn glider_frame_3() -> Universe {
    Universe::from_str(
        "
        ◻◻◻◻◻◻
        ◻◻◻◻◻◻
        ◻◼◻◼◻◻
        ◻◻◼◼◻◻
        ◻◻◼◻◻◻
        ◻◻◻◻◻◻
        ",
    )
}

#[cfg(test)]
pub fn glider_frame_4() -> Universe {
    Universe::from_str(
        "
        ◻◻◻◻◻◻
        ◻◻◻◻◻◻
        ◻◻◻◼◻◻
        ◻◼◻◼◻◻
        ◻◻◼◼◻◻
        ◻◻◻◻◻◻
        ",
    )
}
