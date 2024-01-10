# deci
2、10、16進数を相互変換するツール

```
git clone https://github.com/TundraClimate/deci
```
```
cd deci
```
```
cargo build -r
```
```
sudo ln -s ./target/release/deci /usr/local/bin
```

# how to use
```
$ deci --help               
Convert binaly, decimal and hexadecimal.

Usage: deci [OPTIONS] <VALUE>

Arguments:
  <VALUE>  Before conversion value

Options:
  -t, --target <TARGET>  Allow 2, 10, 16
  -b, --base <BASE>      Allow 2, 10, 16 [default: 10]
  -h, --help             Print help
  -V, --version          Print version
```
