extern crate lame_sys;
extern crate num_cpus;
use lame_sys::*;
use lame_rs::Lame;
use std::env;
use std::fs;
use std::sync::Mutex;

// add a pub struct which stores 
// Interface to LAME library
trait ILameInterface
{
    fn encode(&self, input: &str, output: &str);
}



// trait LameInterface {
//     pub fn new() -> LameInterface {
//         LameInterface {}
//     }

//     // This method basically does all the low level LAME stuff to convert a WAV file to MP3
//     // It takes an input file path and an output file path
//     pub fn encode(&self, input: &str, output: &str) {
//         unsafe {
            
//         }
//     }
// }

// worker thread which converts a file per thread
fn encode_thread(id: usize) {
    println!("Worker thread {} started", id);
    // encode thread
    let res_mutex = Mutex::new(0);
    bool done = false;
    while true {
        // get a file from the queue
        // encode the file
        // save the file
        res_mutex.lock()



        if done {
            break;
        }
    }
    
}


pub fn parse_directory(directory: &str) -> Vec<String> {
    let paths = fs::read_dir(directory).unwrap();
    let mut wav_files = Vec::<String>::new();
    for path in paths {
        let path = path.unwrap().path();
        let path_str = path.to_str().unwrap();
        if path_str.ends_with(".wav") {
            wav_files.push(path_str.to_string());
            println!("Found wav file: {}", path_str);
        }
    }
    wav_files
}

fn main() {
    // get number of threads supported by cpu
    let num_cpus = num_cpus::get();
    // TODO: later use a vector of objects to store the wav files
    let args: Vec<String> = env::args().collect();
    println!("Number of cpus: {}", num_cpus);

    let argc = args.len();
    if argc < 2 {
        println!("Usage: {} <input.wav>", args[0]);
        return;
    }

    let directory = &args[1];

    // parse directory and get all wav files
    let mut wav_files = parse_directory(directory); // array of strings

    //allocate the size of vector for the number of threads
    let mut handles = Vec::with_capacity(num_cpus);

    for i in 0..num_cpus {
        let handle = std::thread::spawn(move || {            
            encode_thread(i); // also detect which wav file is encoded by which thread. Pass a reference to the lameInterface object
        });
        handles.push(handle);
    }
    
    //walk through an array of wav files and convert them to mp3
    
}
