use std::time::Instant;

use spellcast::Modifier::*;
use spellcast::{search, word_to_string, Letter};

fn main() {
    let grid = [
        [
            ('r', Some(DoubleLetter)),
            ('r', None),
            ('r', None),
            ('p', None),
            ('g', None),
        ],
        [
            ('n', None),
            ('s', None),
            ('o', None),
            ('a', None),
            ('k', None),
        ],
        [
            ('i', None),
            ('s', None),
            ('u', None),
            ('e', None),
            ('a', None),
        ],
        [
            ('e', None),
            ('r', None),
            ('o', None),
            ('e', None),
            ('e', None),
        ],
        [
            ('d', None),
            ('w', None),
            ('a', None),
            ('n', None),
            ('n', None),
        ],
    ];

    let grid = grid.map(|a| a.map(|(c, m)| Letter::new(c, m)));
    let top = 50;

    _ = std::hint::black_box(search(&grid, 0, top));

    for swaps in 0..=3 {
        let start = Instant::now();
        let words = search(&grid, swaps, top);

        for (word, score) in words {
            println!("{} {}", word_to_string(&word, &grid), score);
        }

        println!("{swaps} swaps: {:?}\n\n", start.elapsed());
    }
}
