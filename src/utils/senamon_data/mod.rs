use crate::{models::senamon::ElementType, Senamon};

pub fn get_senamon_data()-> Vec<Senamon> {
    let pokemon_data = [
        (
            "Charmander",
            1,
            ElementType::Fire,
            8.5,
            39,
            52,
            "Base".to_string(),
            50,
            "Lizard Pokémon".to_string(),
        ),
        (
            "Squirtle",
            1,
            ElementType::Water,
            9.0,
            44,
            48,
            "Base".to_string(),
            50,
            "Tiny Turtle Pokémon".to_string(),
        ),
        (
            "Bulbasaur",
            1,
            ElementType::Grass,
            6.9,
            45,
            49,
            "Base".to_string(),
            50,
            "Seed Pokémon".to_string(),
        ),
        // Agrega más datos de Pokémon aquí...
    ];

    // Crear un array de Senamons usando los datos proporcionados
    let pokemon_team: Vec<Senamon> = pokemon_data
        .iter()
        .map(
            |(
                name,
                level,
                element_type,
                weight,
                health_points,
                attack_level,
                phase,
                energy_level,
                description,
            )| {
                Senamon::new(
                    name,
                    level,
                    element_type,
                    weight,
                    health_points,
                    attack_level,
                    phase,
                    energy_level,
                    description,
                )
            },
        )
        .collect();

    pokemon_team
}
