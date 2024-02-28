pub fn show_console_message(message: &str) {
    println!("{}", message);
}

pub fn get_console_input() -> String {
    let mut input_message = String::new();
    let error_message = "Error al leer el mensaje en consola";

    std::io::stdin()
        .read_line(&mut input_message)
        .expect(error_message);

    input_message
}
