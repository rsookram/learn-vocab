# learn-vocab

A set of tools for identifying Korean vocabulary to learn given a set of texts
to read.


## Setup

### [Poetry](https://python-poetry.org/docs/)

Once Python 3 and Poetry are installed, run `poetry install` to install the
dependencies.

### [Rust](https://www.rust-lang.org/learn/get-started)

Once Rust and Cargo are installed, run `cargo build --release` to build the
`learn-vocab` binary (it will be in `target/release/learn-vocab`).


## Workflow

The overall workflow for using the code in this repository is split into 3
steps:

1. Importing text that represents content you want to be able to read.
1. Identifying words that you already know.
1. Using the data from the previous two steps to determine what to learn next.


### Import Text

Run the following command from the root of the repository to import the text in
`$FILE` into the database `$DATA_DB`. The database will be created if it
doesn't already exist.

```shell
poetry run python3 import_document/import.py $DATA_DB $FILE
```

### Import Known Words

Known words are stored in a newline-separated UTF-8 text file (`$KNOWN_WORDS`).
Updating this manually can be time consuming though, so there's a script that
can take text comprised of known words and extract the words from it. It's used
like this:

```shell
poetry run python3 import_vocab/import.py $KNOWN_WORDS < $KNOWN_TEXT >
$NEW_KNOWN_WORDS
```

This is especially useful if you use an SRS which contains your known words.
The data from the SRS can be dumped and imported into the necessary format
using this script.

### Words to Learn

This section uses the `learn-vocab` tool. It has three subcommands which
presents words to learn in different ways:

#### `unknown`

The `unknown` subcommand lists words from the imported text that aren't in
`$KNOWN_WORDS` by frequency (descending).

Example usage:

```shell
learn-vocab unknown $KNOWN_WORDS $DATA_DB
```

#### `sentences`

The `sentences` subcommand lists sentences containing the given word. It will
try to highlight the given word, but this may fail if the word is conjugated in
the sentence.

Example usage:

```shell
learn-vocab sentences $DATA_DB $WORD
```

#### `n-plus-one`

The `n-plus-one` subcommand lists n+1 sentences (words where there is only one
unknown word). It will try to highlight the word to learn in the sentence like
the `sentences` subcommand.

Example usage:

```shell
learn-vocab n-plus-one $DATA_DB
```

There is one other subcommand that needs to be used in conjunction with this to
work properly: `compact`. The `compact` subcommand removes words from the
database that have already been learned, reducing its size. It can be used
like:

```shell
learn-vocab compact $KNOWN_WORDS $DATA_DB
```


## License

[MIT](LICENSE)
