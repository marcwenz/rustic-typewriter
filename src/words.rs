use std::{
    fs::File,
    io::{self, BufRead, Error, ErrorKind},
    path::Path,
};

use rand::{rngs::ThreadRng, Rng};

pub struct WordList {
    words: Vec<String>,
    rng: ThreadRng,
}

impl WordList {
    pub fn new(num: usize) -> WordList {
        let mut num = num;
        if num > 2000 {
            num = 2000;
        }
        WordList {
            words: load_words(num),
            rng: rand::thread_rng(),
        }
    }

    pub fn get_next_random(&mut self) -> String {
        self.words
            .get(self.rng.gen_range(0, &self.words.len()))
            .unwrap()
            .to_owned()
    }

    pub fn get_random_range(&mut self, i: usize) -> io::Result<Vec<String>> {
        if i > self.words.len() {
            return Err(Error::new(ErrorKind::Other, "oh no!"));
        }
        let mut words: Vec<String> = vec![];
        for j in 0..i {
            words.push(self.get_next_random());
        }
        Ok(words)
    }
}

fn load_words(num: usize) -> Vec<String> {
    let mut c = 0;
    let mut words: Vec<String> = vec![];
    if let Ok(lines) = read_lines("words/common.txt") {
        for line in lines {
            if let Ok(word) = line {
                words.push(word);
                c += 1;
            }
            if c == num {
                break;
            }
        }
    }
    words
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
