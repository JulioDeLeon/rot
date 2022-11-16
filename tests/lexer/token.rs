use regex::RegexSet;
use rot::lexer::token::*;
use std::collections::HashMap;

#[test]
fn test_evaluate_elvis_kind() {
    let cdict = build_complex_dictionary();
    let sdict = build_simple_dictionary();
    let mut sample = "?:";
    let opt_kind = find_kind(cdict, sdict, sample.to_string());
    assert_eq!(Some(Kind::Elvis), opt_kind);
}
