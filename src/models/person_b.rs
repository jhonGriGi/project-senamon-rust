use crate::traits::greeting::Greeting;

pub struct PersonB {
    name: String
}
impl Greeting for PersonB {
    fn say_hello(&self) -> String {
        format!("Hello person b {}", self.name.clone())
    }
}

impl PersonB {
    pub fn create(name: String) -> PersonB {
        PersonB {
            name
        }
    }
}