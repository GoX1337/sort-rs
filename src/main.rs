mod array;
mod sort;

fn main() {
    let mut values = array::build_random_array(255, 255);
    println!("Before sort: {:?}", values);
    let last = values.len() - 1;
    sort::quicksort(&mut values, 0, last);
    println!("After sort:  {:?}", values);
}
