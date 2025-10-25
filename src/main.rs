use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub instant);
lalrpop_mod!(pub instant2);
lalrpop_mod!(pub instant3);

fn main() {
    println!("Hello, world!");
}

#[test]
fn instant1() {
    let a = instant::TermParser::new().parse("((123))");
    dbg!(a);

    assert!(instant::TermParser::new().parse("22").is_ok());
    assert!(instant::TermParser::new().parse("(22)").is_ok());
    assert!(instant::TermParser::new().parse("(((22)))").is_ok());
    assert!(instant::TermParser::new().parse("((22)").is_err());
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
