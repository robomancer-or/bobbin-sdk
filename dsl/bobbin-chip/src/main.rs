extern crate clap;
extern crate bobbin_chip as chip;

use clap::{Arg, App, ArgMatches};
use std::fs::File;
use std::path::{Path, PathBuf};
use std::io::{self, Write};

use chip::{TopLevel, Device, Board, Peripheral};
use chip::reader;
use chip::builder;

pub struct AppError(i32, String);

impl From<io::Error> for AppError {
    fn from(other: io::Error) -> Self {
        AppError(1, format!("IO Error: {:?}", other))
    }
}

impl From<reader::ReadError> for AppError {
    fn from(other: reader::ReadError) -> Self {
        AppError(1, format!("ReadError: {:?}", other))
    }
}

fn main() {
    let matches = App::new("chip")
        .arg(Arg::with_name("build")
            .long("build"))     
        .arg(Arg::with_name("modules")
            .long("modules")) 
        .arg(Arg::with_name("crate")
            .long("crate")) 
        .arg(Arg::with_name("board")
            .long("board")) 
        .arg(Arg::with_name("periph")
            .long("periph")) 
        .arg(Arg::with_name("root")
            .long("root"))     
        .arg(Arg::with_name("cargo-template")
            .long("cargo-template")
            .value_name("CARGO_TEMPLATE")
            .takes_value(true)
        )     
        .arg(Arg::with_name("input"))
        .arg(Arg::with_name("output"))
        .get_matches();
        
    if !matches.is_present("input") {
        println!("{}", matches.usage());
        std::process::exit(1);
    }

    if matches.is_present("build") {
        let src = matches.value_of("input").unwrap();
        let dst = matches.value_of("output").unwrap();
        // println!("{} => {}", src, dst);
        builder::build_inner(src, dst, false, false).unwrap();
        std::process::exit(0);
    }


    let cmd = if matches.is_present("modules") {
        if !matches.is_present("output") {
            println!("No output directory specified");
            std::process::exit(1);
        }
        cmd_modules
    } else if matches.is_present("crate") {
        if !matches.is_present("output") {
            println!("No output directory specified");
            std::process::exit(1);
        }
        cmd_crate       
    } else if matches.is_present("board") {
        if !matches.is_present("output") {
            println!("No output directory specified");
            std::process::exit(1);
        }
        let board = match load_board(matches.value_of("input").unwrap()) {
            Ok(board) => board,
            Err(AppError(code, reason)) => {
                writeln!(std::io::stderr(), "Error 0x{:x}: {}", code, reason).unwrap();
                std::process::exit(code);            
            }
        };
        match cmd_board(&matches, &board) {
            Ok(_) => std::process::exit(0),
            Err(AppError(code, reason)) => {
                writeln!(std::io::stderr(), "Error 0x{:x}: {}", code, reason).unwrap();
                std::process::exit(code);
            }
        }
    } else if matches.is_present("periph") {
        if !matches.is_present("output") {
            println!("No output directory specified");
            std::process::exit(1);
        }
        let periph = match load_periph(matches.value_of("input").unwrap()) {
            Ok(periph) => periph,
            Err(AppError(code, reason)) => {
                writeln!(std::io::stderr(), "Error 0x{:x}: {}", code, reason).unwrap();
                std::process::exit(code);            
            }
        };
        match cmd_periph(&matches, &periph) {
            Ok(_) => std::process::exit(0),
            Err(AppError(code, reason)) => {
                writeln!(std::io::stderr(), "Error 0x{:x}: {}", code, reason).unwrap();
                std::process::exit(code);
            }
        }
    } else {
        println!("{}", matches.usage());
        std::process::exit(1);
    };

    let device = match load_device(matches.value_of("input").unwrap()) {
        Ok(device) => device,
        Err(AppError(code, reason)) => {
            writeln!(std::io::stderr(), "Error 0x{:x}: {}", code, reason).unwrap();
            std::process::exit(code);            
        }
    };
    match cmd(&matches, &device) {
        Ok(_) => {},
        Err(AppError(code, reason)) => {
            writeln!(std::io::stderr(), "Error 0x{:x}: {}", code, reason).unwrap();
            std::process::exit(code);
        }
    }
}

fn cmd_modules(matches: &ArgMatches, device: &Device) -> Result<(), AppError> {
    Ok(try!(chip::codegen::gen_modules(matches, &mut std::io::stdout(), &device)))
}

fn cmd_crate(matches: &ArgMatches, device: &Device) -> Result<(), AppError> {
    let cfg = chip::codegen::crates::Config {
        out_path: PathBuf::from(matches.value_of("output").expect("No output path specified")),
        cargo_template:  PathBuf::from(matches.value_of("cargo-template").expect("Required parameter cargo-template missing")),
    };
    Ok(try!(chip::codegen::gen_crate(cfg, &mut std::io::stdout(), &device)))
}

fn cmd_board(matches: &ArgMatches, board: &Board) -> Result<(), AppError> {
    let cfg = chip::codegen::board::Config {
        out_path: PathBuf::from(matches.value_of("output").expect("No output path specified")),
        cargo_template:  PathBuf::from(matches.value_of("cargo-template").expect("Required parameter cargo-template missing")),
    };
    Ok(try!(chip::codegen::gen_board(cfg, &mut std::io::stdout(), &board)))
}

fn cmd_periph(matches: &ArgMatches, periph: &Peripheral) -> Result<(), AppError> {
    let cfg = chip::codegen::periph::Config {
        out_path: PathBuf::from(matches.value_of("output").expect("No output path specified")),
        cargo_template:  PathBuf::from(matches.value_of("cargo-template").expect("Required parameter cargo-template missing")),
    };
    Ok(try!(chip::codegen::gen_periph(cfg, &mut std::io::stdout(), &periph)))
}


fn load_device<P: AsRef<Path>>(p: P) -> Result<Device, AppError> {
    let mut input = try!(File::open(&p));
    match try!(reader::read(&mut input, p.as_ref())) {
        TopLevel::Device(device) => Ok(device),
        TopLevel::Board(_) => Err(AppError(1, format!("Expected Device, got Board"))),
        TopLevel::Peripheral(_) => Err(AppError(1, format!("Expected Device, got Peripheral"))),
    }
}

fn load_board<P: AsRef<Path>>(p: P) -> Result<Board, AppError> {
    let mut input = try!(File::open(&p));
    match try!(reader::read(&mut input, p.as_ref())) {
        TopLevel::Device(_) => Err(AppError(1, format!("Expected Board, got Device"))),
        TopLevel::Board(board) => Ok(board),
        TopLevel::Peripheral(_) => Err(AppError(1, format!("Expected Board, got Peripheral"))),
    }
}

fn load_periph<P: AsRef<Path>>(p: P) -> Result<Peripheral, AppError> {
    let mut input = try!(File::open(&p));
    match try!(reader::read(&mut input, p.as_ref())) {
        TopLevel::Device(_) => Err(AppError(1, format!("Expected Peripheral, got Device"))),
        TopLevel::Board(_) => Err(AppError(1, format!("Expected Peripheral, got Board"))),
        TopLevel::Peripheral(periph) => Ok(periph),
    }
}