#[derive(Debug)]
struct ColorRegularStruct {
    red: u8,
    green: u8,
    blue: u8,
}

#[derive(Debug)]
struct ColorTupleStruct(u8, u8, u8);

#[derive(Debug)]
struct UnitStruct;

fn main() {
    // 1. Regular struct: fields are named, so order doesn't matter.
    let magenta = ColorRegularStruct {
        blue: 255,
        red: 255,
        green: 0,
    };
    println!("magenta = {magenta:?}");
    println!("its red channel = {}", magenta.red);

    // 2. Struct update syntax: take every field from `magenta` except the ones
    // listed. Handy when you only want to tweak one channel.
    let purple = ColorRegularStruct { red: 128, ..magenta };
    println!("purple  = {purple:?}");

    // 3. Field init shorthand: when the variable name matches the field name,
    // `red: red` can be shortened to just `red`.
    let (red, green, blue) = (0, 0, 255);
    let pure_blue = ColorRegularStruct { red, green, blue };
    println!("blue    = {pure_blue:?}");

    // 4. Tuple struct: fields are positional, accessed with .0 / .1 / .2.
    let yellow = ColorTupleStruct(255, 255, 0);
    println!("yellow  = {yellow:?}  (green channel = {})", yellow.1);

    // Tuple structs destructure just like tuples do.
    let ColorTupleStruct(r, g, b) = yellow;
    println!("destructured yellow -> r={r} g={g} b={b}");

    // 5. Unit struct: no data at all, so the name *is* the value.
    let unit = UnitStruct;
    println!("unit    = {unit:?}  (size in bytes: {})", size_of::<UnitStruct>());

    // A struct's size is just its fields packed together: 3 x u8 = 3 bytes.
    println!(
        "sizes: regular={} tuple={}",
        size_of::<ColorRegularStruct>(),
        size_of::<ColorTupleStruct>(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regular_structs() {
        let green = ColorRegularStruct {
            red: 0,
            green: 255,
            blue: 0,
        };

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // TODO: Instantiate a tuple struct.
        let green = ColorTupleStruct(0, 255, 0);

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // TODO: Instantiate a unit struct.
        let unit_struct = UnitStruct;
        let message = format!("{unit_struct:?}s are fun!");

        assert_eq!(message, "UnitStructs are fun!");
    }
}
