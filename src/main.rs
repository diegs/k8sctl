#[macro_use]

extern crate clap;

use clap::AppSettings;
use clap::Arg;
use clap::SubCommand;

fn main() {
    let matches = app_from_crate!(", ")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg(
            Arg::with_name("kubeconfig")
                .value_name("KUBECONFIG")
                .long("kubeconfig")
                .help("Path to the kubeconfig file to use for CLI requests.")
                .takes_value(true)
                .default_value("")
                .hide_default_value(true),
        )
        .arg(
            Arg::with_name("namespace")
                .value_name("NAMESPACE")
                .long("namespace")
                .short("n")
                .help("If present, the namespace scope for this CLI request.")
                .takes_value(true)
                .default_value("")
                .hide_default_value(true),
        )
        .subcommand(
            SubCommand::with_name("get")
                .about("Display one or many resources.")
                .arg(Arg::with_name("type").value_name("TYPE").required(true))
                .arg(Arg::with_name("name").value_name("NAME").required(false))
                .arg(
                    Arg::with_name("output")
                        .value_name("output")
                        .long("output")
                        .short("o")
                        .help("Output format")
                        .takes_value(true)
                        .possible_values(&["json", "yaml", "wide", "name"]),
                ),
        )
        .get_matches();

    match matches.subcommand_name() {
        Some("get") => println!("ok, coming right up"),
        _ => (),
    }
}
