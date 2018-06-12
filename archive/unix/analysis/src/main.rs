extern crate walkdir;
extern crate plotlib;

use walkdir::WalkDir;
use plotlib::histogram::Histogram as HistPlot;
use plotlib::plot::Plot;
use plotlib::view::View;
use std::env;
use std::process::exit;


fn get_stat(path: &str, mt: u64, lt: u64) -> Vec<f64> {
    let mut stat = Vec::new();
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        match entry.metadata() {
            Ok(metadata) => if ! metadata.is_dir() && metadata.len() > mt * 1024 && metadata.len() < lt * 1024 { 
                    stat.push((metadata.len() as f64) / 1024.0);
                },
            Err(_) => continue,
        };
    };
    stat
}

fn help(){
    println!("use the following format: prog path-to-analyze file-to-output morethan lessthan");
}

fn main() {

    let path = match env::args().nth(1) {
        Some(v) => v,
        None => { help(); exit(1); }
    };
    let output = match env::args().nth(2) {
        Some(v) => v,
        None => { help(); exit(1); }
    };
    let mt = match env::args().nth(3) {
        Some(expr) => expr.parse::<u64>().unwrap(),
        None => { help(); exit(1); },
    };
    let lt = match env::args().nth(4) {
        Some(expr) => expr.parse::<u64>().unwrap(),
        None => { help(); exit(1); },
    };
            
    let stat = get_stat(path.as_str(), mt, lt);
    let hist = HistPlot::from_vec(stat.as_slice(), 50);
    let view = View::new().add(&hist);
    Plot::single(&view).save(output);
    
}
