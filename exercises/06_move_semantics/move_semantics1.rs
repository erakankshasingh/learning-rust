// TODO: Fix the compiler error in this function.
fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(88);

    vec
}

fn main() {
    // You can optionally experiment here.
    let vec0 = vec![22, 44, 66];

    // `fill_vec` takes `vec0` *by value*, so ownership moves into the function.
    // After this line `vec0` is no longer usable — try uncommenting the
    // `println!` below and the compiler will complain about a moved value.
    let vec1 = fill_vec(vec0);
    // println!("{vec0:?}"); // error[E0382]: borrow of moved value: `vec0`

    println!("{vec1:?}"); // [22, 44, 66, 88]

    // The value came back out of the function, so we own it again and can
    // feed it straight back in for another round.
    let vec2 = fill_vec(vec1);
    println!("{vec2:?}"); // [22, 44, 66, 88, 88]

    // If you want to keep the original around, hand over a clone instead.
    let vec3 = vec![1, 2, 3];
    let vec4 = fill_vec(vec3.clone());
    println!("original: {vec3:?}, filled: {vec4:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics1() {
        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec(vec0);
        assert_eq!(vec1, vec![22, 44, 66, 88]);
    }
}
