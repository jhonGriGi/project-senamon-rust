use crate::traits::greeting::Greeting;

pub struct PersonA {
    name: String
}

impl Greeting for PersonA {

    fn say_hello(&self) -> String {
        format!("Hello person a {}", self.name.clone())
    }
}

impl PersonA {
    pub fn create(name: String) -> PersonA {
        PersonA {
            name
        }
    }
}