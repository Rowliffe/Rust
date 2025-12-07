fn main() {
    let a: char = 'A';
    let b: char = 'B';

    let mut r: &char = &a;
    dbg!(*r);

    r = &b;
    dbg!(*r);
}