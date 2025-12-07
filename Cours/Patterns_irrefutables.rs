fn takes_tuple(tuple: (char, i32, bool)) {
    let a = tuple.0;
    let b = tuple.1;
    let c = tuple.2;

    // This does the same thing as above.
    let (a, b, c) = tuple;

    // Ignore the first element, only bind the second and third.
    let (_, b, c) = tuple;

    // Ignore everything but the last element.
    let(.., c) = tuple;
}

fn takes_array(array: [u8; 5]) {
    let [first,..,last] = array;
}

fn main() {
    takes_tuple(('a', 777, true));
}
