fn main() {

    //boucle tant que
    let mut x = 200;
    while x>10 {
        x = x / 2;
    }
    println!("Final x: {x}");
    
    // boucle for
    for x in 1..5 {
        println!("x: {x}");
    }
    
    for elem in [1,2,3,4,5] {
        println!("elem : {elem}");
    }
}


