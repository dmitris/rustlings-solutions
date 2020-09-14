// primitive_types3.rs
// Create an array with at least 100 elements in it where the ??? is.
// Execute `rustlings hint primitive_types3` for hints!

fn main() {
    let mut a = [0; 100];
    for i in 0..100 {
        a[i] = i;
    }

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!(
            "Meh, I eat arrays like that for breakfast. Len = {}",
            a.len()
        );
    }
    for x in a.iter() {
        println!("element {:?}", x)
    }
}
