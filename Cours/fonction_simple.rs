fn interproduct (a: i32, b:i32, c:i32) -> i32 {
    return a * b + b * c + c * a;
} 

fn main() {
    println!("résultat : {} ", interproduct(120, 250, 248));
    
}