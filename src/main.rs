
use std::io::{File, Open, Read};
use std::io::BufferedReader;

mod trie;

pub fn chomp(s:&mut String) {
    let last = s.as_slice().char_indices().last();
    match last {
        Some(_) => {
            s.pop();
        },
        _ => {}
    }
}

fn main() {
    let mut mytrie = trie::new();

    let p = Path::new("/usr/share/dict/words");
    let file = match File::open_mode(&p, Open, Read) {
        Ok(f) => f,
        Err(e) => panic!("File error: {}", e)
    };

    let mut reader = BufferedReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(l) => {
                let mut m = l.clone();
                chomp(&mut m);
                mytrie.insert(m.as_slice(), m.clone());
            },
            Err(e) => panic!("Reading lines: {}", e)
        }
    }

    let testwords = &[
        "existential",
        "cromulent",
        "shaganappi",
        "multisegmentate"
    ];
    for word in testwords.iter() {
        println!("find '{}' --> {}", word, mytrie.find(*word));
    }
}
