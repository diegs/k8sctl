extern crate clap;

use self::clap::ArgMatches;

pub fn get(matches: &ArgMatches) {
    let typ = matches.value_of("type").unwrap();
    match matches.value_of("name") {
        Some(name) => println!("fetching {}/{}", typ, name),
        None => println!("fetching {}", typ),
    }
    let kubeconfig = matches.value_of("kubeconfig").unwrap_or("no kubeconfig");
    let namespace = matches.value_of("namespace").unwrap_or("no namespace");
    let output = matches.value_of("output").unwrap_or("");
    println!(
        "with options: kubeconfig={}, namespace={}, output={}",
        kubeconfig,
        namespace,
        output
    )
}
