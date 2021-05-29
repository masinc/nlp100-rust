use crate::answer::Answer;

struct Problem00 {}

impl Answer for Problem00 {
    fn answer(
        reader: &mut impl std::io::Read,
        writer: &mut impl std::io::Write,
    ) -> anyhow::Result<()> {
        let mut str = String::new();
        reader.read_to_string(&mut str)?;

        let str: String = str.chars().rev().collect();
        write!(writer, "{}", str)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() -> anyhow::Result<()> {
        let mut reader = "stressed".as_bytes();
        let mut writer = vec![];
        Problem00::answer(&mut reader, &mut writer)?;

        assert_eq!("desserts", String::from_utf8(writer)?);
        Ok(())
    }
}
