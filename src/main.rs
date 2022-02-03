use rusqlite::Connection;
use rusqlite::OpenFlags;
use std::collections::BTreeSet;
use std::env;
use std::fs::File;
use std::io;
use std::io::BufRead;

struct WordWithCount {
    word: String,
    count: u32,
}

fn main() {
    let mut args = env::args();
    args.next(); // skip program name

    let file_path = args.next().unwrap();
    let db_path = args.next().unwrap();

    let known_words = read_known_words(&file_path);

    let conn = Connection::open_with_flags(&db_path, OpenFlags::SQLITE_OPEN_READ_ONLY).unwrap();

    let mut stmt = conn
        .prepare(
            "
            SELECT word, COUNT(*)
            FROM Word JOIN WordInSentence ON Word.id = WordInSentence.WordId
            GROUP by WordId
            ORDER BY 2 DESC
            ",
        )
        .unwrap();
    let word_with_count_iter = stmt
        .query_map([], |row| {
            Ok(WordWithCount {
                word: row.get(0).unwrap(),
                count: row.get(1).unwrap(),
            })
        })
        .unwrap();

    for item in word_with_count_iter {
        let word_with_count = item.unwrap();
        let word = word_with_count.word;
        if known_words.contains(&word) || word.is_ascii() {
            continue;
        }

        println!("{:2} {}", word_with_count.count, word);
    }
}

fn read_known_words(path: &str) -> BTreeSet<String> {
    let file = File::open(&path).unwrap();
    let mut known_words = BTreeSet::new();

    let lines = io::BufReader::new(file).lines();
    lines.flat_map(|line| line).for_each(|line| {
        known_words.insert(line);
    });

    known_words
}
