#![allow(unused)]

use std::sync::LazyLock;

use arrayvec::ArrayVec;
use top::Top;
use trie::TrieNode;

pub mod top;
pub mod trie;

const WIDTH: usize = 5;
const HEIGHT: usize = 5;

pub type Grid = [[Letter; WIDTH]; HEIGHT];
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
    pub character: char,
    pub modifier: Option<Modifier>,
}

impl Letter {
    pub fn new(character: char, modifier: Option<Modifier>) -> Self {
        Self {
            character,
            modifier,
        }
    }
}

pub fn search(grid: &Grid, swap: usize, top: usize) -> Vec<(Word, u32)> {
    fn find_words(
        words: &mut Top<(Word, u32)>,
        node: &TrieNode,
        word: &mut Word,
        swap: usize,
        grid: &Grid,
    ) {
        if node.is_complete() {
            let score = find_score(word, grid);

            if !words.worst().is_some_and(|(_, worst)| score <= *worst) {
                let word = word.clone();
                words.insert_by_key((word, score), |(_, score)| std::cmp::Reverse(*score));
            }
        }

        if node.is_leaf() {
            return;
        }

        let not_in_word = |pos: &(usize, usize)| !word.iter().any(|(p, _)| p == pos);

        let next: ArrayVec<(usize, usize), 25> = match word.last().copied() {
            Some(((x, y), _)) => [
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ]
            .into_iter()
            .filter_map(|(dx, dy)| {
                let nx = x as isize + dx;
                let ny = y as isize + dy;

                (nx >= 0 && nx < WIDTH as isize && ny >= 0 && ny < HEIGHT as isize)
                    .then(|| (nx as usize, ny as usize))
            })
            .filter(not_in_word)
            .collect(),

            None => (0..WIDTH)
                .flat_map(|i| (0..HEIGHT).map(move |j| (i, j)))
                .filter(not_in_word)
                .collect(),
        };

        if swap > 0 {
            for (c, child) in node.children() {
                for &pos @ (x, y) in &next {
                    if grid[y][x].character == c {
                        continue;
                    }

                    word.push((pos, Some(c)));
                    find_words(words, child, word, swap - 1, grid);
                    word.pop();
                }
            }
        }

        for pos @ (x, y) in next {
            let c = grid[y][x].character;

            if let Some(child) = node.child(c) {
                word.push((pos, None));
                find_words(words, child, word, swap, grid);
                word.pop();
            }
        }
    }

    let mut words = Top::new(top);
    let mut word = Word::new();
    find_words(&mut words, &ROOT, &mut word, swap, grid);
    words.into_inner()
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
