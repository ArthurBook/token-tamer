use pyo3::prelude;

#[prelude::pyclass]
pub struct CharTrie {
    children: std::collections::HashMap<char, CharTrie>,
    is_end_of_word: bool,
}

#[prelude::pymethods]
impl CharTrie {
    #[new]
    pub fn new() -> Self {
        Self {
            children: std::collections::HashMap::new(),
            is_end_of_word: false,
        }
    }

    pub fn insert(&mut self, word: &str) {
        let mut node = self;
        for ch in word.chars() {
            node = node.children.entry(ch).or_insert(CharTrie::new());
        }
        node.is_end_of_word = true;
    }

    pub fn search(&self, word: &str) -> bool {
        let mut node = self;
        for ch in word.chars() {
            if let Some(next_node) = node.children.get(&ch) {
                node = next_node;
            } else {
                return false;
            }
        }
        return node.is_end_of_word;
    }

    pub fn next_allowed_tokens(&self, prefix: &str) -> Vec<char> {
        let mut node = self;
        for ch in prefix.chars() {
            if let Some(next_node) = node.children.get(&ch) {
                node = next_node;
            } else {
                return vec![]; // No tokens found for the given prefix
            }
        }
        return node.children.keys().cloned().collect();
    }
}

#[prelude::pymodule]
fn tokentamer(_py: prelude::Python, m: &prelude::PyModule) -> prelude::PyResult<()> {
    m.add_class::<CharTrie>()?;
    Ok(())
}
