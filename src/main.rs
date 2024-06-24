use std::fs::OpenOptions;
use std::io::{self, Write};
use std::fs::File;
use std::io::{BufRead};
use std::path::Path;
fn main() {
    println!("Enter the sentence to save or press 'Enter' to see what is already saved.");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let path = "Données_sauvegardées.txt";

    let mut file = match OpenOptions::new().append(true).create(true).open(&path) {
        Err(why) => panic!("Can not open the file: {}", why),
        Ok(file) => file,
    };

    match file.write_all(input.as_bytes()) {
        Err(why) => panic!("Can not write in the file: {}", why),
        Ok(_) => println!("Sentence is save in 'Données_sauvegardées.txt'"),
    }

    if let Ok(lines) = read_lines("Données_sauvegardées.txt") {
        for line in lines.flatten() {
            println!("{}", line);
        }
    }
}


fn read_lines<P>(Données_sauvegardées: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open("Données_sauvegardées.txt")?;
    Ok(io::BufReader::new(file).lines())
}
