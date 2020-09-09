use protoc_rust;
use protoc_rust::Customize;
use clap::{Arg};

fn main() {
    let matches = clap::App::new("protobuf rs generator")
        .version("0.1")
        .author("eniv <[ainvyu@gmail.com](mailto:ainvyu@gmail.com)>")
        .about("Generator")
        .arg(Arg::with_name("out_dir")
            .short("o")
            .long("out_dir")
            .value_name("out_dir")
            .help("Output Directory")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("input")
            .short("i")
            .long("input")
            .value_name("input")
            .help("Input file paths")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("include")
            .short("I")
            .long("include")
            .value_name("include")
            .help("Append a path to `-I` args")
            .required(true)
            .takes_value(true))
        .get_matches();

    let out_dir = matches.value_of("out_dir").unwrap();
    let inputs: Vec<_> = matches.values_of("input").unwrap().collect();
    let include = matches.value_of("include").unwrap();

    dbg!(&out_dir);
    dbg!(&inputs);
    dbg!(&include);

    protoc_rust::Args::new()
        .out_dir(out_dir)
        .inputs(&inputs)
        .include(include)
        .customize(Customize {
            //carllerche_bytes_for_bytes: Some(true),
            //carllerche_bytes_for_string: Some(true),
            serde_derive: Some(true),
            ..Default::default()
        })
        .run()
        .expect(include);
}
