// Rust programs have 3 memory regions where data is stored:
//   data memory - For data that is fixed in size and static. very fast to ause. compilers can optimize code.
//   stack memory - For data that is declared as variables within a function. very fast to access. compilers can optimize code.
//   heap memory - For data that is created while the application is running. slower to use. 
//      When data is added to this region we call it an allocation. When data is removed from this section we call it a deallocation.

struct SeaCreature {
    animal_type: String,
    name: String,
    arms: i32,
    legs: i32,
    weapon: String,
}

fn main() {
    // SeaCreature's data is on stack
    let ferris = SeaCreature {
        // String struct is also on stack,
        // but holds a reference to data on heap
        animal_type: String::from("crab"),
        name: String::from("Ferris"),
        arms: 2,
        legs: 4,
        weapon: String::from("claw"),

        // The text inside the quotes is read only data (e.g. "Ferris"), therefore it is placed in the data memory region.
    };

    let sarah = SeaCreature {
        animal_type: String::from("octopus"),
        name: String::from("Sarah"),
        arms: 8,
        legs: 0,
        weapon: String::from("brain"),
    };
    
    println!(
        "{} is a {}. They have {} arms, {} legs, and a {} weapon",
        ferris.name, ferris.animal_type, ferris.arms, ferris.legs, ferris.weapon
    );
    println!(
        "{} is a {}. They have {} arms, and {} legs. They have no weapon..",
        sarah.name, sarah.animal_type, sarah.arms, sarah.legs
    );
}
