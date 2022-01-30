pub struct Person {
    pub name: String,
    pub age: i8
}

pub trait Person_ {
    fn new(name_: String, age_: i8) -> Self;
    fn print_characteristics(&self) -> ();
}

impl Person_ for Person {
    fn new(name_: String, age_: i8) -> Person {
        Person {
            name: name_,
            age: age_
        }
    }

    fn print_characteristics(&self) -> () {
        println!("Hi! My name is {} and I'm {} years old.",
            self.name, self.age);
    }
}