use std::{
    env::{self}, error::Error, fs::File, io::{self, BufRead}
};

enum CountType {
    Words,
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
        let count = count_words(&f)?;
        total += count;
        println!("{count: >8} {f}");
    }

    if files_len > 1 {
        println!("{total: >8} total")
    }

    Ok(())
}

fn cli_args() -> Result<Args, Box<dyn Error>> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() < 1 {
        return Err("no input file(s)")?;
    }

    let mut count_type: CountType = CountType::Words;
    if args[0].starts_with("-") {
        match args[0].as_str() {
            "-w" => count_type = CountType::Words,
            _ => return Err("invalid count type")?
        }
    }
    Ok(Args { files: args, count_type })
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
