use std::{collections::HashMap, str::Chars};

type NodesMap = HashMap<char, Box<Node>>;

#[derive(Debug)]
pub struct Node {
    children: NodesMap,
    value: Option<String>,
}

impl Node {
    pub fn new() -> Self {
        Self {
            children: HashMap::new(),
            value: None,
        }
    }

    pub fn add(&mut self, mut chars: Chars, value: String) {
        if let Some(child_char) = chars.next() {
            if !self.children.contains_key(&child_char) {
                self.children.insert(child_char, Box::new(Node::new()));
            }
            let node = self.children.get_mut(&child_char).unwrap();
            node.add(chars, value);
        } else {
            self.value = Some(value);
        }
    }

    pub fn get(&self, mut prefix: Chars) -> Vec<String> {
        let mut result = Vec::new();
        if let Some(child_char) = prefix.next() {
            if let Some(child_node) = self.children.get(&child_char) {
                let mut child_result = child_node.get(prefix);
                result.append(&mut child_result);
            }
        } else {
            if let Some(value) = &self.value {
                result.push(value.clone());
            }
            for (_, child_node) in &self.children {
                let mut child_result = child_node.get(prefix.clone());
                result.append(&mut child_result);
            }
        }
        result
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

    pub fn add(&mut self, value: String) {
        let chars_value = value.clone();
        let mut chars = chars_value.chars();
        if let Some(root_char) = chars.next() {
            if !self.roots.contains_key(&root_char) {
                self.roots.insert(root_char, Box::new(Node::new()));
            }
            let node = self.roots.get_mut(&root_char).unwrap();
            node.add(chars, value);
        }
    }

    pub fn get(&self, prefix: String) -> Vec<String> {
        let mut result = Vec::new();
        let mut chars = prefix.chars();
        if let Some(root_char) = chars.next() {
            if let Some(root_node) = self.roots.get(&root_char) {
                let mut child_result = root_node.get(chars.clone());
                result.append(&mut child_result);
                if let (Some(value), None) = (&root_node.value, chars.next()) {
                    result.push(value.clone());
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut trie = Trie::new();
        trie.add("aaa".to_string());
        trie.add("aab".to_string());
        trie.add("aac".to_string());
        trie.add("aba".to_string());
        trie.add("abb".to_string());
        trie.add("aa".to_string());
        trie.add("ab".to_string());
        trie.add("a".to_string());

        let result = trie.get("".to_string());
        println!("{:#?}", result);
    }
}
