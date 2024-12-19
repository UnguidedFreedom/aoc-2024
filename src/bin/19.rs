use itertools::Itertools;
use std::collections::HashMap;

advent_of_code::solution!(19);

struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_word: bool,
}

impl TrieNode {
    fn new() -> Self {
        Self {
            children: HashMap::new(),
            is_word: false,
        }
    }

    fn insert(&mut self, word: &str) {
        let mut node = self;
        for c in word.chars() {
            node = node.children.entry(c).or_insert_with(TrieNode::new);
        }
        node.is_word = true;
    }

    fn is_word(&self) -> bool {
        self.is_word
    }

    fn get(&self, c: char) -> Option<&TrieNode> {
        self.children.get(&c)
    }
}

fn doable(line: &str, towels: &[&str]) -> bool {
    if line.is_empty() {
        return true;
    }

    for &towel in towels {
        if let Some(stripped) = line.strip_prefix(towel) {
            if doable(stripped, towels) {
                return true;
            }
        }
    }

    false
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let towels = lines.next().unwrap().split(", ").collect_vec();

    lines.next();

    let response = lines.filter(|line| doable(line, &towels)).count() as u64;

    Some(response)
}

fn doable_count(line: &str, towels: &TrieNode, memo: &mut HashMap<String, u64>) -> u64 {
    if line.is_empty() {
        return 1;
    }

    if let Some(&count) = memo.get(line) {
        return count;
    }

    let mut ways = 0;

    let mut trie = towels;

    for (i, char) in line.chars().enumerate() {
        if let Some(node) = trie.get(char) {
            if node.is_word() {
                ways += doable_count(&line[i + 1..], towels, memo);
            }
            trie = node;
        } else {
            break;
        }
    }

    memo.insert(line.to_string(), ways);

    ways
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();

    let mut trie = TrieNode::new();

    for towel in lines.next().unwrap().split(", ") {
        trie.insert(towel);
    }

    lines.next();

    let mut memo = HashMap::new();

    let response = lines.map(|line| doable_count(line, &trie, &mut memo)).sum();

    Some(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
