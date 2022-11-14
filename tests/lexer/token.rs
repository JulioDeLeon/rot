use rot::lexer::token::is_space;

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