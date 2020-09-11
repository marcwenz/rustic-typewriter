use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn load_words(num: usize) -> Vec<String> {
    if num > 2000 {
        panic!("Number of words has to be less that 2000");
    }
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
