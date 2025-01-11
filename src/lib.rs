#![allow(unused)]

use std::sync::LazyLock;

use trie::TrieNode;

pub mod trie;

pub type Grid = [[Letter; 5]; 5];
pub type Word = Vec<((usize, usize), Option<char>)>;

static ROOT: LazyLock<TrieNode> =
    LazyLock::new(|| TrieNode::build(include_str!("../words.txt").lines()));

static SCORES: [u32; 26] = [
    1, // a
    4, // b
    5, // c
    3, // d
    1, // e
    5, // f
    3, // g
    4, // h
    1, // i
    7, // j
    6, // k
    3, // l
    4, // m
    2, // n
    1, // o
    4, // p
    8, // q
    2, // r
    2, // s
    2, // t
    4, // u
    5, // v
    5, // w
    7, // x
    4, // y
    8, // z
];

pub enum Modifier {
    DoubleLetter,
    TripleLetter,
    DoubleWord,
}

pub struct Letter {
    character: char,
    modifier: Option<Modifier>,
}

impl Letter {
    pub fn new(character: char, modifier: Option<Modifier>) -> Self {
        Self {
            character,
            modifier,
        }
    }
}

pub fn search(grid: &Grid, swap: usize) -> Vec<(Word, u32)> {
    let mut results: Vec<(Word, u32)> = find_words(grid, swap)
        .into_iter()
        .map(|word| {
            let score = find_score(&word, grid);
            (word, score)
        })
        .collect();

    results.sort_unstable_by_key(|(_, score)| std::cmp::Reverse(*score));
    results
}

pub fn word_to_string(word: &Word, grid: &Grid) -> String {
    word.iter()
        .map(|&((x, y), c)| {
            c.map(|c| c.to_ascii_uppercase())
                .unwrap_or(grid[y][x].character)
        })
        .collect()
}

fn find_score(word: &Word, grid: &Grid) -> u32 {
    let mut score = 0;
    let mut multiplier = 1;

    for &((x, y), c) in word {
        let letter = &grid[y][x];
        let character = c.unwrap_or(letter.character);
        let character_score = SCORES[character as usize - 'a' as usize];

        score += match letter.modifier {
            Some(ref modifier) => match modifier {
                Modifier::DoubleLetter => character_score * 2,
                Modifier::TripleLetter => character_score * 3,
                Modifier::DoubleWord => {
                    multiplier = 2;
                    character_score
                }
            },

            None => character_score,
        }
    }

    score * multiplier + if word.len() >= 6 { 10 } else { 0 }
}

fn find_words(grid: &Grid, swap: usize) -> Vec<Word> {
    fn find_words(words: &mut Vec<Word>, node: &TrieNode, curr: Word, swap: usize, grid: &Grid) {
        if node.leaf() {
            words.push(curr.clone());
        }

        let width = grid[0].len();
        let height = grid.len();

        let mut next: Vec<(usize, usize)> = match curr.last().copied() {
            Some(((x, y), _)) => {
                let deltas = [
                    (-1, -1),
                    (-1, 0),
                    (-1, 1),
                    (0, -1),
                    (0, 1),
                    (1, -1),
                    (1, 0),
                    (1, 1),
                ];

                deltas
                    .into_iter()
                    .filter_map(|(dx, dy)| {
                        let nx = x as isize + dx;
                        let ny = y as isize + dy;

                        (nx >= 0 && nx < width as isize && ny >= 0 && ny < height as isize)
                            .then(|| (nx as usize, ny as usize))
                    })
                    .collect()
            }

            None => (0..width)
                .flat_map(|i| (0..height).map(move |j| (i, j)))
                .collect(),
        };

        next.retain(|pos| !curr.iter().any(|(p, _)| p == pos));

        if swap > 0 {
            for (c, child) in node.children() {
                for &pos in &next {
                    let mut next = curr.clone();
                    next.push((pos, Some(c)));
                    find_words(words, child, next, swap - 1, grid);
                }
            }
        }

        for pos @ (x, y) in next {
            let c = grid[y][x].character;

            if let Some(child) = node.child(c) {
                let mut next = curr.clone();
                next.push((pos, None));
                find_words(words, child, next, swap, grid);
            }
        }
    }

    let mut words = Vec::new();
    find_words(&mut words, &ROOT, Vec::new(), swap, grid);
    words
}
