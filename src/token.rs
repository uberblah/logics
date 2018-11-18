use combine::{
    attempt, between,
    char::{alpha_num, spaces},
    choice, many1, token, ParseError, Parser,
};

#[derive(Debug)]
pub enum Token {
    False,
    Implies,
    Bound(String),
    Unbound(String),
    Begin,
    End,
}

pub fn lexer<I>() -> impl Parser<Input = I, Output = Token>
where
    I: combine::RangeStream<Item = char>,
    I::Error: ParseError<char, I::Range, I::Position>,
{
    let symbol_chars = || choice((alpha_num(), token('_'), token('_')));
    // skip beginning whitespace
    spaces::<I>()
        .silent()
        .and(
            choice((
                // a given token is one of these
                attempt(token('(')).map(|_| Token::Begin),
                attempt(token(')')).map(|_| Token::End),
                attempt(many1::<String, _>(symbol_chars()).map(|s| {
                    if s == "implies" {
                        return Token::Implies;
                    }
                    if s == "false" {
                        return Token::False;
                    }
                    Token::Bound(s)
                })),
                attempt(
                    token('\'').and(many1::<String, _>(symbol_chars())
                        .map(|s| Token::Unbound(s))).map(|p| p.1)
                ),
            ))
            // skip trailing whitespace, and between tokens
            .skip(spaces().silent()),
        )
        .map(|p| p.1)
}
