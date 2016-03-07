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
