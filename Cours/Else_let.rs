fn main() {
    let maybe_number: Option<u32> = Some(21);

    let result = if let Some(x) = maybe_number {
        x * 2
    } else {
        0
    };

    println!("Le résultat est {}", result);
}
