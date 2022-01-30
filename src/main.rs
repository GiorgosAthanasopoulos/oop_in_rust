mod person;
use crate::person::Person;
use crate::person::Person_;

fn main() {
    let mut name_: String = String::from("George");
    let mut age_: i8 = 19i8;

    let mut person: Person = Person {
        name: name_,
        age: age_
    };

    person.print_characteristics();

    name_ = String::from("Nick");
    age_ = 20;

    person.name = name_;
    person.age = age_;

    person.print_characteristics();
}
