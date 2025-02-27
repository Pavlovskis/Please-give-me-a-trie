use crate::tree::Trie;

pub trait ErrorCheckable {
    /// If word is not found in trie, try to find closest words to the provided string
    fn spelling_check(&self, word:&str) -> Option<Vec<String>>;
}

impl ErrorCheckable for Trie {
    // Needs some serious optimisation, I know
    fn spelling_check(&self, word:&str) -> Option<Vec<String>> {
        if self.contains(word) { return None; }

        let mut res:Vec<String> = Vec::with_capacity(4);

        let edits:[fn(&Trie, word:&str, distance:usize) -> Vec<String>; 4] = [
            Self::deletion, Self::transposition, Self::alteration, Self::insertion 
        ];

        for edit in edits {
            res.append( &mut edit(&self, word, 1) );
        }

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
    fn deletion(&self, word:&str, distance:usize) -> Vec<String>;

    // A string s has a transposition distance 1 from another string t if and only if t is equal to s with two adjacent 
    // characters transposed. The only strings that are a transposition Distance of 1 from “house” are “ohuse”, “huose”, 
    // “hosue” and “houes”. Note that if a string s has a transposition distance of 1 from another string t then |s| = |t|. 
    // Also, there are exactly | t | - 1 strings that are a transposition distance of 1 from t . 
    // The dictionary may contain 0 to n of the strings one transposition distance from t .
    fn transposition(&self, word:&str, distance:usize) -> Vec<String>;

    // A string s has an alteration distance 1 from another string t if and only if t is equal to s with exactly one character 
    // in s replaced by a lowercase letter that is not equal to the original letter. The only strings that are an alternation 
    // distance of 1 from “top” are “aop”, “bop”, …, “zop”, “tap”, “tbp”, …, “tzp”, “toa”, “tob”, …, and “toz”.
    // Note that if a string s has an alteration distance of 1 from another string t then |s| = |t|. 
    // Also, there are exactly 25* | t | strings that are an alteration distance of 1 from t . 
    // The dictionary may contain 0 to n of the strings one alteration distance from t .
    fn alteration(&self, word:&str, distance:usize) -> Vec<String>;

    // A string s has an insertion distance 1 from another string t if and only if t has a deletion distance of 1 from s . 
    // The only strings that are an insertion distance of 1 from “ask” are “aask”, “bask”, “cask”, … “zask”, “aask”, “absk”, “acsk”, … 
    // “azsk”, “asak”, “asbk”, “asck”, … “aszk”, “aska”, “askb”, “askc”, … “askz”. Note that if a string s has an insertion 
    // distance of 1 from another string t then |s| = |t|+1. Also, there are exactly 26* (|t|+1) strings that are an insertion 
    // distance of 1 from t . The dictionary may contain 0 to n of the strings one insertion distance from t .
    fn insertion(&self, word:&str, distance:usize) -> Vec<String>;
}

impl ErrorCheckEdits for Trie {
    
    fn deletion(&self, word:&str, distance:usize) -> Vec<String> {
        let mut res:Vec<String> = Vec::with_capacity(4);

        for i in 0..word.len() - distance as usize + 1 {
            let new_word:String = word[..i].to_string() + &word[i + distance as usize..];
            if self.contains(&new_word) { res.push(new_word); }
        }
        res
    }

    fn transposition(&self, word:&str, mut distance:usize) -> Vec<String> {
        if distance == 1 { distance += 1; }

        let mut res:Vec<String> = Vec::with_capacity(4);

        'outer: for d in 2..distance + 1 {
            for i in 0..word.len() - d + 1 {
                let mut chars_slice:Vec<char> = word[i..i + d].chars().collect();
            
                let permutations:Vec<String> = permutations(&mut chars_slice);
    
                for perm in permutations {
                    let new_word:String = word[0..i].to_string() + &perm + &word[i + d..];
                    if self.contains(&new_word) { 
                        res.push(new_word); 
                        break 'outer;
                    }
                }
            }
        }
        res
    }

    //todo
    fn alteration(&self, word:&str, _distance:usize) -> Vec<String> {
        let mut res:Vec<String> = Vec::with_capacity(word.len());
        
        for (i, _) in word.char_indices() {
            let (left, right) = ( &word[0..i], &word[i + 1..] );
            
            for n in 97..123 as u8 {
                let new_word:String = left.to_string() + &n.to_string() + right;

                if self.contains(&new_word) { res.push(new_word);}
            }
            println!()
        }
        res
    }

    //todo
    fn insertion(&self, word:&str, _distance:usize) -> Vec<String> {
        let mut res:Vec<String> = Vec::with_capacity(16);

        for (i, _) in word.char_indices() {
            let (left, right) = ( &word[0..i], &word[i..] );

            for n in 97..123 as u8 {
                let mut new_right:String = String::with_capacity(left.len() + right.len() + 1);
                new_right.push(n as char);
                new_right.push_str(right);

                let new_word = left.to_owned() + &new_right;
                if self.contains(&new_word) { res.push(new_word); }
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
