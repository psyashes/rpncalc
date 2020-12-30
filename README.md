# Reverse Polish Notation Calculator

A simple calculator for RPN(Reverse Polish Notation)

## Install

```bash
git clone git@github.com:psyashes/rpncalc.git
cd rpncalc
cargo install --path .
```

## Usage

- stdin

```bash
$ rpncalc
1 1 +
2
2 2 *
4
3 3 /
1
4 4 %
0
```

- file
```bash
$ rpncalc input.txt
2
21
1000000
```

