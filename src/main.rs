use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub instant1);
lalrpop_mod!(pub instant2);
lalrpop_mod!(pub instant3);
lalrpop_mod!(pub instant4);

pub mod ast;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstantError {
    InputTooBig,
}


fn main() {
    println!("Hello, world!");
}

#[test]
fn instant1() {
    let a = instant1::TermParser::new().parse("((123))");
    dbg!(a);

    assert!(instant1::TermParser::new().parse("22").is_ok());
    assert!(instant1::TermParser::new().parse("(22)").is_ok());
    assert!(instant1::TermParser::new().parse("(((22)))").is_ok());
    assert!(instant1::TermParser::new().parse("((22)").is_err());
}

#[test]
fn instant2() {
    let a = instant2::ExprParser::new().parse("(13 + (7 * 3))");
    dbg!(a);

    assert!(instant2::ExprParser::new().parse("(13 + (7 * 3))").is_ok());
    assert!(instant2::ExprParser::new().parse("13 + 7 * 3").is_ok());
    assert!(instant2::ExprParser::new().parse("(22)").is_ok());
    assert!(instant2::ExprParser::new().parse("(((22)))").is_ok());
    assert!(instant2::ExprParser::new().parse("((22)").is_err());
}

#[test]
fn instant3() {
    let a = instant3::ExprParser::new().parse("(13 + (7 * 3))");
    dbg!(a);

    assert!(instant3::ExprParser::new().parse("(13 + (7 * 3))").is_ok());
    assert!(instant3::ExprParser::new().parse("13 + 7 * 3").is_ok());
    assert!(instant3::ExprParser::new().parse("(22)").is_ok());
    assert!(instant3::ExprParser::new().parse("(((22)))").is_ok());
    assert!(instant3::ExprParser::new().parse("((22)").is_err());
}

#[test]
fn instant4() {
    let a = instant4::ExprParser::new().parse("(13 + (7 * 3))");
    dbg!(a);

    assert!(instant4::ExprParser::new().parse("(13 + (7 * 3))").is_ok());
    assert!(instant4::ExprParser::new().parse("13 + 7 * 3").is_ok());
    assert!(instant4::ExprParser::new().parse("(22)").is_ok());
    // assert!(instant4::ExprParser::new().parse("(((21239870495872037598232)))").is_ok());
    assert!(instant4::ExprParser::new().parse("((22)").is_err());
}

#[test]
fn instant5() {
    use lalrpop_util::ParseError;

    let expr = instant4::ExprParser::new().parse("(((21239870495872037598232)))");
    dbg!(&expr);
    assert!(expr.is_err());
}
