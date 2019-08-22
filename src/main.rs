mod github;

use regex::Regex;
use serde::Deserialize;
use std::{error::Error, process::exit};

// https://help.github.com/en/articles/virtual-environments-for-github-actions#default-environment-variables
#[derive(Deserialize, Debug)]
struct Config {
    #[serde(default = "default_pattern", with = "serde_regex")]
    pattern: Regex,
    github_event_path: String,
    github_event_name: String,
}

fn default_pattern() -> Regex {
    Regex::new(
        "\\[(ci|actions|actions ci|actions-ci|actionsci) skip|skip (ci|actions|actions ci|actions-ci|actionsci)\\]"
    ).expect("invalid pattern")
}

fn skip(config: Config) -> Result<bool, Box<dyn Error>> {
    let Config {
        pattern,
        github_event_path,
        github_event_name,
    } = config;
    if "push" != github_event_name {
        return Ok(true);
    }
    Ok(message_filter(
        &pattern,
        github::parse(github_event_path)?
            .into_iter()
            .map(|c| c.message),
    ))
}

fn message_filter<C, M>(
    pattern: &Regex,
    messages: C,
) -> bool
where
    M: AsRef<str>,
    C: IntoIterator<Item = M>,
{
    messages
        .into_iter()
        .any(|message| pattern.is_match(message.as_ref()))
}

fn main() -> Result<(), Box<dyn Error>> {
    if skip(envy::from_env()?)? {
        exit(78)
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn message_filter_skips_on_default_cases() -> Result<(), Box<dyn Error>> {
        let default = default_pattern();
        for messages in vec![
            vec!["[skip ci]"],
            vec!["[skip actions ci]"],
            vec!["[skip actions]"],
            vec!["[skip actionsci]"],
            vec!["[skip ci]"],
            vec!["[ci skip]"],
            vec!["[actions ci skip]"],
            vec!["[actions skip]"],
            vec!["[actions ci skip]"],
            vec!["[actionsci skip]"],
        ] {
            assert!(message_filter(&default, messages));
        }
        Ok(())
    }
    
    #[test]
    fn message_filter_passes_on_default_cases() -> Result<(), Box<dyn Error>> {
        let default = default_pattern();
        for messages in vec![
            vec!["skip ci"],
        ] {
            assert!(!message_filter(&default, messages));
        }
        Ok(())
    }
}
