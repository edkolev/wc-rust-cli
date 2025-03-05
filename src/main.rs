use std::{
    env::{self},
    error::Error,
    fs::File,
    io::{self, BufRead, Read},
};

enum CountType {
    Words,
    Lines,
    Bytes,
}

struct Args {
    files: Vec<String>,
    count_type: CountType,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = cli_args()?;
    let files_len = args.files.len();

    let mut total: usize = 0;
    for f in args.files {
        let count = match args.count_type {
            CountType::Lines => count_lines(&f)?,
            CountType::Words => count_words(&f)?,
            CountType::Bytes => count_bytes(&f)?,
        };

        total += count;
        println!("{count: >8} {f}");
    }

    if files_len > 1 {
        println!("{total: >8} total")
    }

    Ok(())
}

fn cli_args() -> Result<Args, Box<dyn Error>> {
    let mut args: Vec<String> = env::args().skip(1).collect();
    if args.len() < 1 {
        return Err("no input file(s)")?;
    }

    let mut count_type: CountType = CountType::Words;
    if args[0].starts_with("-") {
        match args[0].as_str() {
            "-w" => count_type = CountType::Words,
            "-l" => count_type = CountType::Lines,
            "-c" => count_type = CountType::Bytes,
            _ => return Err("invalid count type")?,
        }
        args = env::args().skip(2).collect();
    }
    Ok(Args {
        files: args,
        count_type,
    })
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

fn count_lines(path: &String) -> Result<usize, Box<dyn Error>> {
    let file = File::open(path)?;
    let buf_reader = io::BufReader::new(file);
    Ok(buf_reader.lines().count())
}

fn count_bytes(path: &String) -> Result<usize, Box<dyn Error>> {
    let file = File::open(path)?;
    let buf_reader = io::BufReader::new(file);
    Ok(buf_reader.bytes().count())
}
