pub mod tree;
mod node;
pub mod error_check;
pub mod autocomplete;

#[macro_export]
macro_rules! trie {
    [] => {
        Trie::new()
    };
    [ $($word:expr),+ $(,)? ] => {
        {
            let mut temp_trie = Trie::new();
            $( temp_trie.insert($word); )*
            temp_trie
        }
    };
}


#[cfg(test)]
mod tests {
    const LINES:usize = 1000; // first n lines from file ./data/10k_cmn_words.txt

    use std::{collections::HashMap, fs::File, io::{BufRead, BufReader, Read, Seek, SeekFrom}};

    use crate::{autocomplete::AutoCompletable, tree::{Trie, TrieBuilder}};

    #[test]
    fn insert() {
        let file = File::open("./data/10k_cmn_words.txt")
        .expect("File not found");
    
        let mut reader:BufReader<File> = BufReader::new(file);
    
        let mut control_words:HashMap<char, Vec<String>> = get_first_n_words(&mut reader, LINES);
    
        let mut control_total:usize = 0;
        for (_, v) in control_words.iter_mut() {
            v.sort();
            control_total += v.len();
        }
    
        let trie:Trie = TrieBuilder::from(reader).lines(LINES).build();

        assert_eq!(control_total, trie.words);
    
        for (k, mut trie_v) in trie.get_words() {
            trie_v.sort();
            
            if let Some(words_v) = control_words.get(&k) {
                // println!("W [{}] => {:?}", k, words_v);
                // println!("T [{}] => {:?}", k, trie_v);
    
                assert_eq!(words_v, &trie_v);
            }
        }

    }

    #[test]
    fn delete() {
        // i don't know what to do
    }

    #[test]
    fn contain() {
        let file = File::open("./data/10k_cmn_words.txt")
        .expect("File not found");
    
        let mut reader:BufReader<File> = BufReader::new(file);

        let control_words:HashMap<char, Vec<String>> = get_first_n_words(&mut reader, LINES);

        let trie:Trie = TrieBuilder::from(reader).lines(LINES).build();

        for (_, words) in control_words {
            for word in words {
                assert_eq!(trie.contains(&word), true, "Word [{word}] not found in trie");
            } 
        }
    }

    #[test]
    fn autocomplete() {
        let prefixes:Vec<&str> = vec!["an", "ba", "de", "pri"];

        let file = File::open("./data/10k_cmn_words.txt")
        .expect("File not found");
    
        let mut reader:BufReader<File> = BufReader::new(file);

        let control_words:HashMap<char, Vec<String>> = get_first_n_words(&mut reader, LINES);

        let trie:Trie = TrieBuilder::from(reader).lines(LINES).build();

        for prefix in prefixes {
            let mut trie_suf:Vec<String> = trie.complete(prefix);
            if trie.contains(prefix) {
                trie_suf.push("".to_string());
            }
            trie_suf.sort();
    
            let mut control_suf:Vec<String> = get_words_starting_with(&control_words, prefix);
            control_suf.sort();

            assert_eq!( control_suf, trie_suf);
        }
    }

    #[test]
    fn spelling_correction() {
        
    }

    // Helper functions

    fn get_first_n_words(reader:&mut BufReader<File>, n:usize) -> HashMap<char, Vec<String>> {
        let mut words:HashMap<char, Vec<String>> = HashMap::with_capacity(26);

        for (i, raw) in reader.by_ref().lines().enumerate() {
            if i >= n { break; }
            let Ok(line) = raw else { continue; };
    
            let Some(first) = line.chars().next() else { continue; };
            if let Some(v) = words.get_mut(&first) {
                v.push(line);
            }else {
                words.insert(first, vec![line]);
            }
        }
        let _ = reader.seek(SeekFrom::Start(0));

        words
    }

    fn get_words_starting_with(words:&HashMap<char, Vec<String>>, str:&str) -> Vec<String> {
        if str.is_empty() { return Vec::new(); }

        let mut res:Vec<String> = Vec::with_capacity(8);

        let first_ch:char = str.chars().next().unwrap();

        if let Some(v) = words.get(&first_ch) {
            for word in v {
                if word.len() < str.len() { continue; }
                if &word[0..str.len()] == str { res.push(word[str.len()..].to_string()); }
            }
        }
        
        res
    }

    

}