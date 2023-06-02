use std::collections::HashMap;
const ELEMENTS: usize = 1_000_000;
fn main() {
    let mut my_vector = Vec::new();
    let now = std::time::Instant::now();
    for i in 0..ELEMENTS {
        my_vector.push(i)
    }
    let elapsed = now.elapsed();
    println!(
        "Inserting {ELEMENTS} elements into a vector took {} usecs",
        elapsed.as_micros()
    );
    let mut my_hashmap = HashMap::new();
    let now = std::time::Instant::now();
    for i in 0..ELEMENTS {
        my_hashmap.insert(i, i);
    }
    let elapsed = now.elapsed();
    println!(
        "Inserting {ELEMENTS} elements into a Hashmap took {} usecs",
        elapsed.as_micros()
    );

    let elem_to_find = ELEMENTS - 2;
    let now = std::time::Instant::now();
    my_vector.iter().find(|n| **n == elem_to_find);
    let elapsed = now.elapsed();
    println!(
        "Searching {ELEMENTS} elements in a Vector took {} usecs",
        elapsed.as_micros()
    );
    let now = std::time::Instant::now();
    let _ = my_hashmap.get(&elem_to_find);
    let elapsed = now.elapsed();
    println!(
        "Searching {ELEMENTS} elements in a Hashmap took {} usecs",
        elapsed.as_micros()
    );
}
