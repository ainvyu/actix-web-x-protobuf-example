#[cfg(feature = "with-bytes")]
extern crate bytes;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate protobuf;

use clap::{Arg};

mod handler;
mod server;
mod test;
mod custom_serde;

fn main() {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

    let matches = clap::App::new("Protobuf Example")
        .version("0.1")
        .author("eniv <[ainvyu@gmail.com](mailto:ainvyu@gmail.com)>")
        .about("Receive application/x-protobuf")
        .arg(Arg::with_name("port")
            .short("p")
            .long("port")
            .value_name("port")
            .help("Listen port")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("web_workers")
            .short("wc")
            .long("web_workers")
            .value_name("web_workers")
            .help("Web Workers count")
            .default_value("1024")
            .required(true)
            .takes_value(true))
        .get_matches();

    let port = matches.value_of("port").unwrap_or("8080");
    let web_workers = matches.value_of("web_workers").unwrap().parse()
        .expect("Web worker value is integer value");

    dbg!(port);
    dbg!(web_workers);

    server::run(port,
                web_workers);
}
