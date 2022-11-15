use rot::lexer::token::*;
use regex::RegexSet;
use std::collections::HashMap;

#[test]
fn test_evaluate_keywords() {
    let cdict = build_complex_dictionary();
    let sdict = build_simple_dictionary();

    let mut sample = "?:";
    let opt_kind = find_kind(cdict, sdict, sample.to_string());
    assert_eq!(Some(Kind::Elvis), opt_kind);
    // could add all cases but for now this is fine.
}

#[test]
pub fn test_build_complex_dictionary() {
    let mut ret: Vec<(String, Kind)> = Vec::new();
    ret.push((r"^[ \t\r\f]+$".to_string(), Kind::WhiteSpace));
    ret.push((r"^#.*\r?\n$".to_string(), Kind::Comment));
    ret.push((r#"^""".*"""$\r\n"#.to_string(), Kind::MultiLnStringLiteral));
    ret.push((r"^[0-9]+$".to_string(), Kind::IntLiteral));
    ret.push((r#"^".*"$"#.to_string(), Kind::StringLiteral));
    ret.push((
        r"^[0-9]+(\.[0-9]+)?$".to_string(),
        Kind::DoubleLiteral,
    ));
    // ret.push((r"".to_string(), Kind::Identifier));
    // advanced operators
    ret.push((r"^\?:$".to_string(), Kind::Elvis));
    ret.push((r"^\|\|$".to_string(), Kind::LogicalOr));
    ret.push((r"^&&$".to_string(), Kind::LogicalAnd));
    ret.push((r"^==$".to_string(), Kind::IsEqual));
    ret.push((r"^!=$".to_string(), Kind::NotEqual));
    ret.push((r"^-=$".to_string(), Kind::Increment));
    ret.push((r"^\+=$".to_string(), Kind::Decrement));
    ret.push((r"^<=$".to_string(), Kind::LessThanOrEqual));
    ret.push((r"^\+=$".to_string(), Kind::GreaterThanOrEqual));
    ret.push((r"[a-zA-Z_][a-zA-Z0-9_]*".to_string(), Kind::Identifier));

    let mut dict: HashMap<usize, Kind> = HashMap::new();
    let mut patterns: Vec<String> = Vec::new();
    for (i, x) in ret.iter().enumerate() {
        let (pattern, kind) = x;
        dict.insert(i, kind.clone());
        patterns.push(pattern.to_string())
    }

    let set = RegexSet::new(patterns).unwrap();
    let matches: Vec<_> = set.matches(" ").into_iter().collect();
    assert_eq!(matches, vec![0]);
    //get first match
    let first_match = matches[0];
    let kind = dict.get(&first_match);
    match kind {
        Some(x) => {
            assert_eq!(x.clone(), Kind::WhiteSpace)
        },
        None => {
            assert_eq!(true, false)
        }
    }
}

