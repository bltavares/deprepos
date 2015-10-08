extern crate git2;

use std::fs::File;
use std::io::Read;
use git2::Repository;

struct Git {
    destination: String,
    reference: String,
    source: String,
    repo: Option<git2::Repository>,
}

trait Dependency {
    fn destination(&self) -> &str;

    fn ensure_destination_exists(&self) {
        let destination = self.destination();
        std::fs::create_dir_all(destination).unwrap();
    }

    fn sync(&mut self);
}

impl Git {
    fn needs_cloning(&self) -> bool {
        let mut git_location = std::path::PathBuf::from(&self.destination);
        git_location.push(".git");
        std::fs::metadata(git_location).is_err()
    }

    fn clone(&mut self) {
        let result = git2::Repository::clone(&self.source, &self.destination);
        self.repo = Some(result.unwrap());
    }

    fn update(&mut self) {
        let error_message = &format!("Couldn't find origin remote on {}", &self.destination);
        let mut remote = self.repo().find_remote("origin").ok().expect(error_message);
        remote.fetch(&[], None, None).unwrap();
    }

    fn checkout(&mut self) {
        let mut checkout_options = git2::build::CheckoutBuilder::new();
        checkout_options.force();

        let rev_reference = self.reference.clone();
        let repo = self.repo();
        let oid = repo.revparse_single(&rev_reference).unwrap();

        repo.set_head_detached(oid.id()).unwrap();
        repo.reset(&oid, git2::ResetType::Hard, Some(&mut checkout_options)).unwrap();
    }

    fn repo(&mut self) -> &Repository {
        match self.repo {
            Some(ref repo) => repo,
            None => {
                self.repo = Repository::open(&self.destination).ok();
                self.repo()
            }
        }
    }
}

impl Dependency for Git {
    fn destination(&self) -> &str {
        &self.destination
    }


    fn sync(&mut self) {
        self.ensure_destination_exists();

        if self.needs_cloning() {
            println!("Needs cloning");
            self.clone();
        } else {
            println!("Needs fetch");
            self.update();
        }
        self.checkout();
    }
}

fn dep_content() -> String {
    let mut file = File::open("repo.deps").ok().expect("Can't read dependency files");
    let mut content = String::new();
    file.read_to_string(&mut content).ok().expect("Couldn't read all dependency file content");

    content
}

fn parse_file(s : &str) -> Vec<Box<Dependency>> {
    s.lines().map(|dep| {
        let mut line_words = dep.split_whitespace();
        let repo_type = line_words.next().expect("First argument should be: git");
        let destination = String::from(line_words.next().expect("Second argument should be a destination"));
        let reference = String::from(line_words.next().expect("Third argument should be a reference"));
        let source = String::from(line_words.next().expect("Fourth argument should be a source url"));


        match repo_type {
            "git" => Box::new(Git {
                destination: destination,
                reference: reference,
                source: source,
                repo: None
            }) as Box<Dependency>,
            _ => {
                panic!("Can't handle repositories of type: {}", repo_type);
            }
        }

    }).collect()
}

fn main() {
    let content = dep_content();
    let parsed = parse_file(&content);
    for mut dep in parsed {
        dep.sync();
    }
}
