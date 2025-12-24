# BrBaText
Turns text into text where as many letters as possible have been replaced with chemical elements.

# Installation
```
cargo install --path .
```

# Usage
```bash
$ echo "Hello World! HELLO WORLD!" | brba_text
[He]llo [W]orld! [H]ELL[O] [W][O]RLD!
```
Does not support Unicode, just ignores those bytes. You can recompile it with a different ``elements.txt`` to change the behavior, although only elements of lengths 1 or 2 are allowed.


# Benchmarking
I used the Project Gutenberg UTF-8 version of Moby Dick to benchmark this (inputs/moby_dick.txt). You will need to download this seperately to benchmark it yourself.

```bash
$ du -h inputs/moby_dick.txt 
1.3M    inputs/moby_dick.txt
```

```bash
$ cargo bench
process_text_large      time:   [5.1793 ms 5.3112 ms 5.4583 ms]
```