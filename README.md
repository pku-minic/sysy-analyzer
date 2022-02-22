# sysy-analyzer

A simple SysY program analyzer, currently supports only lexical/grammar analyzing.

## Usage

```sh
# show help message
cargo run
# run on SysY source file
cargo run -- source.c statistics.json
# run on directory
cargo run -- sysy_files statistics.json
```

## Classifier for SysY Test Cases

This repository also contains a SysY test case classifier based on `sysy-analyzer`. You can run:

```sh
cargo run -- sysy_files | ./classifier.py > result.csv
```

## Copyright and License

Copyright 2022 MaxXing. License GPLv3.
