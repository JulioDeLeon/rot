use rot::lexer::token::*;

#[test]
fn is_space_test() {
    let mut check = "\n";
    assert_eq!(true, is_space(check));

    check = "\t";
    assert_eq!(true, is_space(check));

    check = "\r";
    assert_eq!(true, is_space(check));

    check = " ";
    assert_eq!(true, is_space(check));

    check = "  ";
    assert_eq!(true, is_space(check));

    check = "  c";
    assert_eq!(false, is_space(check));
}

#[test]
fn evaluate_keywords() {
    let cdict = build_complex_dictionary();
    let sdict = build_simple_dictionary();

    let mut sample = "?:";
    let opt_kind = find_kind(cdict, sdict, sample.to_string());
    assert_eq!(Some(Kind::Elvis), opt_kind);
    // could add all cases but for now this is fine.
}
