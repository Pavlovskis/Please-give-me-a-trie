use crate::{node::NodeRefOps, tree::Trie};

pub trait AutoCompletable {
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
    /// let mut sufs:Vec<String> = t2.complete("an");
    /// sufs.sort();
    /// 
    /// assert_eq!(vec!["d".to_string(), "t".to_string(), "ymore".to_string()], sufs);
    /// ```
    /// # Time Complexity
    /// Takes <i>O</i>(n) time
    fn complete(&self, prefix:&str) -> Vec<String>;
}

impl AutoCompletable for Trie {
    fn complete(&self, prefix:&str) -> Vec<String> {
        let mut res:Vec<String> = Vec::with_capacity(8);

        let Some(cur) = self.go_to(prefix) else { return res; };
        
        for (_, node) in cur.as_ref().borrow().get_children() {
            let mut suf:Vec<String> = node.clone().preorder();
            res.append(&mut suf);
        }

        res
    }
}