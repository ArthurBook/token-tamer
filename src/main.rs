use std::collections::HashMap;

struct TrieNode<T> {
    children: HashMap<T, TrieNode<T>>,
    is_end_of_sequence: bool,
}

impl<T: Eq + std::hash::Hash + Clone> TrieNode<T> {
    // Create a new Trie node
    fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            is_end_of_sequence: false,
        }
    }

    // Insert a sequence into the trie
    fn insert(&mut self, sequence: &[T]) {
        let mut node: &mut TrieNode<T> = self;
        for token in sequence {
            node = node
                .children
                .entry(token.clone())
                .or_insert_with(TrieNode::new);
        }
        node.is_end_of_sequence = true; // Mark the end of a sequence
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trie_insert() {
        let mut trie = TrieNode::new();
        trie.insert(&vec!['a', 'b', 'c']);
        trie.insert(&vec!['a', 'b', 'd']);
    }
}

fn main() {
    let mut trie: TrieNode<i32> = TrieNode::new();
    trie.insert(&vec![1, 2, 3]);
    trie.insert(&vec![1, 2, 4]);
    trie.insert(&vec![1, 2, 5]);
}
