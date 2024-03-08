use std::path::PathBuf;
use git2::{Repository,RepositoryState,RepositoryOpenFlags};
use serde::Serialize;

#[derive(Serialize)]
pub struct Manifest {
    status: String,
    //hash: String,
    //long_hash: String,
    //origin: String,
    //tags: Vec<String>,
    //branch: Vec<String>,
}

pub struct Repo {
    repo: Repository,
}

impl Repo {
    pub fn new(path: Option<PathBuf>) -> Repo {
        let path: PathBuf = match path {
            Some(path) => path,
            None => PathBuf::from("."),
        };
        let flags = RepositoryOpenFlags::empty();
        let repo: Repository = Repository::open_ext(path, flags, &[] as &[&std::ffi::OsStr]).unwrap();
        Repo {
            repo: repo,
        }
    }

    pub fn manifest(&self) -> Manifest {
        let status = status(&self.repo);
        Manifest {
            status: status
        }
    }
}

fn status(repo: &Repository) -> String {
    let state: RepositoryState = repo.state();
    format!("{:?}", state).to_lowercase()
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use tempfile::TempDir;
    use git2::{Repository,RepositoryInitOptions};

    fn repo_init() -> (TempDir, Repository) {
        let td = TempDir::new().unwrap();
        let mut opts = RepositoryInitOptions::new();
        opts.initial_head("main");
        let repo = Repository::init_opts(td.path(), &opts).unwrap();
        {
            let mut config = repo.config().unwrap();
            config.set_str("user.name", "name").unwrap();
            config.set_str("user.email", "email").unwrap();
            let mut index = repo.index().unwrap();
            let id = index.write_tree().unwrap();

            let tree = repo.find_tree(id).unwrap();
            let sig = repo.signature().unwrap();
            repo.commit(Some("HEAD"), &sig, &sig, "initial\n\nbody", &tree, &[])
                .unwrap();
        }
        (td, repo)
    }

    /// Tests that a newly configured repository shows "clean" for status
    #[test]
    fn test_status_clean() {
        let (td, repo) = repo_init();
        let status: String = super::status(&repo);
        assert_eq!(status, "clean");
    }


    #[test]
    fn test_status() {
        let (td, repo) = repo_init();
        assert_eq!(repo.statuses(None).unwrap().len(), 0);
        File::create(&td.path().join("test")).unwrap();
        let statuses = repo.statuses(None).unwrap();
        assert_eq!(statuses.iter().count(), 1);
        let status = statuses.iter().next().unwrap();
        assert_eq!(status.path(), Some("test"));
        let string_status = super::status(&repo);
        assert_eq!(string_status, "clean");
    }
}
