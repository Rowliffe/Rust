// Struct classique : les attributs sont nommés
struct PointA {
    x: i32,
    y: i32,
}

// Tuple Struct : les attributs n'ont que leur type
struct PointB(i32, i32);

struct Newtons(i32);

fn print_force(force: Newtons) {
    println!("Force: {}", force.0);
}

fn main() {
    // Struct classique
    let p0 = PointA { x: 17, y: 23 };
    println!("PointA: ({}, {})", p0.x, p0.y);

    // Tuple struct
    let p1 = PointB(17, 23);
    println!("PointB: ({}, {})", p1.0, p1.1);

    // Tuple classique, dit anonyme
    let p2: (i32, i32) = (17, 23);
    println!("PointC: ({}, {})", p2.0, p2.1);

    let force = Newtons(12);
    print_force(force);
}


