use clap::{Arg, App, value_t};

mod models;

fn main() {
    let args = App::new("crypto_check")
        .version(clap::crate_version!())
        .author("Kai Park <kaipark1201@gmail.com>")
        .about("Check Cryptos")
        .arg(Arg::with_name("token")
            .short("t")
            .long("token")
            .help("Token Value")
            .default_value("TOKEN"))
        .arg(Arg::with_name("exchange")
            .short("e")
            .long("exchange")
            .value_name("exchange")
            .default_value("upbit")
            .help("Encoding speed from 1 (best) to 10 (fast but ugly)")
            .takes_value(true))
        .get_matches();
        if let Some(i) = args.value_of("token") {
           println!("Value for input: {}", i);
       }

       if let Some(ex) = args.value_of("exchange") {
           println!("Value for config: {}", ex);
       }
}
