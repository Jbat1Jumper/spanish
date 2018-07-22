/// This is an experiment around a spanish corups I've found in Kaggler.
///
/// For information about what this does and how to run this use the `run` script located in
/// the root of this crate.
///
/// TODO: Create that run file :squirrel:
///

#[macro_use]
extern crate log;
extern crate simple_logger;
extern crate xml;
extern crate zip;

use std::result;

/// This is an attempt for a controlled way of developing code. If we are writting many functions
/// there will be some times when we may need to compile the code but there could be some
/// unimplemented functions or some of their cases. 
///
/// To work around this without fighting the compiler we can make those funcitons return a Result.
/// If the function is unimplemented we can return an error (which can also be typed). In that way
/// we can still compile our code and also track where are the unimplemented parts of the code.
///
enum ErrorCause {
    TODO,
}

/// This is a wrapper around the result with our known errors.
///
type Result<T> = result::Result<T, ErrorCause>;


use std::fs::File;
use std::hash::Hash;
use std::io::Read;
use zip::read::ZipArchive;


/// This'll be the object with which we'll work. It requires the same traits as the markov
/// chain token.
///
trait Word: Eq + Hash + Clone {}
impl Word for String {}


fn main() {
    simple_logger::init().unwrap();
    match read_zip() {
        Ok(_words) => {
            info!("Here are words");
        },
        Err(_err) => {
            error!("Uffa");
        }
    }
}


/// This function is the first approach to reading this corpus, just extracting the words in a
/// giant vector.
///
/// One of the possible improvements would be to return an iterator to not parse all the words
/// at the same time.
///
fn read_zip() -> Result<Vec<String>> {
    info!(
        "Opening the Spanish Corupus zip..."
    );
    let file = File::open("spanish_corpus.utf8.zip").unwrap();
    let mut zip = ZipArchive::new(file).unwrap();

    info!(
        "Reading files..."
    );
    for i in 0..zip.len() {
        let mut file = zip.by_index(i).unwrap();

        debug!("- File with index {}: {}", i, file.name());
        debug!("  - size = {}",            file.size());
        debug!("  - compressed_size = {}", file.compressed_size());
        debug!("  - last_modified = {}",   file.last_modified().ctime());
    };
    
    Err(ErrorCause::TODO)
}

/// This function takes a source from where to read the xml file, and a target where it should
/// write the fixed version.
///
/// This is because there are some errors with the XML format, caused mostly by unescaped
/// characters.
///
/// Returns Ok if it was successful, and an ErrorCause if not.
///
fn fix_file(source: &mut Read, target: &mut Write) -> Result<()> {
    Err(ErrorCause::TODO)
}


use std::io::{Write, BufReader};

use xml::ParserConfig;
use xml::reader::Error;

fn get_first_error_in_file(source: &mut Read) -> Option<Error> {
    let reader = ParserConfig::new()
        .whitespace_to_characters(true)
        .ignore_comments(false)
        .create_reader(BufReader::new(source));

    for e in reader {
        match e {
            Err(e) => {
                return Some(e)
            },
            _ => { },
        }
    }
    None
}

