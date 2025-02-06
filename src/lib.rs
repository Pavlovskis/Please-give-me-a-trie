pub mod trie;
mod node;

#[cfg(test)]
mod tests {
    use std::{fs::File, io::BufReader};

    use crate::{test_help::get_words_starting_with, trie::{Trie, TrieBuilder}};

    #[test]
    fn insert() {
        // let file = File::open("./data/10k_cmn_words.txt")
        //     .expect("File not found");

        // let starts:Vec<&str> = vec!["a", "b", "d", "on", "we", "an"];

        // let mut reader:BufReader<File> = BufReader::new(file);

        // let mut control_words:Vec<String> = get_words_starting_with(&mut reader, starts[0], 100);
        // control_words.sort();

        // let trie:Trie = TrieBuilder::from(reader).lines(100).build();

        // let mut trie_words:Vec<String> = trie.get_suffix_of(starts[0]);
        // trie_words.sort();

        // println!("[{}] {:?}", trie_words.len(), trie_words);
        // println!("[{}] {:?}",control_words.len(), control_words);

        // for (i, tword) in trie_words.into_iter().enumerate() {
        //     assert_eq!( format!("{}{}", starts[0], tword), control_words[i+1] );
        // }
    }

    #[test]
    fn suffix() {

    }

    #[test]
    fn delete() {

    }

    #[test]
    fn contain() {

    }

}

mod test_help {
    use std::{fs::File, io::{BufRead, BufReader, Seek, SeekFrom}};

    pub fn get_words_starting_with(reader:&mut BufReader<File>, str:&str, lines:usize) -> Vec<String> {
        let mut res:Vec<String> = Vec::with_capacity(lines);

        for (i, raw) in reader.lines().enumerate() {
            if i >= lines { break; }
            
            let Ok(line) = raw else {
                eprintln!("Error processing line {}", i);
                continue;
            };

            if str == &line[0..str.len()] {
                res.push(line.trim().to_string());
            }
        }
        
        let _ = reader.seek(SeekFrom::Start(0));

        res
    }
}