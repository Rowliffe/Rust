fn main () {
    let mut c: char = 'a';
    let r = &mut c;

    *r = 'z';
    println!("La nouvelle lettre est : {}",c);
}