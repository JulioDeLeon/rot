use rot::lexer::token::*;

#[test]
fn test_evaluate_elvis_kind() {
    let cdict = build_complex_dictionary();
    let sdict = build_simple_dictionary();
    let sample = "?:";
    let opt_kind = find_kind(cdict, sdict, sample.to_string());
    assert_eq!(Some(Kind::Elvis), opt_kind);
}
