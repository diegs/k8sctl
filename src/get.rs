extern crate clap;

use self::clap::ArgMatches;

pub fn get(matches: &ArgMatches) {
    let typ = matches.value_of("type").unwrap();
    match matches.value_of("name") {
        Some(name) => println!("fetching {}/{}", typ, name),
        None => println!("fetching {}", typ),
    }
}
