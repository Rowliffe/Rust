fn collatz_length(mut n: i32) -> u32 {
    let mut length = 1;
    while n!= 1{
        if n % 2 == 0 {
            n = n / 2;
            println!("est pair : {n}");
        }else {
            n = n * 3 + 1;
            println!("est impair : {n}");
        }
        length += 1;
    }
    return length;
        
}

/*c#[test]
fn test_collatz_length() {
    assert_eq!(collatz_length(6), 8);
    assert_eq!(collatz_length(19), 20);
    assert_eq!(collatz_length(27), 111);
}
*/
fn main() {
    let nombre = 6;
    println!("La suite de Collatz pour {} atteint 1 en {} étapes.", nombre, collatz_length(nombre));
}


