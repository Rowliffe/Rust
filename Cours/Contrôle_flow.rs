enum Option<T> {
    Some(T),   // Il y a une valeur
    None,      // Il n'y a pas de valeur
}

fn main() {
    let x: Option<i32> = Some(42);  // x a une valeur
    let y: Option<i32> = None;      // y n'a pas de valeur
}
