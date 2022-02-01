import sqlite3
import sys

from konlpy.tag import Okt


db_path = sys.argv[1]
doc_path = sys.argv[2]

excluded_pos = [
        'Josa',
        'Punctuation',
        'Suffix',
        'Foreign',
        'Determiner',
        'Number',
]

prepare_db = '''
PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS Document(
    id INTEGER PRIMARY KEY,
    path TEXT UNIQUE NOT NULL
);

CREATE TABLE IF NOT EXISTS Sentence(
    id INTEGER PRIMARY KEY,
    sentence TEXT NOT NULL,
    doc_id INTEGER REFERENCES Document(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS Word(
    id INTEGER PRIMARY KEY,
    word TEXT UNIQUE NOT NULL
);

CREATE TABLE IF NOT EXISTS WordInSentence(
    wordId INTEGER NOT NULL REFERENCES Word(id) ON DELETE CASCADE,
    sentenceId INTEGER NOT NULL REFERENCES Sentence(id) ON DELETE CASCADE,
    PRIMARY KEY (wordId, sentenceId)
);
'''

insert_document = 'INSERT INTO Document (path) VALUES (?)'
insert_sentence = 'INSERT INTO Sentence (sentence, doc_id) VALUES (?, ?)'
insert_word = 'INSERT INTO Word (word) VALUES (?)'
insert_word_in_sentence = 'INSERT INTO WordInSentence (wordId, sentenceId) VALUES (?, ?)'

has_document = 'SELECT 1 FROM Document WHERE path=?'
get_word_id = 'SELECT id FROM Word WHERE word=?'

def load_sentences(path):
    with open(path) as f:
        sentences = f.readlines()

    sentences = (s.split('. ') for s in sentences)
    sentences = (s.strip() for ss in sentences for s in ss if s.strip())
    return [s.replace('\xa0', ' ') for s in sentences]

def extract_words(model, sentences):
    sentences_to_words = {}

    for s in sentences:
        words = model.pos(s, stem=True)
        words = set((w for (w, pos) in words if pos not in excluded_pos))

        sentences_to_words[s] = words

    return sentences_to_words

def main():
    # setup
    con = sqlite3.connect(db_path)
    cur = con.cursor()

    cur.executescript(prepare_db)

    cur.execute(has_document, (doc_path,))
    result = cur.fetchone()
    if result is not None:
        print('already imported', doc_path)
        return

    print('importing', doc_path)

    model = Okt()

    sentences = load_sentences(doc_path)
    sentences_to_words = extract_words(model, sentences)

    # insert
    cur.execute(insert_document, (doc_path,))
    doc_id = cur.lastrowid

    word_to_id = {}
    for words in sentences_to_words.values():
        for w in words:
            cur.execute(get_word_id, (w,))
            result = cur.fetchone()

            if result is not None:
                word_to_id[w] = result[0]
            else:
                cur.execute(insert_word, (w,))
                word_to_id[w] = cur.lastrowid

    for (sentence, words) in sentences_to_words.items():
        cur.execute(insert_sentence, (sentence, doc_id))
        sentence_id = cur.lastrowid

        for w in words:
            cur.execute(insert_word_in_sentence, (word_to_id[w], sentence_id))

    # finish
    con.commit()
    con.close()


main()
