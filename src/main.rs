use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufRead;
use std::io::{self, Write};
use std::path::Path;

fn main() {
    let path = "Données_sauvegardées.txt";

    loop {
        println!("----Choose an option:----");
        println!("1: Enter a new sentence");
        println!("2: See file content");
        println!("3: leave");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim() {
            "1" => {
                println!("Enter the sentense to save:");
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");

                let mut file = match OpenOptions::new().append(true).create(true).open(&path) {
                    Err(why) => panic!("Can not open the file: {}", why),
                    Ok(file) => file,
                };

                match file.write_all(input.as_bytes()) {
                    Err(why) => panic!("Can not write in the file: {}", why),
                    Ok(_) => println!("Sentence saved in 'Données_sauvegardées.txt'"),
                }
            }
            "2" => {
                println!("----File content in 'Données_sauvegardées.txt':----");
                if let Ok(lines) = read_lines(&path) {
                    for line in lines.flatten() {
                        println!("{}", line);
                    }
                }
            }
            "3" => {
                println!("Bye!");
                break;
            }
            _ => {
                println!("Invalid option, please try again.");
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
