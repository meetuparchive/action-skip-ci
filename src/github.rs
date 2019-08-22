use serde::Deserialize;
use std::{error::Error, fs::File, path::Path};

#[derive(Deserialize)]
struct Push {
    commits: Vec<Commit>,
}

#[derive(Deserialize)]
pub struct Commit {
    pub message: String,
}

pub fn parse<P>(path: P) -> Result<Vec<Commit>, Box<dyn Error>>
where
    P: AsRef<Path>,
{
    Ok(serde_json::from_reader::<_, Push>(File::open(path.as_ref())?)?.commits)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_parses_files() -> Result<(), Box<dyn Error>> {
        let commits = parse("tests/data/push_event.json")?;
        assert!(commits.first().iter().any(|c| c.message == "test"));
        Ok(())
    }
}
