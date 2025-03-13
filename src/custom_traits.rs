trait Animal {
    fn name(&self) -> String;
    fn make_sound(&self) -> String;
    fn eat(&self) -> String;
}

struct Cat {
    name: String,
    make_sound: String,
    eat: String,
}

impl Cat {
    // Takes ownership of the struct
    fn take_ownership(self) -> String {
        self.name
    }

    // Takes a mutable reference to the struct
    fn change_name(&mut self, new_name: String) {
        self.name = new_name;
    }

    // Takes an immutable reference to the struct
    fn get_name(&self) -> &str {
        &self.name
    }
}

fn main() {
    let mut cat = Cat {
        name: String::from("Kitty"),
        make_sound: String::from("Meow"),
        eat: String::from("Fish"),
    };

    // Immutable reference
    println!("Cat's name: {}", cat.get_name());

    // Mutable reference
    cat.change_name(String::from("Whiskers"));
    println!("Cat's new name: {}", cat.get_name());

    // Ownership transfer
    let name = cat.take_ownership();
    println!("Cat's name after ownership transfer: {}", name);
}
