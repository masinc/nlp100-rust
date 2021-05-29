use std::io::BufReader;

use crate::answer::Answer;

use whiteread::Reader;

struct Problem02;

impl Answer for Problem02 {
    fn answer(
        reader: &mut impl std::io::Read,
        writer: &mut impl std::io::Write,
    ) -> anyhow::Result<()> {
        let mut reader = Reader::new(BufReader::new(reader));
        let arg1: String = reader.line()?;
        let arg2: String = reader.line()?;

        let str: String = arg1
            .chars()
            .zip(arg2.chars())
            .map(|(chr1, chr2)| chr1.to_string() + &chr2.to_string())
            .collect();

        write!(writer, "{}", str)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test() -> Result<()> {
        let mut reader = "パトカー\nタクシー".as_bytes();
        let mut writer = vec![];

        Problem02::answer(&mut reader, &mut writer)?;

        let result = String::from_utf8(writer)?;

        assert_eq!("パタトクカシーー", result);

        Ok(())
    }
}
