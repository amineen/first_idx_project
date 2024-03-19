fn main() {
    let friend = Person::new("Aaron", 30);
    println!("{} is {} years old.", friend.name, friend.age);
}

//struct
struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn new(name: &str, age: u8) -> Person {
        Person {
            name: name.to_string(),
            age,
        }
    }
}
