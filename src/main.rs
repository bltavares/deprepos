use std::fs::File;
use std::io::Read;

#[derive(Debug)]
struct Git {
    destination: String,
    reference: String,
    source: String
}

trait Dependency {
    fn destination(&self) -> &str;

    fn ensure_destination_exists(&self) {
        let destination = self.destination();
        std::fs::create_dir_all(destination)
            .ok()
            .expect(&format!("Could not create the destination: {}", destination));
    }

    fn sync(&self);
}

impl Dependency for Git {
    fn destination(&self) -> &str {
        &self.destination
    }


    fn sync(&self) {
        self.ensure_destination_exists();
        println!("LOL")
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
                source: source
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
    for dep in parsed {
        dep.sync();
    }
}
