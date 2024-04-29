use pyo3::prelude;

#[prelude::pyclass]
pub struct Trie {
    children: std::collections::HashMap<char, Trie>,
    is_end_of_word: bool,
}

#[prelude::pymethods]
impl Trie {
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
            node = node.children.entry(ch).or_insert(Trie::new());
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
        node.is_end_of_word
    }
}

#[prelude::pymodule]
fn triehard(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Trie>()?;
    Ok(())
}