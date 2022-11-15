use rot::lexer::token::*;

#[test]
fn evaluate_keywords() {
    let cdict = build_complex_dictionary();
    let sdict = build_simple_dictionary();

    let mut sample = "?:";
    let opt_kind = find_kind(cdict, sdict, sample.to_string());
    assert_eq!(Some(Kind::Elvis), opt_kind);
    // could add all cases but for now this is fine.
}
