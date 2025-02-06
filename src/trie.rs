#![allow(dead_code)]

use std::{collections::HashMap, fmt::Display, fs::File, io::{BufRead, BufReader}};

use super::node::{Node, NodeRef, NodeRefOps};

pub struct Trie {
    pub words:usize,
    pub root:NodeRef,
}

impl Display for Trie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (ch, v) in self.get_words() {
            println!("[ {} | {} ] => {:?}", ch, v.len(), v);
        }
        write!(f, "")
    }
}

impl Trie {
    /// Create new empty [`Trie`].
    /// 
    /// # Examples
    /// ```
    /// let mut trie:Trie = Trie::new();
    /// ```
    pub fn new() -> Self {
        Self { 
            words:0, 
            root: Node::new( '\0', false).to_ref()
        }
    }
    
    /// Get all the words in the trie, categorizes them by the first letter of the word
    /// 
    /// ## Example
    /// ```
    /// let mut trie:Trie = Trie::new();
    /// 
    /// trie.inser("this");
    /// trie.inser("that");
    /// 
    /// let words:Hashmap<char, Vec<String>> = trie.get_words();
    /// ```
    pub fn get_words(&self) -> HashMap<char, Vec<String>> {
        let mut res:HashMap<char, Vec<String>> = HashMap::with_capacity( self.root.as_ref().borrow().children_size() );

        for (ch, node) in self.root.as_ref().borrow().get_children() {
            let words:Vec<String> = node.clone().preorder();
            res.insert(*ch, words);
        }

        res
    }

    /// Get only the words that begin with the provided suffix, excludes the provided suffix from the resulting words, 
    /// returns unsorted array of suffixes.
    /// # Examples
    /// ```
    /// let mut trie:Trie = Trie::new();
    /// 
    /// trie.inser("and");
    /// trie.inser("ant");
    /// trie.inser("anymore");
    /// 
    /// let mut sufs:Vec<String> = t2.get_suffix_of("an");
    /// sufs.sort();
    /// 
    /// assert_eq!(vec!["d".to_string(), "t".to_string(), "ymore".to_string()], sufs);
    /// ```
    /// # Time Complexity
    /// Takes <i>O</i>(n) time
    pub fn get_suffix_of(&self, suf:&str) -> Vec<String> {
        let mut res:Vec<String> = Vec::with_capacity(8);

        let Some(cur) = self.go_to(suf) else { return res; };
        
        for (_, node) in cur.as_ref().borrow().get_children() {
            let mut suf:Vec<String> = node.clone().preorder();
            res.append(&mut suf);
        }

        res
    }

    /// Check if the word is in trie, stops as soon as possible if one of the char differs
    /// 
    /// # Examples
    /// ```
    /// let mut trie:Trie = Trie::new();
    /// 
    /// trie.inser("word");
    /// 
    /// assert_eq!(trie.contains("word"), true);
    /// ```
    /// # Time Complexity
    /// Takes <i>O</i>(1) time
    pub fn contains(&self, word:&str) -> bool {
        let Some(cur) = self.go_to(word) else {
            return false;
        };
        if cur.as_ref().borrow().is_end_of_word { return true; }

        false
    }

    /// Insert word into prefix tree, if word is already in the tree ignores it
    /// 
    /// # Examples
    /// ```
    /// let mut trie:Trie = Trie::new();
    /// 
    /// trie.inser("word");
    /// ```
    /// # Time Complexity
    /// Takes <i>O</i>(1) time
    pub fn insert(&mut self, word:&str) {
        let mut cur:NodeRef = self.root.clone();
        
        let mut is_last_char:bool = false;
        for (i, ch) in word.char_indices() {
            if i == word.len() - 1 { is_last_char = true; }
            
            let ochild:Option<NodeRef> = cur.as_ref().borrow_mut().get_child(ch);
            if let Some(node) = ochild { // if current node has the child
                // println!("{:?}", cur.as_ref().borrow().get_children());
                if !is_last_char {
                    cur = node;
                    continue;
                }
                if !node.as_ref().borrow().is_end_of_word {
                    node.as_ref().borrow_mut().is_end_of_word = true;
                    self.words += 1;
                }
                return;
            } else { // if current not doesn't have the child
                // println!("{:?}", cur.as_ref().borrow().get_children());
                let new_child:NodeRef = cur.as_ref().borrow_mut().new_child(ch, is_last_char);
                
                if is_last_char { 
                    self.words += 1;
                    return;
                }
                cur = new_child;
            }
        }
    }

