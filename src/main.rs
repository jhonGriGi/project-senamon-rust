mod models;
mod traits;
mod utils;

mod prelude {
    pub use crate::models::{person_a::PersonA, person_b::PersonB};
    pub use crate::traits::greeting::Greeting;
    pub use crate::utils::io::{show_console_message, get_console_input};
}

use crate::prelude::*;

fn main() {
    show_console_message("Ingresa un valor");
    let value = get_console_input();
    let final_message = format!("valor actual {}", value).to_string();
    show_console_message(&final_message);
}
