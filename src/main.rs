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
        println!("3: Remove a sentence");
        println!("4: Leave");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("!Failed to read line!");

        match choice.trim() {
            "1" => {
                println!("----Enter the sentence to save:----");
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("!Failed to read line!");

                let mut file = match OpenOptions::new().append(true).create(true).open(&path) {
                    Err(why) => panic!("!Cannot open the file: {}!", why),
                    Ok(file) => file,
                };

                match file.write_all(input.as_bytes()) {
                    Err(why) => panic!("!Cannot write to the file: {}!", why),
                    Ok(_) => println!("----Sentence saved in 'Données_sauvegardées.txt'----"),
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
                println!("----Remove a sentence----");
                if let Ok(mut sentences) = read_sentences(&path) {
                    for (index, sentence) in sentences.iter().enumerate() {
                        println!("{}: {}", index, sentence);
                    }

                    println!("----Enter the index of the sentence to remove:----");
                    let mut index_str = String::new();
                    io::stdin()
                        .read_line(&mut index_str)
                        .expect("!Failed to read line!");
                    let index: usize = match index_str.trim().parse() {
                        Ok(num) => num,
                        Err(_) => {
                            println!("!Invalid index!");
                            continue;
                        }
                    };

                    if index < sentences.len() {
                        sentences.remove(index);
                        if let Err(why) = write_sentences(&path, &sentences) {
                            panic!("!Cannot write to the file: {}!", why);
                        }
                        println!("----Sentence removed successfully----");
                    } else {
                        println!("!Invalid index!");
                    }
                }
            }
            "4" => {
                println!("Bye!<3");
                break;
            }
            _ => {
                println!("!Invalid option, please try again.!");
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

fn read_sentences<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let mut sentences = Vec::new();
    for line in reader.lines() {
        sentences.push(line?);
    }
    Ok(sentences)
}

fn write_sentences<P>(filename: P, sentences: &[String]) -> io::Result<()>
where
    P: AsRef<Path>,
{
    let mut file = File::create(filename)?;
    for sentence in sentences {
        writeln!(file, "{}", sentence)?;
    }
    Ok(())
}
