pub struct Options<'a> {
    pub typ: &'a str,
    pub name: Option<&'a str>,
    pub kubeconfig: &'a str,
    pub namespace: &'a str,
    pub output: &'a str,
}

pub fn get(opts: Options) {
    match opts.name {
        Some(name) => println!("fetching {}/{}", opts.typ, name),
        None => println!("fetching {}", opts.typ),
    };
    println!(
        "with options: kubeconfig={}, namespace={}, output={}",
        opts.kubeconfig,
        opts.namespace,
        opts.output
    );
}
