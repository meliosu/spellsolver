use std::{array, collections::HashMap};

use askama::Template;
use axum::response::IntoResponse;
use axum::routing;
use axum::{extract::Query, http::StatusCode};

use spellcast::{word_to_string, Grid, Letter, Modifier, Word};

#[tokio::main]
async fn main() {
    env_logger::init();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:1337").await.unwrap();
    let router = axum::Router::new()
        .route("/", routing::get(index))
        .route("/find", routing::get(find));

    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();
}

async fn index() -> impl IntoResponse {
    Index
}

fn parse_grid(query: HashMap<String, String>) -> Result<Grid, StatusCode> {
    let mut grid: Grid = array::from_fn(|_| array::from_fn(|_| Letter::new(' ', None)));

    let parse = |key: &str| key.parse::<usize>().ok();

    for (key, value) in query {
        if let Some(cell) = key.strip_suffix("DL").and_then(parse) {
            let (x, y) = (cell % 5, cell / 5);

            grid[y][x].modifier = Some(Modifier::DoubleLetter);
        } else if let Some(cell) = key.strip_suffix("DW").and_then(parse) {
            let (x, y) = (cell % 5, cell / 5);

            grid[y][x].modifier = Some(Modifier::DoubleWord);
        } else if let Some(cell) = parse(&key) {
            let (x, y) = (cell % 5, cell / 5);

            let Some(c) = value.chars().next() else {
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            };

            grid[y][x].character = c.to_ascii_lowercase();
        } else {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    for row in &grid {
        for letter in row {
            if !('a'..='z').contains(&letter.character) {
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        }
    }

    Ok(grid)
}

async fn find(Query(query): Query<HashMap<String, String>>) -> Result<Answer, StatusCode> {
    let grid = parse_grid(query)?;
    let (word, score) = spellcast::search(&grid, 0, 1)[0].clone();

    log::info!("best word: {}", word_to_string(&word, &grid));

    Ok(Answer { grid, word, score })
}

#[derive(Template)]
#[template(path = "index.html")]
struct Index;

#[derive(Template)]
#[template(path = "answer.html")]
struct Answer {
    grid: Grid,
    word: Word,
    score: u32,
}

impl Answer {
    fn char_picked_replaced_index(&self, index: &usize) -> (char, bool, bool, usize) {
        let (x, y) = (index % 5, index / 5);

        match self
            .word
            .iter()
            .enumerate()
            .find_map(|(i, &((nx, ny), c))| (nx == x && ny == y).then_some((i, c)))
        {
            Some((index, Some(c))) => (c, true, true, index),

            Some((index, None)) => {
                let c = self.grid[y][x].character;
                (c, true, false, index)
            }

            None => {
                let c = self.grid[y][x].character;
                (c, false, false, 0)
            }
        }
    }
}
