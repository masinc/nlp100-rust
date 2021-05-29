use std::io::prelude::*;

pub trait Answer {
    fn answer(reader: &mut impl Read, writer: &mut impl Write) -> anyhow::Result<()>;
}
