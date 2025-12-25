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
I use random data to benchmark this (inputs/random.txt)

```bash
$ head -c 10M /dev/urandom | base64 | head -c 1G > inputs/random.txt
```

```bash
$ cargo bench
process_text_random     time:   [54.685 ms 55.291 ms 55.947 ms]
```