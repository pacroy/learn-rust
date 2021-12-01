#![allow(dead_code)] // this line prevents compiler warnings

// enum elements can also have one or more data types allowing them to behave like union from C.
// An enum data value will have a memory size equal to its largest element.
// In addition to element data types (if any), each element also has a numeric value that represents which tag it is.
// Rust's `enum` is something also known as a 'tagged' union.
// The combining of types to make a new type is what people mean when they say Rust has 'algebraic' types.
enum Species { Crab, Octopus, Fish, Clam }
enum PoisonType { Acidic, Painful, Lethal }
enum Size { Big, Small }
enum Weapon {
    Claw(i32, Size),
    Poison(PoisonType),
    None
}

struct SeaCreature {
    species: Species,
    name: String,
    arms: i32,
    legs: i32,
    weapon: Weapon,
}

fn main() {
    // SeaCreature's data is on stack
    let ferris = SeaCreature {
        // String struct is also on stack,
        // but holds a reference to data on heap
        species: Species::Crab,
        name: String::from("Ferris"),
        arms: 2,
        legs: 4,
        weapon: Weapon::Claw(2, Size::Small),
    };

    match ferris.species {
        Species::Crab => {  // When an `enum` is pattern matched using `match`, you can bind a variable name to each data value.
            match ferris.weapon {
                Weapon::Claw(num_claws,size) => {
                    let size_description = match size {
                        Size::Big => "big",
                        Size::Small => "small"
                    };
                    println!("ferris is a crab with {} {} claws", num_claws, size_description)
                },
                _ => println!("ferris is a crab with some other weapon")
            }
        },
        _ => println!("ferris is some other animal"),
    }
}