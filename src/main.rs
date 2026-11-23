use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let version = env!("RELEASE_VERSION");
    match args.as_slice() {
        [flag] if flag == "--version" => println!("testing-csm-ci-client {version}"),
        [command, flag] if command == "version" && flag == "--json" => println!(r#"{{"version":"{version}"}}"#),
        _ => std::process::exit(2),
    }
}
