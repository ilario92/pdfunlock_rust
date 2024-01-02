use std::io;
use lopdf::{Document};

fn main() {
    let mut filename = String::new();
    let mut password = String::new();

    println!("Inserisci il path del file PDF:");
    io::stdin()
        .read_line(&mut filename)
        .expect("Errore nella lettura dell'input");
    filename = filename.trim_end().to_string();
    match Document::load(filename.clone()) {
        Ok(mut doc) => {
            println!("PDF loaded successfully.");
            if doc.is_encrypted() {
                println!("Inserisci la password del file PDF:");
                io::stdin()
                    .read_line(&mut password)
                    .expect("Errore nella lettura dell'input");
                println!("PDF encrypted! Try decrypt by password...");
                if doc.decrypt(password.trim_end()).is_ok() {
                    let new_filename = change_filename(filename.as_str());
                    println!("Decrypted!! Saving with filename");
                    doc.save(new_filename).expect("TODO: panic message");
                } else {
                    println!("Wrong password!")
                }
            } else {
                println!("File not encrypted!")
            }
        }
        Err(e) => println!("Error: {:?}", e),
    }
}

fn change_filename(input: &str) -> String {
    input
        .rsplitn(2, '.')
        .collect::<Vec<&str>>()
        .into_iter()
        .rev()
        .collect::<Vec<&str>>()
        .join("_dec.")
}