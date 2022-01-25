import sys

from konlpy.tag import Okt


learned_path = sys.argv[1]

model = Okt()

doc = sys.stdin.read().replace('\n', '. ')
morphs = model.morphs(doc, stem=True)

with open(learned_path) as f:
    learned = f.readlines()


# Use dict instead of set for stable order
words = { line.rstrip() : None for line in learned }

for m in morphs:
    if m not in words:
        words[m] = None

for word, *_ in words.items():
    print(word)
