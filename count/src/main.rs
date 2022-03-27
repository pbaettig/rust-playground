use std::io::BufReader;
use std::io;
use std::env;
use std::fs::File;

use std::io::prelude::*;

enum InputSource {
    File(String),
    Stdin,
}

impl InputSource  {
    fn display(&self) -> String {
        match self {
            InputSource::File(f) => f.to_string(),
            InputSource::Stdin => String::from("stdin")
        }
    }

    fn reader(&self) -> Box<dyn BufRead> {
        match self {
            InputSource::Stdin => Box::new(BufReader::new(io::stdin())),
            InputSource::File(filename) => Box::new(BufReader::new(File::open(filename).unwrap()))
        } 
    }
}

struct CountResult {
    source: String,
    lines: usize,
    words: usize,
    characters: usize,
}

impl CountResult {
    fn display(&self) -> String {
        format!("{}: {} lines, {} words, {} characters", self.source, self.lines, self.words, self.characters)
    }
}

fn count(input: InputSource) -> CountResult {
   let mut r = CountResult { source: input.display(), lines: 0, words: 0, characters: 0};
  
    // read lines in buf_reader...
    for line in input.reader().lines() {
        let line_length = match &line {
            Err(_) => 0,
            Ok(l) => l.len(),
        };

        let words = match &line {
            Err(_) => 0,
            Ok(l) => l.split(" ").filter(|&x| !x.is_empty()).count()
        };

        //                            v add newline char
        r.characters += line_length + 1;
        r.words += words;
        r.lines +=1;
    }
    
    r
}

fn main() {
    if env::args().len() == 1 {
        println!("{}",count(InputSource::Stdin).display());
    } else {
        for arg in env::args().skip(1) {
            println!("{}",count(InputSource::File(arg)).display());
        }
    }
}
