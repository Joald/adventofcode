### part 1
`input.hs` is `input.txt` but as Haskell code
changed from `input.txt` using vim commands:
```
:%s/:/ =/
:%s/\//`div`/
```
and adding `main = print root`
### part 2
`input.pl` is `input.txt` but as Prolog code
changed from `input.txt` by removing the humn line and changing the root line to `PLMP = RMTT.` and using vim commands:
```
:%s/:/ =/
:%s/.*/\U&/
:%s/\n/, /
```
followed by wrapping the entire file/line in braces, finishing the line with a trailing period and adding `use_module(library(clpq)).`
The result is obtaining by running the lines as queries, e.g. using `swipl < input.pl`