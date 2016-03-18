extern crate getopts;
extern crate langid;

use std::env;
use std::process::exit;
use std::fs::File;
use std::io;
use std::io::{Read, Write, stdout, stderr};

use getopts::Options;

use langid::model::Model;
use langid::classifier::Classifier;


const COMMANDS: &'static str = "
Commands:
    train [-o FILE] <FILE FILE...>   train a model from text
    classify [FILE]                  classifiy text based on pre-trained models
";


type ExitCode = i32;


fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    if args.len() == 1 {
        exit(print_usage(&program));
    }

    let command = args[1].clone();
    let exit_code = match command.as_str() {
        "train" => train(&program, args),
        "classify" => classify(&program, args),
        _ => { print_usage(&program) }
    };
    exit(exit_code);
}


fn print_usage(program: &str) -> ExitCode {
    let brief = format!("Usage: {} <command> [options]", program);
    print!("{}\n{}", brief, COMMANDS);
    1
}


fn print_command_usage(program: &str, command: &str, opts: Options) -> ExitCode {
    let brief = format!("Usage: {} {} [options]", program, command);
    print!("{}", opts.usage(&brief));
    1
}


fn train(program: &str, args: Vec<String>) -> ExitCode {
    let mut opts = Options::new();
    opts.optopt("o", "output", "write the trained model into a file instead of stdout", "FILE");

    let matches = opts.parse(&args[2..]).unwrap();
    let input_filenames = &matches.free;
    let output_filename = matches.opt_str("output");

    if input_filenames.len() == 0 {
        return print_command_usage(program, "train", opts);
    }

    let mut text = String::new();
    for filename in input_filenames {
        match read_file(&filename, &mut text) {
            Ok(_) => {},
            Err(error) => {
                return die(&format!("{}: {}", filename, error));
            },
        };
    }

    let model = Model::build_from_text(&text);
    let contents = model.serialize();
    return match output_filename {
        Some(filename) => {
            match write_file(&filename, &contents) {
                Ok(_) => 0,
                Err(error) => die(&format!("{}: {}", filename, error)),
            }
        },
        None => {
            match stdout().write_all(&contents) {
                Ok(_) => 0,
                Err(error) => panic!("{}", error),
            }
        },
    }
}


fn classify(program: &str, args: Vec<String>) -> ExitCode {
    die("Not implemented")
}


// utilities

fn write_file(filename: &String, contents: &Vec<u8>) -> io::Result<()> {
    let mut file = try!(File::create(filename));
    Ok(try!(file.write_all(contents)))
}


fn read_file(filename: &String, buf: &mut String) -> io::Result<usize> {
    let mut file = try!(File::open(filename));
    let bytes = try!(file.read_to_string(buf));
    Ok(bytes)
}


fn die(message: &str) -> ExitCode {
    writeln!(&mut stderr(), "{}", message).unwrap();
    1
}
