use std::fs::{self, File};
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

use std::collections::BTreeSet;

// TODO: performance (it could be an int)
type Genre = String;

#[derive(Debug)]
struct Movie {
    id: usize,
    title: String,
    genres: BTreeSet<Genre>,
}

impl From<String> for Movie {
    fn from(s: String) -> Self {
        let mut values = s.split(',');
        let id = values.next().unwrap().parse().unwrap();
        let title = values.next().unwrap().into();
        let mut genres: BTreeSet<_> = values
            .next()
            .unwrap()
            .split('|')
            .map(|a| a.to_owned())
            .collect();

        if genres.contains("(no genres listed)") {
            genres.remove("(no genres listed)");
        }

        Self { id, title, genres }
    }
}

type Parts = Vec<String>;

#[derive(Debug, Default)]
struct Query {
    projection: Option<Parts>, // fields/attributes
    selection: Option<Parts>,  // conditions
    scan: Option<Parts>,       // tables
}

use serde_json::Value;

impl From<Value> for Query {
    fn from(json: Value) -> Self {
        let mut query = Query::default();
        for clause in json.as_array().unwrap() {
            if clause[0] == "PROJECTION" {
                // lol
                query.projection = Some(
                    clause[1]
                        .as_array()
                        .unwrap()
                        .into_iter()
                        .map(|a| a.as_str().unwrap().to_owned())
                        .collect(),
                );
            }
            if clause[0] == "SELECTION" {
                // lol
                query.selection = Some(
                    clause[1]
                        .as_array()
                        .unwrap()
                        .into_iter()
                        .map(|a| a.as_str().unwrap().to_owned())
                        .collect(),
                );
            }
            if clause[0] == "SCAN" {
                // lol
                query.scan = Some(
                    clause[1]
                        .as_array()
                        .unwrap()
                        .into_iter()
                        .map(|a| a.as_str().unwrap().to_owned())
                        .collect(),
                );
            }
        }
        query
    }
}

fn main() {
    let query = fs::read_to_string("query.json").unwrap();
    let json: Value = serde_json::from_str(&query).unwrap();
    let query = Query::from(json);
    dbg!(query);

    let lines = read_lines("./ml-20m/movies.csv").unwrap().skip(1);
    let mut movies = vec![];

    for line in lines {
        let line = line.unwrap();
        // println!("{line}");
        let movie = Movie::from(line);
        // println!("{movie:?}");
        movies.push(movie);
    }
}
