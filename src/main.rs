use std::{
    env::{self}, error::Error, fs::File, io::{self, BufRead}
};

fn main() -> Result<(), Box<dyn Error>> {
    let files = cli_files()?;
    let files_len = files.len();

    let mut total: usize = 0;
    for f in files {
        let count = count_words(&f)?;
        total += count;
        println!("{count: >8} {f}");
    }

    if files_len > 1 {
        println!("{total: >8} total")
    }

    Ok(())
}

fn cli_files() -> Result<Vec<String>, Box<dyn Error>> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() < 1 {
        return Err("no input file(s)")?;
    }
    Ok(args)
}

fn count_words(path: &String) -> Result<usize, Box<dyn Error>> {
    let mut count: usize = 0;
    let file = File::open(path)?;
    let buf_reader = io::BufReader::new(file);
    for line in buf_reader.lines() {
        count = count + line?.split_whitespace().count();
    }
    Ok(count)
}
