extern crate combine;
use self::combine::{char, try, string, letter, spaces, many1, parser, Parser, ParserExt};
use self::combine::primitives::{State, Stream, ParseResult};
use lambda::Lambda;
use lambda::Lambda::*;

pub fn lambda<I>(input: State<I>) -> ParseResult<Lambda, I>
    where I: Stream<Item=char> {

    let var = || many1(letter());
    let lex_char = |c| char(c).skip(spaces());

    let app =
        (lex_char('('),
         parser(lambda::<I>),
         lex_char('.'),
         parser(lambda::<I>),
         lex_char(')'))
            .map(|t| App(Box::new(t.1), Box::new(t.3)));

    let abs =
        (var().skip(spaces()),
         string("->").skip(spaces()),
         parser(lambda::<I>))
            .map(|t| Abs(t.0, Box::new(t.2)));

    try(app)
       .or(try(abs))
       .or(try(var().map(Var)))
       .skip(spaces())
       .parse_state(input)
}
