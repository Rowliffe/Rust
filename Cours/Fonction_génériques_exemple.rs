// i32 => nombre entier signé : 1, 7, -45, -12
fn add_i32(a: i32, b: i32) -> i32 {
    return a + b;
}

// f32 => nombre décimal signé : 2.3, 6.8, -9.23
fn add_f32(a: f32, b: f32) -> f32 {
    return a + b;
}

fn main() {
    add_i32(23, 31);
    add_f32(5.12, 6.87);
}

// ================================

fn add<T>(a: T, b: T) -> T {
    return a + b;
}

fn main() {
    add(23, 31);       // En voyant les paramètres d’entrée il va deviner que T doit être remplacé par i32
    add(5.12, 6.87);   // En voyant les paramètres d’entrée il va deviner que T doit être remplacé par f32
}
