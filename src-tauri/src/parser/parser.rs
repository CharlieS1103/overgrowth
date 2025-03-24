/* use std::fs::File;
use std::io::Read;

use combine::parser::char::{char, digit, letter, space, string};
use combine::stream::PointerOffset;
use combine::{attempt, choice, many1, skip_many, EasyParser, Parser};
use chrono::{DateTime, Duration, NaiveDate, Utc};

#[derive(Debug, PartialEq)]
pub enum ComparisonOperator {
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
pub enum MetadataField {
    Type(String, String ),
    Author(String, String),
    Date(String, i64, ComparisonOperator, i64),
    Other(String, String)
}

#[derive(Debug, PartialEq)]
pub enum Action {
    ChangeColor(String),
    AddOverlay(String),
}

#[derive(Debug, PartialEq)]
pub struct ImageMetadata {
    pub fields: Vec<MetadataField>,
    pub actions: Vec<Action>,
}

fn comparison_operator<Input>() -> impl Parser<Input, Output = ComparisonOperator>
where
    Input: combine::Stream<Token = char>,
{
    //Fix up this chatmonstered code at some point
    choice((
        // Equals
        skip_many(space())
            .with(char('='))
            .skip(skip_many(space()))
            .map(|_| ComparisonOperator::Equals),

        // Not equals
        skip_many(space())
            .with(char('!'))
            .with(char('='))
            .skip(skip_many(space()))
            .map(|_| ComparisonOperator::NotEquals),

        // Greater than
        skip_many(space())
            .with(char('>'))
            .skip(skip_many(space()))
            .map(|_| ComparisonOperator::GreaterThan),

        // Less than
        skip_many(space())
            .with(char('<'))
            .skip(skip_many(space()))
            .map(|_| ComparisonOperator::LessThan),

        // Greater than or equal
        skip_many(space())
            .with(char('>'))
            .with(char('='))
            .skip(skip_many(space()))
            .map(|_| ComparisonOperator::GreaterThanOrEqual),

        // Less than or equal
        skip_many(space())
            .with(char('<'))
            .with(char('='))
            .skip(skip_many(space()))
            .map(|_| ComparisonOperator::LessThanOrEqual),
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

pub fn date_literal<Input>() -> impl Parser<Input, Output = i64>
where
    Input: combine::Stream<Token = char>,
{
  // Need to rewrite date parser to allow app.date and app.last_opened to be compared to the a literal
    let date_parser = (
        integer_literal(),
        char('/'),
        integer_literal(),
        char('/'),
        integer_literal(),
    )
    .map(|(month, _, day, _, year)| {
        let date = format!("{}/{}/{}", month, day, year);
        let date_time = NaiveDate::parse_from_str(&date, "%m/%d/%Y").unwrap();
        let now = Utc::now().naive_utc().date();
        let duration = date_time.signed_duration_since(now);
        duration.num_days()
    });

    let duration_parser = integer_literal().map(|i| i);
    // convert keyword "NOW" to a duration
    let now_parser = string("NOW").map(|_| {
        let now = Utc::now().naive_utc().date();
        let duration = now.signed_duration_since(now);
        duration.num_days()
    });

    // Given duration is in days from now, we should be able to then use comparison operators on dates
    choice((date_parser, duration_parser, now_parser))
   
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
        string("Type"),
        string(" is "),
        string_literal(),
    )
    .map(|(_, key, _, value)| MetadataField::Type(key.to_string(),value));

    let author_parser = (
        attempt(string("metadata field ")),
        string("Author"),
        string(" is "),
        string_literal(),
    )
    .map(|(_, key, _, value)| MetadataField::Author(key.to_string() ,value));

    let date_parser = (
        attempt(string("metadata field ")),
        string("Date"),
        space(),
        date_literal(),
        comparison_operator(),
        date_literal(),
    )
    .map(|(_,key,_,value,op,value2,)| MetadataField::Date(key.to_string(), value,op, value2));

    choice((attempt(date_parser), attempt(author_parser), attempt(type_parser)))
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
// Under current design, language will have to be really basic one liners, which for the time being is fine,
// I'll set up the parser to go line by line.

    #[test]
    fn type_test() {
        let input: &str = r#"where metadata field Type is "Landscape"{ change color to "blue" }"#;
        let result: Result<(ImageMetadata, &str), combine::easy::Errors<char, &str, PointerOffset<str>>> = image_metadata().easy_parse(input);
        println!("{:?}", result);
        assert!(result.is_ok());
    }

    #[test]
    fn author_test(){
        let input: &str = r#"where metadata field Author is "John Doe"{ change color to "blue" }"#;
        let result: Result<(ImageMetadata, &str), combine::easy::Errors<char, &str, PointerOffset<str>>> = image_metadata().easy_parse(input);
        println!("{:?}", result);
        assert!(result.is_ok());
    }
    #[test]
    fn date_test() {
        let input: &str = r#"where metadata field Date 12/23/2005 = 12/23/2005{ change color to "blue" }"#;
        let result: Result<(ImageMetadata, &str), combine::easy::Errors<char, &str, PointerOffset<str>>> = image_metadata().easy_parse(input);
        println!("{:?}", result);
        assert!(result.is_ok());
    }
    #[test]
    fn now_test() {
        let input: &str = r#"where metadata field Date 12/23/2005 = NOW{ change color to "blue" }"#; // Obviously should = false
        let result: Result<(ImageMetadata, &str), combine::easy::Errors<char, &str, PointerOffset<str>>> = image_metadata().easy_parse(input);
        println!("{:?}", result);
        assert!(result.is_ok());
    }
// Note: currently it uses any date compared to another, need to make it so it can compare with the actual date of metadata within the image
    fn and_test() {
        let input : &str = r#"where metadata field Date 12/23/2005 = Now AND metadata field Type is "Landscape"{ change color to "blue" }"#;
        // note this won't currently function, need to implement AND operator
    }   

    #[test]
    fn test_all(){
        type_test();
        author_test();
        date_test();
        now_test();
    }
}
    */