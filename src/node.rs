use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub(crate) const ALPHABET_SIZE:usize = 26;

pub type NodeRef = Rc<RefCell<Node>>;

pub trait NodeRefOps {
    fn preorder(&self) -> Vec<String>;
}

impl NodeRefOps for NodeRef {
    /// Traverse the trie, fetching all the words that origitate at the root
    fn preorder(&self) -> Vec<String> {
        let mut words:Vec<String> = Vec::with_capacity(8);
        let mut stack:Vec<(usize, NodeRef)> = Vec::with_capacity(16);
        
        let mut cur_lvl:usize = 1;
        stack.push( (cur_lvl, self.clone()) );
        
        let mut word:String = String::with_capacity(8);

        while let Some( (_, n_ref) ) = stack.pop() {
            let node = n_ref.as_ref().borrow_mut();
            word.push(node.val);
            
            if node.get_children().is_empty() && node.is_end_of_word {
                let Some( (plvl, _) ) = stack.last() else { 
                    words.push(word.clone());
                    continue;
                };
                
                words.push(word.clone());
                word = word.split_at(plvl - 1).0.to_string();
                cur_lvl = *plvl;
                continue;
            }
            if node.is_end_of_word {
                words.push(word.clone());
            }

            for (_, child) in node.get_children() {
                stack.push( (cur_lvl + 1, child.clone()) );
            }
            cur_lvl += 1;
        }
        words
    }
}

#[derive(Debug)]
pub struct Node {
    pub val:char,
    children:HashMap<char, NodeRef>,
    pub is_end_of_word:bool
}

impl Node {
    pub fn new(val:char, is_end_of_word:bool) -> Self {
        Self { 
            val,
            children: HashMap::with_capacity(ALPHABET_SIZE / 2), 
            is_end_of_word
        }
    }

    pub fn to_ref(self) -> NodeRef {
        Rc::new(RefCell::new(self))
    }

    pub fn children_size(&self) -> usize {
        self.children.len()
    }

    pub fn get_children(&self) -> &HashMap<char, NodeRef> {
        &self.children
    }

    pub fn get_child(&self, c:char) -> Option<&NodeRef> {        
        self.children.get(&c)
    }

    pub fn new_child(&mut self, c:char, is_end_of_word:bool) -> NodeRef {
        let new:NodeRef = Self::new(c, is_end_of_word).to_ref();
        self.children.insert(c, new.clone());

        new
    }

    pub fn remove_child(&mut self, c:char) {
        self.children.remove(&c);
    }

}