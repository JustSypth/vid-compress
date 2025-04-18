pub fn cli() {
    let args: Vec<String> = std::env::args().collect();
    let mut args_iter = args.into_iter().skip(1);

    while let Some(flag) = args_iter.next() {
        match flag.to_lowercase().as_str() {
            "--version" => version(),
            _ => todo!(),
        }
    }
}

fn version() {
    let package_name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    println!("{} {}", package_name, version);

    std::process::exit(0);
}