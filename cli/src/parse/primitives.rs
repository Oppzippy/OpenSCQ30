use std::{borrow::Cow, str::FromStr};

use anyhow::{anyhow, bail};

pub fn bool(unparsed: &str) -> anyhow::Result<bool> {
    bool::from_str(unparsed).map_err(Into::into)
}

pub fn i32(unparsed: &str) -> anyhow::Result<i32> {
    i32::from_str(unparsed).map_err(Into::into)
}

pub fn one_of_options<'a>(
    unparsed: &str,
    options: &'a [Cow<'static, str>],
) -> anyhow::Result<&'a Cow<'static, str>> {
    if let Some(exact_match) = options.iter().find(|option| **option == unparsed) {
        return Ok(exact_match);
    }
    let mut case_insensitive_matches = options
        .iter()
        .filter(|option| option.eq_ignore_ascii_case(unparsed));
    let case_insensitive_match = case_insensitive_matches.next();
    if let Some(ambiguous_match) = case_insensitive_matches.next() {
        bail!(
            "{unparsed} is ambiguous, could refer to {} or {}",
            case_insensitive_match.expect("if the second match is some, the first must be as well"),
            ambiguous_match,
        );
    }

    case_insensitive_match
        .ok_or_else(|| anyhow!("{unparsed} is not a valid option. Expected one of: {options:?}"))
}

pub fn many_of_options(
    unparsed: &str,
    options: &[Cow<'static, str>],
) -> anyhow::Result<Vec<Cow<'static, str>>> {
    string_vec(unparsed)?
        .into_iter()
        .map(|str| one_of_options(&str, options).cloned())
        .collect::<anyhow::Result<Vec<_>>>()
}

pub fn string_vec(unparsed: &str) -> anyhow::Result<Vec<String>> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(unparsed.as_bytes());
    let maybe_row = reader.records().next().transpose()?;
    let strings = maybe_row
        .map(|row| {
            row.into_iter()
                .map(|entry| entry.to_string())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    Ok(strings)
}

pub fn i16_vec(unparsed: &str) -> anyhow::Result<Vec<i16>> {
    unparsed
        .split(",")
        .map(i16::from_str)
        .collect::<Result<Vec<_>, _>>()
        .map_err(Into::into)
}
