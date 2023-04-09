use clearscreen::clear;
use console::style;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;

fn main() {
    const MAX_ATTEMPTS: u8 = 3;

    let welcome_options = &[
        "Siema z rana",
        "Siema z wieczora",
        "Dobry wieczór",
        "Siema z nocy",
        "Dobranoc",
        "Siema z dnia",
    ];

    let mut failed_attempts = 0;

    let mut selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Wybierz swoje przywitanie")
        .items(&welcome_options[..])
        .interact()
        .unwrap();

    while selection != 0 {
        clear().expect("Nie udało się wyczyścić ekranu");
        println!(
            "{}",
            style(format!(
                "Niepoprawne przywitanie, próbuj dalej, pozostało prób: {}",
                MAX_ATTEMPTS - failed_attempts
            ))
            .bright()
            .red()
            .bold()
        );
        selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Wybierz swoje przywitanie")
            .items(&welcome_options[..])
            .interact()
            .unwrap();
        failed_attempts += 1;

        if failed_attempts == MAX_ATTEMPTS {
            clear().expect("Nie udało się wyczyścić ekranu");
            println!(
                "{}",
                style("Przygotuj się na karę, zostaniesz zbanowany na 24h")
                    .on_red()
                    .black()
                    .italic()
            );
            break;
        }
    }

    if selection == 0 {
        clear().expect("Nie udało się wyczyścić ekranu");
        println!(
            "{}",
            style("Prawidłowe przywitanie, SIEMA Z RANA!!!")
                .on_green()
                .black()
                .italic()
        );
    }
}
