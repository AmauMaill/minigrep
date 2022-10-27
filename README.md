# About

This small project was done following the (very nice) [tutorial](https://doc.rust-lang.org/book/ch12-00-an-io-project.html) from the Rust book[^1].

## Core features

The minigrep program is a CLI that allows to search for a word in a document.

It is possible to search with or without case sensitivity.

## Usage

The Most basic usage is using `cargo run -- [query] [file_path]`.

Using (with `poem.txt` file):

```bash
cargo run -- to poem.txt
```

Will return:

```
Are you nobody, too?
How dreary to be somebody!
```

You can also use `export IGNORE_CASE=1` to use the case insensitive option. You can reverse to using case sensitive option by using `unset IGNORE_CASE`.

Finally, it is possible to output the result of the program in a file (here in `output.txt`):

```bash
cargo run -- to poem.txt > output.txt
```

## Testing

The program was made following TDD, you can run the tests with the command `cargo test`.

[^1]: I would like to emphasise on the fact that: 

    - I did not design this program. 
    - This is just the result of me following a walkthrough.
