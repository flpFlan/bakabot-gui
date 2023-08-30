use git2::Repository;

fn main() {
    loop {
        if let Ok(_) = Repository::open("./bakabot") {break;}
        if let Ok(_) = Repository::clone("https://github.com/flpFlan/bakabot", "./bakabot") {break;}
        std::process::exit(1);
    }
}
