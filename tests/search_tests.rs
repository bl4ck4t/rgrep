use std::io::Cursor;
use rgrep::search::*;

#[test]
fn finds_matching_lines() {
    let input = "\
hello world
this is rust
hello systems programming";

    let reader = Cursor::new(input);
    let res = search("hello", reader).unwrap();

    assert_eq!(res.len(), 2);
    assert_eq!(res[0], "hello world");
    assert_eq!(res[1], "hello systems programming");
}

#[test]
fn finds_case_insensitive_matches() {
    let input = "\
Hello World
this is Rust
HELLO systems";

    let reader = Cursor::new(input);
    let result = search_case_insensitive("hello", reader).unwrap();

    assert_eq!(result.len(), 2);
}
