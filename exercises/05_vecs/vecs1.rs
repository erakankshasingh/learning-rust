fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // Array

    // TODO: Create a vector called `v` which contains the exact same elements as in the array `a`.
    // Use the vector macro.
    let v = vec![10, 20, 30, 40];

    (a, v)
}

fn main() {
    // You can optionally experiment here.
    let a1 = [1, 2, 3]; // Array
    let v1 = vec![1, 2, 3]; // Vector
    println!("{a1:?} {v1:?}");
    println!("{:?}", array_and_vec());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, *v);
    }
}
