fn animal_habitat(animal: &str) -> &str {
    // TODO: Fix the compiler error in the statement below.
    let identifier: i32;
    if animal == "crab" {
        identifier = 1;
    } else if animal == "gopher" {
        identifier = 2;
    } else if animal == "snake" {
        identifier = 3;
    } else {
        identifier = 0;
    };

    // Don't change the expression below!
    if identifier == 1 {
        "Beach"
    } else if identifier == 2 {
        "Burrow"
    } else if identifier == 3 {
        "Desert"
    } else {
        "Unknown"
    }
}

fn main() {
    // You can optionally experiment here.
    let animal_str: &str = "crab";
    println!("{}", animal_habitat(animal_str));
    println!("{}", animal_habitat("gopher"));
    println!("{}", animal_habitat("snake"));
    println!("{}", animal_habitat("tiger"));
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gopher_lives_in_burrow() {
        assert_eq!(animal_habitat("gopher"), "Burrow")
    }

    #[test]
    fn snake_lives_in_desert() {
        assert_eq!(animal_habitat("snake"), "Desert")
    }

    #[test]
    fn crab_lives_on_beach() {
        assert_eq!(animal_habitat("crab"), "Beach")
    }

    #[test]
    fn unknown_animal() {
        assert_eq!(animal_habitat("dinosaur"), "Unknown")
    }
}
