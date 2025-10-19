# wordle-pattern-maker

For a set of target Wordle patterns, tries to give a sequence of words that fits the target.

For example:

```
XXXXX -> wizzo
XYXGG -> tawse
GGGGG -> amuse
```

Patterns are defined using a string of characters. Wildcards are also available, which can represent multiple possible options.

```
G -> correct letter, correct position
Y -> correct letter, incorrect position
X -> incorrect letter
? -> any valid letter (G/Y)
* -> any letter (G/Y/X)
```

Example usage:

```
$ wordle-pattern-maker -s amuse -p XXXXX -p XYXGG -p ??X?? -p *X*X* -p GGGGG
Possible solutions for pattern XXXXX:
  wizzo
  (and 1102 others)
Possible solutions for pattern XYXGG:
  tawse
  (and 31 others)
Possible solutions for pattern ??X??:
  amies
  (and 48 others)
Possible solutions for pattern *X*X*:
  acute
  (and 6025 others)
Possible solutions for pattern GGGGG:
  amuse
```

Unfortunately, due to the limited wordlist, not all patterns are possible.

```
$ wordle-pattern-maker -s amuse -p XYXYX -p GGYYY
Possible solutions for pattern XYXYX:
  laxed
  (and 537 others)
No possible solutions found for pattern GGYYY.
Some patterns have no possible solutions. :(
```

More options are available under `wordle-pattern-maker --help`.

Wordlist obtained via https://github.com/tabatkins/wordle-list, under MIT. (Thanks!)
