use crate::problem::Problem;
use thiserror::Error;

struct Problem03;

#[derive(Error, Debug)]
pub enum Error {
    #[error("The empty existed in the split string of arg")]
    EmptyChar,
}

impl Problem for Problem03 {
    type Args = String;
    type Result = String;
    fn answer(args: Self::Args) -> anyhow::Result<Self::Result> {
        Ok(args
            .split_ascii_whitespace()
            .map(|x| x.chars().next())
            .collect::<Option<String>>()
            .ok_or(Error::EmptyChar)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let args = "Now I need a drink, alcoholic of course, after the heavy lectures involving quantum mechanics.".into();
        assert_eq!("NInadaocathliqm", Problem03::answer(args)?);

        Ok(())
    }
}
