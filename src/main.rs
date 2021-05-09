use clap::{Arg, App};
use std::fs::File;
use std::io::Read;
use freers;
fn main() {
    let matches = App::new("Program free")
        .version("0.2")
        .author("Autor: polluktus")
        .about("Poakzuje zużycie ramu")
        .usage("free [OPCJE]")
        .arg(Arg::with_name("mebi")
            .short("m")
            .long("mebi")
            .multiple(false)
            .required(false)
            .takes_value(false)
            .help("wyjście w mebibajtach"))
        .arg(Arg::with_name("bytes")
            .short("b")
            .long("bytes")
            .multiple(false)
            .required(false)
            .takes_value(false)
            .help("wyjście w bajtach"))
        .get_matches();
    let mebi = matches.is_present("mebi");
    let bytes = matches.is_present("bytes");

    let mut memf = File::open("/proc/meminfo").unwrap();
    let mut contains = String::new();
    memf.read_to_string(&mut contains).unwrap();

    if mebi == false && bytes == false{
        freers::basic(&contains);
    } else if mebi == true && bytes == false {
        freers::mebi(&contains);
    } else if mebi == false && bytes == true {
        freers::bytes(&contains);
    } else {
        println!("Zły argument");
    }
}