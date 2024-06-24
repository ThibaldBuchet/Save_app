use std::fs::File;
use std::io::{BufWriter, Write};

fn main() -> std::io::Result<()> {

    let file = File::create("Données_sauvegardées.txt")?;

    let mut writer = BufWriter::new(file);

    writer.write_all(b"Hello! My name is Thibald")?;

    writer.flush()?;

    println!("Write Operation Successful");
    Ok(())
}

    