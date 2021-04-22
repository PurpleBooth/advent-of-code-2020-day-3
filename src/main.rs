use std::error::Error;
use std::io;
use std::io::{BufRead, BufReader, Read};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let mut inputs: Vec<_> = vec![];

    for line in stdin.lock().lines() {
        inputs.push(line.map_err(Box::<dyn std::error::Error>::from)?)
    }

    println!("{:?}", validator(&inputs));

    Ok(())
}

fn validator(inputs: &[String]) -> Result<i64, Box<dyn Error>> {
    Ok(inputs
        .iter()
        .map(|input| parse(&input.to_string()))
        .collect::<Result<Vec<_>, Box<dyn Error>>>()?
        .into_iter()
        .filter(|(min, max, character, password)| {
            let count = password.matches(character).count();
            min <= &(count) && count <= *max as usize
        })
        .count() as i64)
}

fn parse(input: &str) -> Result<(usize, usize, String, String), Box<dyn Error>> {
    let mut reader = BufReader::new((input).as_bytes());
    let mut min_bytes = vec![];
    reader.read_until(b'-', &mut min_bytes)?;
    min_bytes.pop();
    let mut max_bytes = vec![];
    reader.read_until(b' ', &mut max_bytes)?;
    max_bytes.pop();
    let mut character_bytes = vec![];
    reader.read_until(b':', &mut character_bytes)?;
    character_bytes.pop();
    let mut rest_bytes = vec![];
    reader.read_to_end(&mut rest_bytes)?;
    rest_bytes.remove(0);

    Ok((
        String::from_utf8(min_bytes)
            .map_err(|err| -> Box<dyn Error> { Box::from(err) })
            .and_then(|c| {
                c.parse()
                    .map_err(|err| -> Box<dyn Error> { Box::from(err) })
            })?,
        String::from_utf8(max_bytes)
            .map_err(|err| -> Box<dyn Error> { Box::from(err) })
            .and_then(|c| {
                c.parse()
                    .map_err(|err| -> Box<dyn Error> { Box::from(err) })
            })?,
        String::from_utf8(character_bytes).map_err(|err| -> Box<dyn Error> { Box::from(err) })?,
        String::from_utf8(rest_bytes).map_err(|err| -> Box<dyn Error> { Box::from(err) })?,
    ))
}

#[cfg(test)]
mod tests {
    use crate::{parse, validator};

    #[test]
    fn test_parse() {
        assert_eq!(
            (1, 3, String::from("a"), String::from("abcde")),
            parse("1-3 a: abcde").unwrap()
        );
        assert_eq!(
            (4, 7, String::from("v"), String::from("vvvvv")),
            parse("4-7 v: vvvvv").unwrap()
        )
    }

    #[test]
    fn no_input_is_0() {
        assert_eq!(0, validator(&[]).unwrap())
    }

    #[test]
    fn simple_examples() {
        assert_eq!(
            2,
            validator(&[
                "1-3 a: abcde".into(),
                "1-3 b: cdefg".into(),
                "2-9 c: ccccccccc".into(),
            ])
            .unwrap()
        )
    }
}
