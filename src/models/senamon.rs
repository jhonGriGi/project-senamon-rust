#[derive(Clone)]
pub enum ElementType {
    Fire,
    Water,
    Grass,
    Flying,
    Electric,
}

pub struct Senamon {
    name: String,
    level: i32,
    element_type: ElementType,
    weight: f32,
    health_points: i32,
    attack_level: i32,
    phase: String,
    energy_level: i32,
    description: String,
}

impl Senamon {
    pub fn new(
        name: &str,
        level: &i32,
        element_type: &ElementType,
        weight: &f32,
        health_points: &i32,
        attack_level: &i32,
        phase: &str,
        energy_level: &i32,
        description: &str,
    ) -> Self {
        Senamon {
            name: name.to_string(),
            level: *level,
            element_type: element_type.clone(),
            weight: *weight,
            health_points: *health_points,
            attack_level: *attack_level,
            phase: phase.to_string(),
            energy_level: *energy_level,
            description: description.to_string(),
        }
    }

    pub fn upgrade_level(&mut self) -> () {
        self.level += 1
    }

    pub fn subtract_health(&mut self, damage: &i32) -> () {
        self.health_points -= damage;
    }
}
