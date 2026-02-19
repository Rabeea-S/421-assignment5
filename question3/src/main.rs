use std::collections::HashMap;

#[derive(Debug)]
struct TrieNode {
    chs: HashMap<char, TrieNode>,
    value: Option<i32>,
}

#[derive(Debug)]
struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Trie {
        Trie {
            root: TrieNode {
                chs: HashMap::new(),
                value: None,
            },
        }
    }

    fn add_string(&mut self, string: String, value: i32) {
        let mut current_node = &mut self.root;
        for c in string.chars() {
            current_node = current_node.chs
                .entry(c)
                .or_insert(TrieNode {
                    chs: HashMap::new(),
                    value: None,
                });
        }
        current_node.value = Some(value);
    }

    fn length(&self) -> usize {
    }

    fn iter(&self) -> Vec<(char, Option<i32>)> {
    }

    fn find(&self, key: &String) -> Option<&TrieNode> {
    }

    fn delete(&mut self, key: &String) -> Option<i32> {
    }
}

fn main() {
    let mut trie = Trie::new();
    trie.add_string("B".to_string(), 1);
    trie.add_string("Bar".to_string(), 2);
    println!("{:#?}", trie);
}

// References:
// https://docs.rs/radix_trie/0.0.9/radix_trie/struct.Trie.html#method.len