extern crate sdl2;

pub fn clamp_min(current_value: i32, delta: i32, clamp: i32) -> i32 {
    let result = current_value + delta;

    match result < clamp {
        true => clamp,
        _ => result
    }
}

pub fn clamp_max(current_value: i32, delta: i32, clamp: i32) -> i32 {
    let result = current_value + delta;

    match result > clamp {
        true => clamp,
        _ => result
    }
}

pub fn overlap(rect1: sdl2::rect::Rect, rect2: sdl2::rect::Rect) -> bool {
    rect1.x() < rect2.x() + rect2.width() as i32 &&
    rect1.x() + rect1.width() as i32 > rect2.x() &&
    rect1.y() < rect2.y() + rect2.height() as i32 &&
    rect1.y() + rect1.height() as i32 > rect2.y()
}
