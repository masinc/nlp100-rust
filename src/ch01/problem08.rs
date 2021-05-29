use crate::problem::Problem;

struct Problem08;

fn cipher(text: String) -> String {
    text.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() && c.is_ascii_lowercase() {
                (219 - (c as u8)) as char
            } else {
                c
            }
        })
        .collect()
}

impl Problem for Problem08 {
    type Args = String;
    type Result = String;
    fn answer(args: Self::Args) -> anyhow::Result<Self::Result> {
        Ok(cipher(args))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let result = Problem08::answer("Cipher".into())?;
        assert_eq!("Crksvi", result);

        Ok(())
    }
}
