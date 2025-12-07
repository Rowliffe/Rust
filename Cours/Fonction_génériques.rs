fn pick_i32(cond: bool, left: i32, right: i32) -> i32 {
    if cond {
        left
    } else {
        right
    }
}

fn pick_char(cond: bool, left: char, right: char) -> char {
    if cond {
        left
    } else {
        right
    }
}

fn pick<T>(cond: bool, left: T, right: T) -> T {
    if cond {
        left
    } else {
        right
    }
}

fn main() {
    println!("picked a number: {:?}", pick(true, 222, 333));
    println!("picked a string: {:?}", pick(false, 'L', 'R'));
}
