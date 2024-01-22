/// https://redis.io/docs/reference/protocol-spec/
use nom::{
    bytes::complete::{tag, take, take_while},
    character::complete::{i32, one_of},
    combinator::verify,
    multi::count,
    number::complete::double,
    sequence::terminated,
    IResult,
};
use num_bigint::BigUint;

#[derive(Debug, PartialEq, Clone)]
pub enum Value<'a> {
    SimpleString(&'a str), // +
    SimpleError(&'a str),  // -
    Integer(i32),          // :
    BulkString(&'a str),   // $
    Array(Vec<Value<'a>>), // *
    Null,                  // _
    Boolean(bool),         // #
    Double(f64),           // ,
    BigNumber(BigUint),    // (
    BulkError,             // !
    VerbatimString,        // =
    Map,                   // %
    Set,                   // ~
    Push,                  // >
}

pub fn parse_value(input: &str) -> IResult<&str, Value> {
    let (input, resp_type) = one_of("+_:$*_#,(!=%~>")(input)?;
    match resp_type {
        '+' => parse_simple_string(input),
        '_' => parse_simple_error(input),
        ':' => parse_integer(input),
        '$' => parse_bulk_string(input),
        '*' => parse_array(input),
        '#' => parse_boolean(input),
        ',' => parse_double(input),
        '(' => parse_big_number(input),
        '!' => parse_bulk_error(input),
        '=' => parse_verbatim_string(input),
        '%' => parse_map(input),
        '~' => parse_set(input),
        '>' => parse_push(input),
        _ => unreachable!(),
    }
}

fn clrf(input: &str) -> IResult<&str, &str> {
    tag("\r\n")(input)
}

fn u32_or_minus1(input: &str) -> IResult<&str, i32> {
    let (input, value) = terminated(verify(i32, |n| n >= &-1), clrf)(input)?;
    Ok((input, value))
}

fn parse_simple_string_raw(input: &str) -> IResult<&str, &str> {
    terminated(take_while(|c| c != '\r' && c != '\n'), clrf)(input)
}

fn parse_simple_string(input: &str) -> IResult<&str, Value> {
    let (input, value) = parse_simple_string_raw(input)?;
    Ok((input, Value::SimpleString(value)))
}

fn parse_simple_error(input: &str) -> IResult<&str, Value> {
    let (input, value) = parse_simple_string_raw(input)?;
    Ok((input, Value::SimpleError(value)))
}

fn parse_integer(input: &str) -> IResult<&str, Value> {
    let (input, value) = terminated(i32, clrf)(input)?;
    Ok((input, Value::Integer(value)))
}

fn parse_bulk_string(input: &str) -> IResult<&str, Value> {
    let (input, length) = terminated(u32_or_minus1, clrf)(input)?;
    if length == -1 {
        return Ok((input, Value::Null));
    }

    let (input, value) = terminated(take(length as u32), clrf)(input)?;
    Ok((input, Value::BulkString(value)))
}

fn parse_array(input: &str) -> IResult<&str, Value> {
    let (input, length) = terminated(u32_or_minus1, clrf)(input)?;
    if length == -1 {
        return Ok((input, Value::Null));
    }
    let (input, values) = count(parse_value, length as usize)(input)?;
    Ok((input, Value::Array(values)))
}

fn parse_null(input: &str) -> IResult<&str, Value> {
    let (input, _) = terminated(tag("_"), clrf)(input)?;
    Ok((input, Value::Null))
}

fn parse_boolean(input: &str) -> IResult<&str, Value> {
    let (input, value) = terminated(one_of("tf"), clrf)(input)?;
    match value {
        't' => Ok((input, Value::Boolean(true))),
        'f' => Ok((input, Value::Boolean(false))),
        _ => unreachable!(),
    }
}

fn parse_double(input: &str) -> IResult<&str, Value> {
    // TODO: it does not handle automatic conversion to integers and "inf", "-inf", "nan"
    let (input, value) = terminated(double, clrf)(input)?;
    Ok((input, Value::Double(value)))
}

fn parse_big_number(input: &str) -> IResult<&str, Value> {
    todo!()
}

fn parse_bulk_error(input: &str) -> IResult<&str, Value> {
    todo!()
}

fn parse_verbatim_string(input: &str) -> IResult<&str, Value> {
    todo!()
}

fn parse_map(input: &str) -> IResult<&str, Value> {
    todo!()
}

fn parse_set(input: &str) -> IResult<&str, Value> {
    todo!()
}

fn parse_push(input: &str) -> IResult<&str, Value> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("+\r\n", Value::SimpleString(""))]
    #[case("+OK\r\n", Value::SimpleString("OK"))]
    #[case("+123456\r\n", Value::SimpleString("123456"))]
    #[case("+Hello World\r\n", Value::SimpleString("Hello World"))]
    fn test_parse_simple_string(#[case] input: &str, #[case] expected: Value) {
        let (input, actual) = parse_value(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case("+a\nb\rc\r\n")]
    fn test_parse_simple_string_error(#[case] input: &str) {
        assert!(parse_value(input).is_err());
    }
}
