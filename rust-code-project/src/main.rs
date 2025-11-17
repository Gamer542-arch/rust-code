use std::io::{self, Write};

// Definicja struktury
struct Osoba {
    imie: String,
    nazwisko: String,
    wiek: u32,
    email: Option<String>, // opcjonalny e-mail
}

fn main() {
    // Przykładowe osoby
    let osoba1 = Osoba {
        imie: String::from("Piotr"),
        nazwisko: String::from("Kowalski"),
        wiek: 30,
        email: Some(String::from("piotr@example.com")),
    };

    let osoba2 = Osoba {
        imie: String::from("Anna"),
        nazwisko: String::from("Nowak"),
        wiek: 25,
        email: None,
    };

    // Pobieranie danych od użytkownika
    let imie = read_input("Podaj imię: ");
    let nazwisko = read_input("Podaj nazwisko: ");
    let wiek: u32 = read_input("Podaj wiek: ").parse().unwrap_or(0);
    let email_input = read_input("Podaj email (lub pozostaw puste): ");
    let email = if email_input.trim().is_empty() {
        None
    } else {
        Some(email_input)
    };

    let osoba3 = Osoba {
        imie,
        nazwisko,
        wiek,
        email,
    };

    // Wyświetlanie wszystkich osób
    println!("Osoba 1: {} {}, wiek: {}, email: {:?}", 
        osoba1.imie, osoba1.nazwisko, osoba1.wiek, osoba1.email);
    println!("Osoba 2: {} {}, wiek: {}, email: {:?}", 
        osoba2.imie, osoba2.nazwisko, osoba2.wiek, osoba2.email);
    println!("Osoba 3: {} {}, wiek: {}, email: {:?}", 
        osoba3.imie, osoba3.nazwisko, osoba3.wiek, osoba3.email);
}   

// Funkcja do wczytywania danych od użytkownika
fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}