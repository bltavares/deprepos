use std::fs::File;
use std::io::Read;

#[derive(Debug)]
enum Dependency {
    Git {
        destination: String,
        reference: String,
        source: String
    }
}

impl<'a> Dependency {
    fn destination(&'a self) -> &'a str {
        match *self {
            Dependency::Git { destination: ref d, .. } => d
        }
    }

    fn ensure_destination_exists(&'a self) {
        let destination = self.destination();
        std::fs::create_dir_all(self.destination()).ok().expect(&format!("Could not create the destination: {}", destination));
    }

    fn sync(&'a self) {
        self.ensure_destination_exists();
        match *self {
            Dependency::Git { .. }  => println!("LOL")
        }
    }
}

fn dep_content() -> String {
    let mut file = File::open("repo.deps").ok().expect("Can't read dependency files");
    let mut content = String::new();
    file.read_to_string(&mut content).ok().expect("Couldn't read all dependency file content");

    content
}

fn parse_file(s : &str) -> Vec<Dependency> {
    s.lines().map(|dep| {
        let mut line_words = dep.split_whitespace();
        let repo_type = line_words.next().expect("First argument should be: git");
        let destination = String::from(line_words.next().expect("Second argument should be a destination"));
        let reference = String::from(line_words.next().expect("Third argument should be a reference"));
        let source = String::from(line_words.next().expect("Fourth argument should be a source url"));

        match repo_type {
            "git" => Dependency::Git {
                destination: destination,
                reference: reference,
                source: source
            },
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
        println!("{:?}", dep);
        dep.sync();
    }
}
