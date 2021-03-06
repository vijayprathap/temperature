use std::io;

fn main() {
    loop {
        greetings();
        let mut option = String::new();
        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line");

        let option: u32 = match option.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        if option == 1 {
            println!("Enter the temperature in Celcius: ");
            let celcius = get_user_input_as_float();
            let farenheit = convert_to_farenheit(celcius);
            println!("\nFarenheit value is {}\n\n", farenheit);
        }
        else if option == 2 {
            println!("Enter the temperature in Farenheit: ");
            let farenheit = get_user_input_as_float();
            let celcius = convert_to_celcius(farenheit);
            println!("\nCelcius value is {}\n\n", celcius);
        }
        else if option == 3 {
            break;
        }
        else {
            println!("The option does not exists!!\n\n");
        }
    }
}

fn get_user_input_as_float() -> f32{
    let mut temperature = String::new();
    io::stdin()
                .read_line(&mut temperature)
                .expect("Failed to read line\n\n");

    let temperature: f32 = match temperature.trim().parse() {
        Ok(num) => num,
        Err(_) => display_error_message_and_get_input_as_float(),
    };
    temperature
}

fn display_error_message_and_get_input_as_float() -> f32{
    println!("\n\nInvalid input\n\n");
    get_user_input_as_float()
}

fn greetings(){
    println!("Hello there!!! We can perform the following operations.\n");
    println!("1. Convert to farenheit");
    println!("2. Convert to celcius");
    println!("3. Exit\n\n");
    println!("Enter your choice: ");
}

fn convert_to_farenheit(celcius:f32) -> f32{
    celcius * 1.8 + 32.00
}

fn convert_to_celcius(farenheit:f32) -> f32{
    (farenheit - 32.00) * 1.8
}
