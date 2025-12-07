fn main() {
    let x: Option<i32> = None;

    if let Some(value) = x {
        println!("x contient : {}", value);
    } else {
        println!("x n'a pas de valeur");
    }
}
