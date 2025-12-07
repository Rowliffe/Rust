fn print_array_elements (array: [i32;3]){
    let [a,b,c] = array;
    println!("First number : {}", a);
    println!("Second number : {}", b);
    println!("Third number : {}", c);
}

fn print_tuple_elements (info: (char,bool)){
    let (letter, is_fun) = info;
    println!("élément 0 : {}", letter);
    println!("élément 1 : {}", is_fun);
}

fn main() {
    let numbers = [10,20,30];
    print_array_elements(numbers);

    let info = ('a', true);
    print_tuple_elements(info);
}