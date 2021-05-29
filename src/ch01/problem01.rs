use crate::answer::Answer;

struct Problem01;

impl Answer for Problem01 {
    fn answer(
        reader: &mut impl std::io::Read,
        writer: &mut impl std::io::Write,
    ) -> anyhow::Result<()> {
        let mut str = String::new();
        reader.read_to_string(&mut str)?;

        let str: String = str
            .chars()
            .enumerate()
            .filter(|(index, _)| index % 2 == 1)
            .map(|(_, chr)| chr)
            .collect();

        write!(writer, "{}", str)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let mut reader = "パタトクカシーー".as_bytes();
        let mut writer = vec![];

        Problem01::answer(&mut reader, &mut writer)?;

        let result = String::from_utf8(writer)?;
        assert_eq!("タクシー", result);

        Ok(())
    }
}
