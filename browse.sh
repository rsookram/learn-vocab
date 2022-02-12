#!/bin/sh

LEARNED="$1"
DB="$2"

CMD='./target/release/learn-vocab'

"$CMD" unknown "$LEARNED" "$DB" |
  fzf --preview "echo {} | cut -d' ' -f2 | xargs $CMD --color=always sentences $DB | sed G" --preview-window=wrap
