# learn-vocab

A set of tools for identifying Korean vocabulary to learn given of set of texts
to read.


## Setup

- poetry (Python 3)
- Rust


## Workflow

The overall workflow for using the code in this repo is split into 3 steps:

1. Importing text that represents content you want to be able to read.
1. Identifying words that you already know.
1. Using the data from the previous two steps to determine what to learn next.


### Import Text

```shell
poetry run python3 import_document/import.py $DATA_DB $FILE
```

### Import Known Words

Known words are stored in a newline-separated UTF-8 text file (`$KNOWN_WORDS`).
Updating this manually can be time consuming though, so there's a script that
can take text containing known words and parse the words out from it. It's used
like this:

```shell
poetry run python3 import_vocab/import.py $KNOWN_WORDS < $KNOWN_TEXT >
$NEW_KNOWN_WORDS
```

This is especially useful if you have your known words in an SRS. The data from
the SRS can be dumped and imported into the necessary format using this script.

### Words to Learn

This section uses the `learn-vocab` tool. It has three subcommands which
presents words to learn in different ways:

- `unknown`
- `sentences`
- `n-plus-one`


## License

[MIT](LICENSE)
