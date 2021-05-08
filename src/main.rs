use clap::{Arg, App};
use std::fs::File;
use std::io::Read;
use freers;
fn main() {
    let matches = App::new("Program free")
        .version("0.1")
        .author("Autor: polluktus")
        .about("Poakzuje zużycie ramu")
        .usage("free [OPCJE]")
        .arg(Arg::with_name("human")
            .short("h")
            .long("human")
            .multiple(false)
            .required(false)
            .takes_value(false)
            .help("wyjście czytelne dla człowieka"))
        .get_matches();
    let human = matches.is_present("human");

    let mut memf = File::open("/proc/meminfo").unwrap();
    let mut contains = String::new();
    memf.read_to_string(&mut contains).unwrap();

    if human == false {
        freers::basic(&contains);
    } else {
        freers::human(&contains);
    }
}