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
        let repo_status = status(&self.repo);
        let status = match repo_status {
            Status::Clean => String::from("clean"),
            Status::Dirty => String::from("dirty"),
        };
        Manifest {
            status: status
        }
    }
}

#[derive(Debug,PartialEq)]
enum Status {
    Clean,
    Dirty
}

fn status(repo: &Repository) -> Status {
    let clean = Status::Clean;
    let dirty = Status::Dirty;

    let state: RepositoryState = repo.state();
    if state != RepositoryState::Clean {
        return dirty;
    };

    let lib_statuses = repo.statuses(None).unwrap();
    let count = lib_statuses.iter().count();
    if count > 0 {
        return dirty;
    } else {
        return clean;
    }
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
        // In a clean repo
        let (_td, repo) = repo_init();

        // We should see a "clean" status from our function
        let status: super::Status = super::status(&repo);
        assert_eq!(status, super::Status::Clean);
    }

    /// Tests that a repository with a new file shows "dirty" for status
    #[test]
    fn test_status_dirty() {
        // In a repo with an untracked file
        let (td, repo) = repo_init();
        File::create(&td.path().join("test")).unwrap();

        // We should have one untracked file in the upstream statuses
        let lib_statuses = repo.statuses(None).unwrap();
        assert_eq!(lib_statuses.iter().count(), 1);

        // We should see a "dirty" status from our function
        let status: super::Status = super::status(&repo);
        assert_eq!(status, super::Status::Dirty);
    }
}
