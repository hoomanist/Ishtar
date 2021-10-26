use super::*;

#[test]
fn open_test(){
    let path = "src/tests/test.ist";
    let mut parser = file_analyzer::Parser{contents: None};
    parser.open(path);
    assert!(parser.contents.is_some());
    assert_eq!(parser.contents.unwrap(), "PSH 1\n");
}

#[test]
fn parse_test() {
    let path = "src/tests/test.ist";
    let mut parser = file_analyzer::Parser{contents: None};
    parser.open(path);
    let parsed_file = parser.parse();
    assert_eq!(parsed_file, vec![file_analyzer::Opcodes{
        instruct: instructions::Instructions::PSH,
        oprands: vec![1]
    }])
}
