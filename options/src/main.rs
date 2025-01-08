fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_space: u8) {
    println!("{num_space}");
}

fn main() {
    let five = Some(5);
    let _six = plus_one(five);
    let _none = plus_one(None);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    let new_roll = 10;
    match new_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (), // все оставшиеся значения игнорируем
    }
}
