use std::fs::File;
use std::io::Read;

use combine::parser::char::{char, digit, letter, space, string};
use combine::stream::PointerOffset;
use combine::{choice, many1, Parser, EasyParser, attempt};

#[derive(Debug, PartialEq)]
enum ComparisonOperator {
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    VariableReference(String),
    StringLiteral(String),
    IntegerLiteral(i64),
    BinaryOperation(Box<Expression>, ComparisonOperator, Box<Expression>),
}

#[derive(Debug, PartialEq)]
enum MetadataField {
    Type(String, String ),
    Author(String, String),
    Other(String, String)
}

#[derive(Debug, PartialEq)]
enum Action {
    ChangeColor(String),
    AddOverlay(String),
}

#[derive(Debug, PartialEq)]
pub struct ImageMetadata {
    fields: Vec<MetadataField>,
    actions: Vec<Action>,
}

fn comparison_operator<Input>() -> impl Parser<Input, Output = ComparisonOperator>
where
    Input: combine::Stream<Token = char>,
{
    choice((
        char('=').with(char('=')).map(|_| ComparisonOperator::Equals),
        char('!').with(char('=')).map(|_| ComparisonOperator::NotEquals),
        char('>').map(|_| ComparisonOperator::GreaterThan),
        char('<').map(|_| ComparisonOperator::LessThan),
        char('>').with(char('=')).map(|_| ComparisonOperator::GreaterThanOrEqual),
        char('<').with(char('=')).map(|_| ComparisonOperator::LessThanOrEqual),
    ))
}

pub fn variable_reference<Input>() -> impl Parser<Input, Output = String>
where
    Input: combine::Stream<Token = char>,
{
    many1(letter()).map(|s: String| s.to_owned())
}

pub fn integer_literal<Input>() -> impl Parser<Input, Output = i64>
where
    Input: combine::Stream<Token = char>,
{
    many1(digit())
        .map(|s: String| s.parse().unwrap())
}

pub fn string_literal<Input>() -> impl Parser<Input, Output = String>
where
    Input: combine::Stream<Token = char>,
{
    char('"')
        .with(many1(choice((letter(), digit(), space()))))
        .skip(char('"'))
        .map(|s: String| s.to_owned())
}

pub fn binary_operation<Input>() -> impl Parser<Input, Output = Expression>
where
    Input: combine::Stream<Token = char>,
{
    (
        variable_reference(),
        comparison_operator(),
        variable_reference(),
    )
    .map(|(left, op, right)| {
        Expression::BinaryOperation(
            Box::new(Expression::VariableReference(left)),
            op,
            Box::new(Expression::VariableReference(right)),
        )
    })
}

fn expression<Input>() -> impl Parser<Input, Output = Expression>
where
    Input: combine::Stream<Token = char>,
{
    choice((
        binary_operation(),
        variable_reference().map(|s| Expression::VariableReference(s)),
        string_literal().map(|s| Expression::StringLiteral(s)),
        integer_literal().map(|i| Expression::IntegerLiteral(i)),
    ))
}

fn metadata_field<Input>() -> impl Parser<Input, Output = MetadataField>
where
    Input: combine::Stream<Token = char>,
{
    let type_parser = (
        attempt(string("metadata field ")),
        string_literal(),
        string(" is "),
        string_literal(),
    )
    .map(|(_, key, _, value)| MetadataField::Type(key,value));

    let author_parser = (
        attempt(string("metadata field ")),
        string_literal(),
        string(" is "),
        string_literal(),
    )
    .map(|(_, key, _, value)| MetadataField::Author(key ,value));


    choice((type_parser, author_parser))
}

pub fn image_metadata<Input>() -> impl Parser<Input, Output = ImageMetadata>
where
    Input: combine::Stream<Token = char>,
{
    
    let field_parser = metadata_field();

    let action_parser = choice((
        string("change color to ")
            .with(string_literal())
            .map(Action::ChangeColor),
        string("add overlay file: ")
            .with(string_literal())
            .map(Action::AddOverlay),
    ));

    let fields_parser = many1(field_parser);
   
    let actions_parser = many1(action_parser);

    (
        string("where"),
        space(),
        fields_parser,
        string("{"),
        space(),
        actions_parser,
        space(),
        string("}"),
    )
    .map(|(_, _, fields, _, _, actions, _, _)| ImageMetadata { fields, actions })
}

pub fn parse(input: &str) -> Result<(ImageMetadata, &str), combine::easy::Errors<char, &str, PointerOffset<str>>> {
    image_metadata().easy_parse(input)
}
pub fn load_file(filename: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}


#[cfg(test)]
mod test_parser {
    

    use super::*;

    #[test]
    fn test() {
        let input: &str = r#"where metadata field "Type" is "Landscape"{ change color to "blue" }"#;
        let result: Result<(ImageMetadata, &str), combine::easy::Errors<char, &str, PointerOffset<str>>> = image_metadata().easy_parse(input);
        println!("{:?}", result);
        assert!(result.is_ok());
    }
}