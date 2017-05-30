import sys

words = {}
for line in sys.stdin:
    line = line.strip()
    letters = list(line)
    letters.sort();
    key = "".join(letters)
    if key in words:
        words[key] = words[key] + [line]
    else:
        words[key] = [line]

for key in words:
    if len(words[key]) > 1:
        print words[key]
