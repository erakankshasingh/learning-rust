fn main() {
    let a = [1, 2, 3, 4, 5];

    // 1. Range forms. Start is inclusive, end is EXCLUSIVE.
    println!("{:?}", &a[1..4]); // [2, 3, 4]
    println!("{:?}", &a[..3]);  // from the start
    println!("{:?}", &a[2..]);  // to the end
    println!("{:?}", &a[..]);   // the whole thing
    println!("{:?}", &a[1..=3]); // ..= makes the end inclusive

    // 2. An array and a slice are different types.
    let arr: [i32; 5] = a;      // fixed length, known at compile time
    let sli: &[i32] = &a[1..4]; // a borrowed view: pointer + length
    println!("{} vs {}", arr.len(), sli.len());

    // 3. Indexing out of bounds panics at RUNTIME, not compile time.
    //    Uncomment to see the panic message:
    // println!("{:?}", &a[1..99]);

    // get() is the non-panicking version — it returns an Option.
    println!("{:?}", a.get(2));  // Some(3)
    println!("{:?}", a.get(99)); // None

    // 4. A slice borrows, it doesn't copy. Writing through a &mut
    //    slice changes the original array.
    let mut b = [10, 20, 30];
    let s = &mut b[..2];
    s[0] = 99;
    println!("{:?}", b); // [99, 20, 30]

    // 5. Handy slice methods.
    println!("{:?}", a.first());          // Some(1)
    println!("{:?}", a.last());           // Some(5)
    println!("{:?}", a.split_at(2));      // ([1, 2], [3, 4, 5])
    println!("{:?}", a.iter().sum::<i32>());
    for pair in a.windows(2) {
        print!("{:?} ", pair); // overlapping pairs
    }
    println!();

    // 6. &str is just a slice of a String's bytes.
    let word = String::from("crustacean");
    let part: &str = &word[0..5];
    println!("{}", part); // crust
}

#[cfg(test)]
mod tests {
    #[test]
    fn slice_out_of_array() {
        let a = [1, 2, 3, 4, 5];

        // TODO: Get a slice called `nice_slice` out of the array `a` so that the test passes.
        let nice_slice = &a[1..4];

        assert_eq!([2, 3, 4], nice_slice);
    }
}
