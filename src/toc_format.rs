use std::{fmt::Display, str::FromStr};

use chumsky::{
    error::Simple,
    primitive::{choice, end, just, take_until},
    text::{self},
    Parser,
};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Toc {
    pub entries: Vec<TocEntry>,
}

impl Toc {
    pub fn page_offset(&mut self, offset: i32) {
        for entry in &mut self.entries {
            entry.page = entry.page.saturating_add_signed(offset);
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TocEntry {
    pub depth: u32,
    pub page: u32,
    pub title: String,
}

impl FromStr for Toc {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parser = toc_parser();
        let entries = parser.parse(s).map_err(|e| {
            anyhow::anyhow!(e
                .into_iter()
                .map(|e| format!("{e}"))
                .fold(String::new(), |acc, val| acc + &val))
        })?;
        Ok(Toc { entries })
    }
}

impl Display for TocEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{0:+<1$}{2} {3}",
            "", self.depth as usize, self.page, self.title
        )
    }
}

impl Display for Toc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, entry) in self.entries.iter().enumerate() {
            write!(f, "{entry}")?;
            if i != self.entries.len() - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

fn toc_parser() -> impl Parser<char, Vec<TocEntry>, Error = Simple<char>> {
    let depth = just('+').repeated().map(|s| s.len() as u32);
    let page = text::int(10)
        .try_map(|s: String, span| {
            s.parse::<u32>()
                .map_err(|_| Simple::custom(span, "Invalid page number"))
        })
        .then_ignore(just(' '));
    let title = take_until(choice((text::newline(), end())))
        .map(|(x, _)| x.into_iter().collect::<String>());

    let toc_entry = depth
        .then(page)
        .then(title)
        .map(|((depth, page), title)| TocEntry { depth, page, title });

    toc_entry
        .repeated()
        .then_ignore(end()) //assert end of files after pasing all entries.
        .collect::<Vec<TocEntry>>()
}
