use crate::{node::NodeRef, tree::Trie};

pub trait ErrorCheckable {
    /// If word is not found in trie, try to find closest words to the provided string
    fn spelling_check(&self, word:&str) -> Option<Vec<String>>;
}

impl ErrorCheckable for Trie {
    // Needs some serious optimisation, I know
    fn spelling_check(&self, word:&str) -> Option<Vec<String>> {
        if self.contains(word) { return None; }

        let mut res:Vec<String> = Vec::with_capacity(4);

        let _edits:[fn(&Trie, word:&str) -> Vec<String>; 4] = [
            Self::deletion, Self::transposition, Self::alteration, Self::insertion 
        ];

        // for edit in edits {
        //     res.append( &mut edit(&self, word, 1) );
        // }

        // res.append( &mut self.deletion(word) );
        // res.append( &mut self.transposition(word) );
        res.append( &mut self.alteration(word) ); // todo!
        // res.append( &mut self.insertion(word) );

        if res.is_empty() { return None; }

        Some(res)
    }
}

trait ErrorCheckEdits {

    // A string s has a deletion distance 1 from another string t if and only if t is equal to s with one character removed. 
    // The only strings that are a deletion distance of 1 from “bird” are “ird”, “brd”, “bid”, and “bir”. 
    // Note that if a string s has a deletion distance of 1 from another string t then |s| = |t| -1. 
    // Also, there are exactly | t | strings that are a deletion distance of 1 from t.
    // The dictionary may contain 0 to n of the strings one deletion distance from t .
    fn deletion(&self, word:&str) -> Vec<String>;

    // A string s has a transposition distance 1 from another string t if and only if t is equal to s with two adjacent 
    // characters transposed. The only strings that are a transposition Distance of 1 from “house” are “ohuse”, “huose”, 
    // “hosue” and “houes”. Note that if a string s has a transposition distance of 1 from another string t then |s| = |t|. 
    // Also, there are exactly | t | - 1 strings that are a transposition distance of 1 from t . 
    // The dictionary may contain 0 to n of the strings one transposition distance from t .
    fn transposition(&self, word:&str) -> Vec<String>;

    // A string s has an alteration distance 1 from another string t if and only if t is equal to s with exactly one character 
    // in s replaced by a lowercase letter that is not equal to the original letter. The only strings that are an alternation 
    // distance of 1 from “top” are “aop”, “bop”, …, “zop”, “tap”, “tbp”, …, “tzp”, “toa”, “tob”, …, and “toz”.
    // Note that if a string s has an alteration distance of 1 from another string t then |s| = |t|. 
    // Also, there are exactly 25* | t | strings that are an alteration distance of 1 from t . 
    // The dictionary may contain 0 to n of the strings one alteration distance from t .
    fn alteration(&self, word:&str) -> Vec<String>;

    // A string s has an insertion distance 1 from another string t if and only if t has a deletion distance of 1 from s . 
    // The only strings that are an insertion distance of 1 from “ask” are “aask”, “bask”, “cask”, … “zask”, “aask”, “absk”, “acsk”, … 
    // “azsk”, “asak”, “asbk”, “asck”, … “aszk”, “aska”, “askb”, “askc”, … “askz”. Note that if a string s has an insertion 
    // distance of 1 from another string t then |s| = |t|+1. Also, there are exactly 26* (|t|+1) strings that are an insertion 
    // distance of 1 from t . The dictionary may contain 0 to n of the strings one insertion distance from t .
    fn insertion(&self, word:&str) -> Vec<String>;
}

impl ErrorCheckEdits for Trie {
    
    fn deletion(&self, word:&str) -> Vec<String> {
        let distance = 1;
        let mut res:Vec<String> = Vec::with_capacity(4);

        // let mut cur:NodeRef = self.root.clone();

        for i in 0..word.len() - distance + 1 {
            let new_word:String = format!("{}{}", &word[..i], &word[i + distance..]);
            if self.contains(&new_word) { res.push(new_word); }
        }
        res
    }

    fn transposition(&self, word:&str) -> Vec<String> {
        let mut distance = 2;
        if distance == 1 { distance += 1; }

        let mut res:Vec<String> = Vec::with_capacity(4);

        for i in 0..word.len() - distance + 1 {
            let mut chars_slice:Vec<char> = word[i..i + distance].chars().collect();
            
            let permutations:Vec<String> = permutations(&mut chars_slice);

            for perm in permutations {
                let new_word:String = format!("{}{}{}", &word[0..i], perm, &word[i+distance..] );
                println!("{}", new_word);
                if self.contains(&new_word) { res.push(new_word); }
            }
        }
        res
    }

    fn alteration(&self, word:&str) -> Vec<String> {
        let mut res:Vec<String> = Vec::with_capacity(word.len());
        
        for (i, ch) in word.char_indices() {
            let (left, right) = ( &word[0..i], &word[i + 1..] );
            
            for n in 97..123 as u8 {
                let mut new_word:String = String::with_capacity(word.len() + 1);
            
                new_word.push_str(left);
                new_word.push(n as char);
                new_word.push_str(right);

                print!("{},", new_word);
                if self.contains(&new_word) { res.push(new_word);}
            }
            println!()
        }
        res
    }

    fn insertion(&self, word:&str) -> Vec<String> {
        let mut res:Vec<String> = Vec::with_capacity(16);

        for (i, ch) in word.char_indices() {
            let (left, right) = ( &word[0..i], &word[i..] );

            for n in 97..123 as u8 {
                let mut new_right:String = String::with_capacity(left.len() + right.len() + 1);
                new_right.push(n as char);
                new_right.push_str(right);

                let new_word = left.to_owned() + &new_right;
                if self.contains(&new_word) {
                    res.push(new_word);
                }
            }
        }
        

        res
    }
}



fn permutations(chars:&mut Vec<char>) -> Vec<String> {
    let n:usize = chars.len();
    let mut results:Vec<String> = Vec::new();
    let mut c:Vec<usize> = vec![0; n];

    let mut i = 0;
    while i < n {
        if c[i] < i {
            if i % 2 == 0 {
                chars.swap(0, i);
            } else {
                chars.swap(c[i], i);
            }

            results.push(chars.iter().collect());
            c[i] += 1;
            i = 0;
        } else {
            c[i] = 0;
            i += 1;
        }
    }
    results
}