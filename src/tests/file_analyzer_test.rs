use super::*;

#[test]
fn open_test(){
    let path = "/home/hooman/code/ishtar/src/tests/test.ist";
    let mut parser = file_analyzer::Parser{contents: None};
    parser.open(path);
    assert!(parser.contents.is_some());
    assert_eq!(parser.contents.unwrap(), "PSH 1;\n");
}
