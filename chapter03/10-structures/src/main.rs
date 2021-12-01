fn main() {
    println!("==== structures ====");
    let ferris = SeaCreature {
        animal_type: String::from("crab"),
        name: String::from("Ferris"),
        arms: 2,
        legs: 4,
        weapon: String::from("claw"),
    };

    println!(
        "{} is a {}. They have {} arms, {} legs, and a {} weapon",
        ferris.name, ferris.animal_type, ferris.arms, ferris.legs, ferris.weapon
    );

    println!("==== struct like a tuple ====");
    // This is still a struct on a stack
    let loc = Location(42, 32);
    println!("{}, {}", loc.0, loc.1);

    println!("==== unit-like struct ====");
    let _m = Marker;
}

// A struct is a collection of fields.
// A field is simply a data value associated with a data structure. Its value can be of a primitive type or a data structure.

struct SeaCreature {
    // String is a struct
    animal_type: String,
    name: String,
    arms: i32,
    legs: i32,
    weapon: String,
}

// you can create structs that are used like a tuple.
struct Location(i32, i32);

// Unit-like struct
//   Structs do not have to have any fields at all.
//   `unit` is another word for an empty tuple ().
struct Marker;
