// move_semantics2.rs
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand for a hint.

// Expected output:
// vec0 has length 3 content `[22, 44, 66]`
// vec1 has length 4 content `[22, 44, 66, 88]`


#[test]
fn main() {
    let mut vec0 = Vec::new();

    // Do not move the following line!
    let mut vec1 = fill_vec(&mut vec0);

    vec1.push(88);
    assert_eq!(vec0, vec![22, 44, 66]);
    assert_eq!(vec1, vec![22, 44, 66, 88]);
}

fn fill_vec(vec: &mut Vec<i32>) -> Vec<i32> {
    // let vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);
    vec.to_vec()
}
