fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(88);

    vec
}

fn main() {
    // You can optionally experiment here.
    let vec_0 = vec![11, 22, 33, 44, 55, 66];
    let vec_1 = fill_vec(vec_0);
    println!("{vec_1:?}"); // [11, 22, 33, 44, 55, 66, 88]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics2() {
        let vec0 = vec![22, 44, 66];

        // Hand `fill_vec` its own copy so `vec0` isn't moved away.
        let vec1 = fill_vec(vec0.clone());

        assert_eq!(vec0, [22, 44, 66]);
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}
