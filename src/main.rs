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
    count_types: Vec<CountType>, // TODO use a hashset to ensure no duplicates
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = cli_args()?;
    let files_len = args.files.len();

    let mut total: usize = 0;
    for f in args.files {
        let count = match args.count_types[..] {
            [CountType::Lines] => count_lines(&f)?,
            [CountType::Words] => count_words(&f)?,
            [CountType::Bytes] => count_bytes(&f)?,
            _ => panic!("unplemented"),
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
    let mut a = Args {files: vec![], count_types: vec![]};
    for arg in env::args().skip(1) {
        if arg.as_str().starts_with("-") {
            match arg.as_str() {
                "-w" => a.count_types.push(CountType::Words),
                "-l" => a.count_types.push(CountType::Lines),
                "-b" => a.count_types.push(CountType::Bytes),
                _ => return Err("invalid count type")?,
            }
        } else {
            a.files.push(arg);
        }
    }

    if a.files.len() == 0 {
        return Err("no input file(s)")?;
    }
    if a.count_types.len() == 0 {
        a.count_types = vec!(CountType::Words)
    }
    Ok(a)
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