    /// Remove word from prefix tree, if word doesn't exist stops as soon as possible
    /// 
    /// # Examples
    /// ```
    /// let mut trie:Trie = Trie::new();
    /// 
    /// trie.inser("word");
    /// 
    /// assert_eq!(trie.contains("word"), true);
    /// 
    /// trie.remove("word");
    /// 
    /// assert_eq!(trie.contains("word"), false);
    /// ```
    /// # Time Complexity
    /// Takes <i>O</i>(1) time
    pub fn remove(&mut self, word:&str) {
        let mut cur:NodeRef = self.root.clone();
        let mut cur_lvl:usize = 1;

        // When going down in trie, store last node that is end of a word or has multiple children
        let mut last:Option<(usize, NodeRef)> = None;
        
        let mut is_last_char:bool = false;
        for (i, ch) in word.char_indices() {
            if i == word.len() - 1 { is_last_char = true; }

            let ochild:Option<NodeRef> = cur.as_ref().borrow().get_child(ch);
            if let Some(node) = ochild {
                if is_last_char {
                    if node.as_ref().borrow().children_size() > 0 {
                        node.as_ref().borrow_mut().is_end_of_word = false;
                    } else { Self::cut_child(last, word); }
                    self.words -= 1;
                    return;
                }
                if node.as_ref().borrow().is_end_of_word || node.as_ref().borrow().children_size() > 1 {
                    last = Some( (cur_lvl, node.clone()) );
                }
                cur = node;
                cur_lvl += 1;
            } else { break; }
        }

        Self::cut_child(last, word);
        self.words -= 1;
    }
    // Specific for remove function
    fn cut_child(last:Option<(usize, NodeRef)>, word:&str) {
        let Some( (lvl, node_to_change) ) = last else { return; };
        
        if let Some(child) = word.split_at(lvl + 1).0[lvl..lvl + 1].chars().next() {
            println!("{}", child);
            node_to_change.as_ref().borrow_mut().remove_child(child);
        }
    } 

    /// Try find a node that coincides with end of the word
    fn go_to(&self, word:&str) -> Option<NodeRef> {
        let mut cur:NodeRef = self.root.clone();

        let mut pchars = word.chars();
        while let Some(ch) = pchars.next() {
            let ochild = cur.as_ref().borrow().get_child(ch);
            
            if let Some(node) = ochild {
                cur = node;
            } else { return None; }
        }
        Some(cur)
    }

}

pub struct TrieBuilder {
    reader:Option<BufReader<File>>,
    lines:Option<usize>
}

impl TrieBuilder {
    /// Specify number of lines that you want insert into the prefix tree, if function is not used it is assumed that all the 
    /// lines will be inserted in the tree
    /// # Examples 
    /// ```
    /// let trie:Trie = TrieBuilder::from(reader).lines(100).build();
    pub fn lines(mut self, n:usize) -> Self {
        self.lines = Some(n);
        self
    }

    pub fn build(self) -> Trie {
        let mut trie:Trie = Trie::new();

        let Some(reader) = self.reader else { return trie; };

        let lines = if let Some(l) = self.lines { l } else { usize::MAX };

        for (i, raw) in reader.lines().enumerate() {
            if i >= lines { break; }
            
            let Ok(line) = raw else { 
                eprintln!("Error processing line {}", i);
                continue;
            };

            trie.insert(line.trim());
        }

        trie
    }
}

impl From<BufReader<File>> for TrieBuilder {

    /// Iterate through a file and insert all the words into prefix tree
    /// 
    /// # Examples 
    /// ```
    /// let file = File::open("./file/path.txt")
    ///     .expect("File not found");
    ///
    /// let reader = BufReader::new(file);
    /// 
    /// let trie:Trie = TrieBuilder::from(reader).build();
    /// ```
    fn from(value: BufReader<File>) -> Self {
        TrieBuilder { reader: Some(value), lines: None }
    }
}