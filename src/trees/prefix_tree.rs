use std::{collections::HashMap, str::Chars};

type NodesMap = HashMap<char, Box<Node>>;

#[derive(Debug)]
pub struct Node {
    children: NodesMap,
}

impl Node {
    pub fn new() -> Self {
        Self {
            children: HashMap::new(),
        }
    }

    pub fn add(&mut self, mut chars: Chars) {
        if let Some(child_char) = chars.next() {
            if !self.children.contains_key(&child_char) {
                self.children.insert(child_char, Box::new(Node::new()));
            }
            let node = self.children.get_mut(&child_char).unwrap();
            node.add(chars);
        }
    }
}

#[derive(Debug)]
pub struct Trie {
    roots: NodesMap,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            roots: HashMap::new(),
        }
    }

    pub fn add(&mut self, str: String) {
        let mut chars = str.chars();
        if let Some(root_char) = chars.next() {
            if !self.roots.contains_key(&root_char) {
                self.roots.insert(root_char, Box::new(Node::new()));
            }
            let node = self.roots.get_mut(&root_char).unwrap();
            node.add(chars);
        }
        println!("{:#?}", self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut trie = Trie::new();
        trie.add("hello".to_string());
        trie.add("hells".to_string());
        trie.add("hello".to_string());
        trie.add("hellow".to_string());
    }
}
