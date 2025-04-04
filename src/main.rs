use std::fs;
use std::path::Path;

use git2::{Repository, Signature, Time};

/// This is a simple script to test what do github takes into account when generating the activity
/// widget (The green dot matrix). Does it take into account the commiter or the author? Both?
/// Unsigned commits too?
fn main() {
    // declare different dates where I had no commits pushed to my github account
    let dates = [
        Time::new(1742502044, 0), // 2025-03-20
        Time::new(1742588444, 0), // 2025-03-21
        Time::new(1742674844, 0), // 2025-03-22
        Time::new(1742761244, 0), // 2025-03-23
    ];

    // Declare signers:

    let signer_me = Signer::new(
        "Joaquín P. Centeno".into(),
        "jpcenteno@users.noreply.github.com".into(),
    );

    let signer_dummy = Signer::new("John Doe".into(), "johndoe@example.org".into());

    // Create a repository instance.
    let path = Path::new("/tmp/tmp.W5kiMYLlyT");
    let repo = &Repository::open(path).expect("Failed to open repo");

    // Make a commit where I'm neither the author nor the commiter.
    commit(&dates[0], &signer_dummy, &signer_dummy, repo);
    // Make a commit where I'm the author but not the commiter.
    commit(&dates[1], &signer_dummy, &signer_me, repo);
    // Make a commit where I'm the commiter but not the author.
    commit(&dates[2], &signer_me, &signer_dummy, repo);
    // Make a commit where I'm both the commiter and the author.
    commit(&dates[3], &signer_me, &signer_me, repo);

    // TODO Push the changes to GitHub.
    // TODO Manually check the github activity.
}

fn commit(time: &Time, commiter: &Signer, author: &Signer, repo: &Repository) {
    // Create a file to commit.
    let path = Path::new("test_file.txt");
    fs::write(&path, "Test content").expect("Failed to write file");

    // Create a index for the commit:
    let tree = {
        let mut index = repo.index().unwrap();
        index.add_path(&path).unwrap();
        index.write().unwrap();

        let tree_oid = index.write_tree().unwrap();

        repo.find_tree(tree_oid).unwrap()
    };

    // Get the parent commit. This needs an initial commit to work.
    let parent_commit = {
        let head_ref = repo.head().ok();
        head_ref
            .as_ref()
            .and_then(|h| h.target())
            .and_then(|oid| repo.find_commit(oid).ok())
            .unwrap() // Will fail if there is no initial commit.
    };

    // Commit:
    repo.commit(
        Some("HEAD"),
        &author.new_signature(time),
        &commiter.new_signature(time),
        "Test commit", // FIXME
        &tree,
        &vec![&parent_commit],
    )
    .unwrap();
}

#[derive(Debug)]
struct Signer {
    name: String,
    email: String,
}

impl Signer {
    pub fn new(name: String, email: String) -> Self {
        Self { name, email }
    }

    pub fn new_signature(&self, time: &git2::Time) -> Signature {
        Signature::new(&self.name, &self.email, time).unwrap()
    }
}
