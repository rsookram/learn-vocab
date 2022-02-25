use anyhow::anyhow;
use anyhow::Result;
use clap::arg;
use clap::command;
use clap::Command;
use owo_colors::OwoColorize;
use owo_colors::Stream;
use rusqlite::Connection;
use rusqlite::OpenFlags;
use std::collections::BTreeSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

struct WordWithCount {
    word: String,
    count: u32,
}

fn main() -> Result<()> {
    let matches = command!()
        .subcommand_required(true)
        .arg(arg!(--color <COLOR>).required(false))
        .subcommand(
            Command::new("unknown")
                .arg(arg!(<LEARNED_PATH>))
                .arg(arg!(<DB_PATH>)),
        )
        .subcommand(
            Command::new("sentences")
                .arg(arg!(<DB_PATH>))
                .arg(arg!(<WORD>)),
        )
        .subcommand(
            Command::new("compact")
                .arg(arg!(<LEARNED_PATH>))
                .arg(arg!(<DB_PATH>)),
        )
        .subcommand(Command::new("n-plus-one").arg(arg!(<DB_PATH>)))
        .get_matches();

    match matches.value_of("color") {
        Some("always") => owo_colors::set_override(true),
        Some("never") => owo_colors::set_override(false),
        Some("auto") => {}
        Some(value) => {
            return Err(anyhow!(
                "argument for --color must be auto, always, or never, but found `{}`",
                value
            ))
        }
        None => {}
    }

    match matches.subcommand() {
        Some(("unknown", sub_matches)) => {
            let learned_path = require_arg(sub_matches, "LEARNED_PATH");
            let db_path = require_arg(sub_matches, "DB_PATH");

            command_unknown(learned_path, db_path)
        }
        Some(("sentences", sub_matches)) => {
            let db_path = require_arg(sub_matches, "DB_PATH");
            let word = require_arg(sub_matches, "WORD");

            command_sentences(db_path, word)
        }
        Some(("compact", sub_matches)) => {
            let learned_path = require_arg(sub_matches, "LEARNED_PATH");
            let db_path = require_arg(sub_matches, "DB_PATH");

            command_compact(learned_path, db_path)
        }
        Some(("n-plus-one", sub_matches)) => {
            let db_path = require_arg(sub_matches, "DB_PATH");

            command_n_plus_1(db_path)
        }
        _ => unreachable!("subcommand is required"),
    }
}

fn require_arg<'a>(matches: &'a clap::ArgMatches, name: &'a str) -> &'a str {
    matches.value_of(name).expect("required arg")
}

fn command_unknown(learned_path: &str, db_path: &str) -> Result<()> {
    let known_words = read_known_words(learned_path)?;

    let conn = Connection::open_with_flags(db_path, OpenFlags::SQLITE_OPEN_READ_ONLY)?;

    let mut stmt = conn.prepare(
        "
        SELECT word, COUNT(*)
        FROM Word JOIN WordInSentence ON Word.id = WordInSentence.wordId
        GROUP by wordId
        ORDER BY 2 DESC
        ",
    )?;
    let word_with_count_iter = stmt.query_map([], |row| {
        Ok(WordWithCount {
            word: row.get(0)?,
            count: row.get(1)?,
        })
    })?;

    for item in word_with_count_iter {
        let word_with_count = item?;
        let word = word_with_count.word;
        if known_words.contains(&word) || word.is_ascii() {
            continue;
        }

        println!("{:2} {}", word_with_count.count, word);
    }

    Ok(())
}

fn command_sentences(db_path: &str, word: &str) -> Result<()> {
    let conn = Connection::open_with_flags(db_path, OpenFlags::SQLITE_OPEN_READ_ONLY)?;

    let mut stmt = conn.prepare(
        "
        SELECT sentence
        FROM Sentence
            JOIN WordInSentence ON Sentence.id = WordInSentence.SentenceId
            JOIN Word ON Word.id = WordInSentence.wordId
        WHERE word = ?
        ",
    )?;
    let sentence_iter = stmt.query_map([word], |row| row.get(0))?;

    for item in sentence_iter {
        let sentence: String = item?;
        println!("{}", highlight_word(&sentence, word));
    }

    Ok(())
}

fn command_compact(learned_path: &str, db_path: &str) -> Result<()> {
    let known_words = read_known_words(learned_path)?;

    let mut conn = Connection::open(db_path)?;

    let tx = conn.transaction()?;

    for word in known_words {
        tx.execute(
            "
            DELETE
            FROM Word
            WHERE word = ?
            ",
            [word],
        )?;
    }

    tx.execute(
        "
        DELETE
        FROM Sentence
        WHERE id NOT IN (SELECT sentenceId FROM WordInSentence)
        ",
        [],
    )?;

    tx.commit()?;

    conn.execute("VACUUM", [])?;

    Ok(())
}

fn command_n_plus_1(db_path: &str) -> Result<()> {
    let conn = Connection::open_with_flags(db_path, OpenFlags::SQLITE_OPEN_READ_ONLY)?;

    let mut stmt = conn.prepare(
        "
        SELECT word, sentence
        FROM Sentence
            JOIN WordInSentence ON Sentence.id = WordInSentence.sentenceId
            JOIN Word ON Word.id = WordInSentence.wordId
        GROUP BY sentenceId
        HAVING COUNT(*) = 1
        ",
    )?;
    let sentence_iter = stmt.query_map([], |row| Ok((row.get(0)?, row.get(1)?)))?;

    for item in sentence_iter {
        let (word, sentence): (String, String) = item?;
        println!("{}\n\n{}", word, highlight_word(&sentence, &word));
        println!("{}", "-".repeat(79))
    }

    Ok(())
}

fn read_known_words(path: &str) -> Result<BTreeSet<String>> {
    let file = File::open(path)?;
    let mut known_words = BTreeSet::new();

    for line in BufReader::new(file).lines() {
        known_words.insert(line?);
    }

    Ok(known_words)
}

fn highlight_word(sentence: &str, word: &str) -> String {
    let stem = word.strip_suffix("하다").unwrap_or(word);
    sentence.replace(
        stem,
        &stem
            .if_supports_color(Stream::Stdout, |text| text.red())
            .to_string(),
    )
}
