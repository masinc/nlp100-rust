use crate::answer::Answer;

struct Problem03;

impl Answer for Problem03 {
    fn answer(
        reader: &mut impl std::io::Read,
        writer: &mut impl std::io::Write,
    ) -> anyhow::Result<()> {
        let mut str = String::new();
        reader.read_to_string(&mut str)?;

        let result: Option<String> = str
            .split_ascii_whitespace()
            .map(|x| x.chars().next())
            .collect();

        write!(writer, "{}", result.unwrap())?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let mut reader = "Now I need a drink, alcoholic of course, after the heavy lectures involving quantum mechanics.".as_bytes();
        let mut writer = vec![];

        Problem03::answer(&mut reader, &mut writer)?;

        let result = String::from_utf8(writer)?;
        assert_eq!("NInadaocathliqm", result);

        Ok(())
    }
}
