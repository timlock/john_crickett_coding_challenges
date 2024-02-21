use std::{cmp::Reverse, collections::{BinaryHeap, HashMap}};

pub struct HuffmanTree {
root: Option<Node>,
}
impl HuffmanTree {
pub fn build_table(&self) -> HashMap<char, String> {
    let table = HashMap::new();
    match &self.root {
        Some(root) => root.build_table(String::new(), table),
        None => table,
    }
}
pub fn byte_size(&self) -> usize {
    match &self.root {
        Some(root) => root.byte_size(0),
        None => 0,
    }
}
}

impl From<&str> for HuffmanTree {
fn from(text: &str) -> Self {
    let mut heap = determine_frequency(text);
    while heap.len() > 1 {
        let first = heap.pop().unwrap();
        let second = heap.pop().unwrap();
        let merged = first.0.merge(second.0);
        heap.push(Reverse(merged));
    }
    let root = match heap.pop() {
        Some(node) => Some(node.0),
        None => None,
    };
    HuffmanTree { root }
}
}

#[derive(Debug, Eq)]
enum Node {
Internal {
    frequency: u32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
},
Leaf {
    frequency: u32,
    character: char,
},
}
impl Node {
fn byte_size(&self, mut total: usize) -> usize {
    match self {
        Node::Internal { left, right, .. } => {
            if let Some(left) = left {
                total = left.byte_size(total);
            }
            if let Some(right) = right {
                total = right.byte_size(total);
            }
        }
        Node::Leaf { frequency, .. } => {
            total += *frequency as usize;
        }
    };
    total
}
fn frequency(&self) -> u32 {
    match self {
        Node::Internal { frequency, .. } => *frequency,
        Node::Leaf { frequency, .. } => *frequency,
    }
}
fn merge(self, other: Self) -> Self {
    Node::Internal {
        frequency: self.frequency() + other.frequency(),
        left: Some(Box::new(self)),
        right: Some(Box::new(other)),
    }
}

fn build_table(&self, code: String, mut table: HashMap<char, String>) -> HashMap<char, String> {
    match self {
        Node::Internal { left, right, .. } => {
            if let Some(left) = left {
                table = left.build_table(code.clone() + "0", table);
            }
            if let Some(right) = right {
                table = right.build_table(code.clone() + "1", table);
            }
        }
        Node::Leaf { character, .. } => {
            table.insert(*character, code);
        }
    };
    table
}
}

impl PartialEq for Node {
fn eq(&self, other: &Self) -> bool {
    self.frequency() == other.frequency()
}
}

impl PartialOrd for Node {
fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    self.frequency().partial_cmp(&other.frequency())
}
}

impl Ord for Node {
fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    self.frequency().cmp(&other.frequency())
}
}

fn determine_frequency(text: &str) -> BinaryHeap<Reverse<Node>> {
let mut counter: HashMap<char, u32> = HashMap::new();
for c in text.chars() {
    match counter.get_mut(&c) {
        Some(value) => *value += 1,
        None => {
            counter.insert(c, 1);
        }
    };
}
counter
    .into_iter()
    .map(|(character, frequency)| {
        Reverse(Node::Leaf {
            frequency,
            character,
        })
    })
    .collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_table() {
        let c = (0..32).map(|_| 'c').collect::<String>();
        let d = (0..42).map(|_| 'd').collect::<String>();
        let e = (0..120).map(|_| 'e').collect::<String>();
        let k = (0..7).map(|_| 'k').collect::<String>();
        let l = (0..43).map(|_| 'l').collect::<String>();
        let m = (0..24).map(|_| 'm').collect::<String>();
        let u = (0..37).map(|_| 'u').collect::<String>();
        let z = (0..2).map(|_| 'z').collect::<String>();
        let text = c + &d + &e + &k + &l + &m + &u + &z;
        let tree = HuffmanTree::from(text.as_str());
        let table = tree.build_table();
        assert!(!table.is_empty());
        assert_eq!(*table.get(&'c').unwrap(), "1110");
        assert_eq!(*table.get(&'d').unwrap(), "101");
        assert_eq!(*table.get(&'e').unwrap(), "0");
        assert_eq!(*table.get(&'k').unwrap(), "111101");
        assert_eq!(*table.get(&'l').unwrap(), "110");
        assert_eq!(*table.get(&'m').unwrap(), "11111");
        assert_eq!(*table.get(&'u').unwrap(), "100");
        assert_eq!(*table.get(&'z').unwrap(), "111100");
    }
}
