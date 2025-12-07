fn main() {
   let a = 10;
   println!("before: {a}");
   {
       let a = "Hello";
       println!("inner scope: {a}");
       
       let a = true;
       println!("shadowed in inner scope: {a}");
   }
    println!("after: {a}");
}


