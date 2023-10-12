use std::error::Error;
use std::fmt::Debug;
use std::fs::File;
use std::process::Command;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Repository {
    git_dir: String,
    work_tree: String,
    name: String,
}

impl Repository {
    pub fn is_clean(&self) -> Result<bool, Box<dyn Error>> {
        let output = Command::new("git")
            .args([
                "--git-dir",
                &self.git_dir,
                "--work-tree",
                &self.work_tree,
                "status",
                "-s",
            ])
            .output()?;
        if output.status.success() {
            Ok(output.stdout.is_empty())
        } else {
            Err("Could not determine status of repository")?
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    repositories()?.iter().for_each(|r| {
        let is_clean = r.is_clean();
        match is_clean {
            Ok(true) => println!("👌 {}", r.name),
            Ok(_) => println!("👎 {}", r.name),
            Err(e) => println!("🤷 {} | {}", r.name, e),
        }
    });
    Ok(())
}

fn repositories() -> Result<Vec<Repository>, Box<dyn Error>> {
    let config_dir = dirs::config_dir().ok_or("Could not determine config dir")?;
    let path = config_dir.join("giti/repositories.json");
    let file = File::open(path)?;
    let repositories: Vec<Repository> = serde_json::from_reader(file)?;
    Ok(repositories)
}
