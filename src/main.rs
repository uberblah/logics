extern crate combine;
extern crate uuid;

#[macro_use] extern crate lalrpop_util;
lalrpop_mod!(pub input_grammar); // synthesized by LALRPOP

#[test]
fn testlalr() {
    assert_eq!(input_grammar::ExprParser::new().parse("22").unwrap(), Value::Bound("22".to_string()));
    assert_eq!(input_grammar::ExprParser::new().parse("(22)").unwrap(), Value::Stmt(vec![Value::Bound("22".to_string())]));
    assert_eq!(input_grammar::ExprParser::new().parse("((((22))))").unwrap(),
        Value::Stmt(vec![Value::Stmt(vec![Value::Stmt(vec![Value::Stmt(vec![Value::Bound("22".to_string())])])])]));
    assert!(input_grammar::ExprParser::new().parse("((22)").is_err());
}

mod token;
mod value;

use self::token::{lexer, Token};
use self::value::Value;
use combine::{many1, Parser};
use std::collections::HashMap;
use uuid::Uuid;

/*
(implies (equiv <a> <b>) (implies <a> <b>))
(implies (equiv <a> <b>) (equiv <b> <a>))

(equiv (not <a>) (implies <a> false))

(implies (and (implies <a> <b>) (implies <b> <c>)) (implies <a> <c>))

(implies <a> (implies <b> (and <a> <b>)))
(implies (and <a> <b>) (and <b> <a>))
(implies (and <a> <b>) <a>)

(implies (or <a> <b>) (implies (not <a>) <b>))
(implies (or <a> <b>) (implies (not <b>) <a>))
(implies (and <a> <b>) (not (or (not <a>) (not <b>))))

(implies (xor <a> <b>) (or (and <a> (not <b>)) (and <b> (not <a>))))

////////////
(xor (is <person> alive) (is <person> dead))
(xor (has homer hot_chocolate) (is homer dead))
(implies (has homer hot_chocolate) (is homer happy))
(has homer hot_chocolate)

////////////
(or
    (and
        (has homer hot_chocolate)
        (not (is homer dead))
    )
    (and
        (is homer dead)
        (not (has homer hot_chocolate))
    )
)
(implies
    (not (and
        (has homer hot_chocolate)
        (not (is homer dead))
    ))
    (and
        (is homer dead)
        (not (has homer hot_chocolate))
    )
)
(implies
    (not (and
        (is homer dead)
        (not (has homer hot_chocolate))
    ))
    (and
        (has homer hot_chocolate)
        (not (is homer dead))
    )
)

///////////
(not a) -> (a implies false)
(implies (and a <b>) a), (implies a false) -> (implies (and a <b>) false)
(implies (and a <b>) false) -> (not (and a <b>))

a
(implies (implies a false) false)
*/

fn main() {
    println!("{}", Uuid::new_v4());

    let a = Uuid::new_v4().to_string();
    let b = Uuid::new_v4().to_string();
    let c = Uuid::new_v4().to_string();
    let d = Uuid::new_v4().to_string();

    let mut binds = HashMap::new();
    let v1 = Value::Stmt(vec![
        Value::Implies,
        Value::Unbound(a.clone()),
        Value::Unbound(a),
        Value::Unbound(b),
    ]);
    let v2 = Value::Stmt(vec![
        Value::Implies,
        Value::Bound(d.clone()),
        Value::Bound(d),
        Value::Unbound(c),
    ]);
    let r#match = v1.r#match(&v2, &mut binds);
    assert!(r#match, "This test must pass");
    println!("match: {}", r#match);

    println!(
        "LEXED STUFF => {:?}",
        many1::<Vec<Token>, _>(lexer()).parse(" (false implies () oh my 'wh_oopsey ) ")
    );
}
