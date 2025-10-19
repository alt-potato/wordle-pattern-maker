# wordle-pattern-maker

For a set of target Wordle patterns, tries to give a sequence of words that fits the target.

For example:

```
XXXXX -> wizzo
XYXGG -> tawse
GGGGG -> amuse
```

Wildcards are also available, which can represent multiple patterns.

```
? -> any valid letter (G/Y)
* -> any letter (G/Y/X)

```

Unfortunately, due to the limited wordlist, not all patterns are possible.

Wordlist obtained via https://github.com/tabatkins/wordle-list, under MIT. (Thanks!)
