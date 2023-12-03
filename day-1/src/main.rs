use std::{error::Error, fs, collections::HashMap};

#[derive(Debug)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    value: char,
}

impl<'a> TrieNode {
    fn new(value: char) -> Self {
        Self { children: HashMap::new(), value }
    }

    fn iter(&'a self) -> TrieIterator {
        TrieIterator::new(self)
    }

    fn get(&self, c: &char) -> Option<&TrieNode> {
        self.children.get(c)
    }

    fn get_mut(&mut self, c: &char) -> Option<&mut TrieNode> {
        self.children.get_mut(c)
    }

    fn new_from_strs(strs: impl Iterator<Item = String>) -> Self {
        let mut head = Self::new(' ');
        for s in strs {
            head.insert_from_iter(&mut s.chars());
        }

        head
    }

    fn insert_from_iter(&mut self, chars: &mut impl Iterator<Item = char>) {
        if let Some(c) = chars.next() {
            match self.get_mut(&c) {
                Some(node) => node.insert_from_iter(chars),
                None => {
                    let new_node = Self::new(c);
                    self.children.insert(c, new_node);
                    self.get_mut(&c).unwrap().insert_from_iter(chars);
                }
            }
        } else {
            let new_node = Self::new('\0');
            self.children.insert('\0', new_node);
        }
    }

    fn get_num_from_digit_or_str(&self, chars: impl Iterator<Item = char>, reverse: bool) -> char {
        let mut iter = self.iter();
        let mut last_c = ' ';
        for c in chars {
            let mut next_word = iter.next(&c);

            if let Some(word) = next_word {
                if word == "" {
                    iter = self.iter();
                    if let Some(node) = self.get(&last_c) {
                        if node.get(&c).is_some() {
                            iter.next(&last_c);
                        }
                    }
                    next_word = iter.next(&c);
                }
            }
            match next_word {
                Some(word) => {
                    if word.len() > 0 {
                        return num_from_str(word, reverse);
                    } else {
                        iter = self.iter();
                    }
                }
                None => {}
            }

            if c.is_digit(10) {
                return c;
            }

            last_c = c;
        }
        panic!("Did not find number")
    }
}

struct TrieIterator<'a> {
    cur_node: &'a TrieNode,
    word: String,
}

impl<'a> TrieIterator<'a> {
    fn new(cur_node: &'a TrieNode) -> Self {
        Self { cur_node, word: String::new() }
    }

    fn next(&mut self, c: &char) -> Option<&str> {
        match self.cur_node.get(c) {
            Some(node) => {
                self.cur_node = node;
                self.word.push(node.value);
                None
            },
            None => {
                match self.cur_node.get(&'\0') {
                    Some(_) => Some(self.word.as_str()),
                    None => Some("")
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let num_trie = build_num_trie(false);
    let num_trie_rev = build_num_trie(true);

    let input = fs::read_to_string("input")?;
    let lines = input.split("\n").filter(|l| l.len() > 0);

    let mut calibration_values = Vec::new();
    for l in lines {
        let forward = l.chars();
        let reverse = l.chars().rev();

        let digit_1 = num_trie.get_num_from_digit_or_str(forward, false);
        let digit_2 = num_trie_rev.get_num_from_digit_or_str(reverse, true);

        let value = String::from_iter([digit_1, digit_2]).parse::<u32>()?;
        calibration_values.push(value);
    }

    println!("{}", calibration_values.iter().sum::<u32>());

    Ok(())
}


fn build_num_trie(reverse: bool) -> TrieNode {
    let mut numbers = vec![
        String::from("one"),
        String::from("two"),
        String::from("three"),
        String::from("four"),
        String::from("five"),
        String::from("six"),
        String::from("seven"),
        String::from("eight"),
        String::from("nine"),
        String::from("zero"),
    ];
    if reverse {
        numbers = numbers.iter()
            .map(|n| String::from_iter(n.chars().rev()))
            .collect();
    }
    TrieNode::new_from_strs(numbers.into_iter())
}

fn num_from_str(s: &str, reverse: bool) -> char {
    let s = if reverse {
        String::from_iter(s.chars().rev())
    } else {
        String::from(s)
    };
    match s.as_str() {
        "one" => '1',
        "two" => '2',
        "three" => '3',
        "four" => '4',
        "five" => '5',
        "six" => '6',
        "seven" => '7',
        "eight" => '8',
        "nine" => '9',
        "zero" => '0',
        _ => unreachable!("Not a number! -> {}", s)
    }
}
