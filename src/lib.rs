//#![allow(non_snake_case)]
//#![allow(unused)]

//extern crate walkdir;
//extern crate regex;

use std::path::PathBuf;
use walkdir::WalkDir;
use regex::Regex;
//use std::usize;
use pyo3::prelude::*;

//#[pyfunction]
//fn sum_as_string(a: usize, b: usize) -> PyResult<String>{
//    Ok((a+b).to_string())
//}

#[pymodule]
fn xb_walk(_py: Python, m: &PyModule) -> PyResult<()>{
//    m.add_function(wrap_pyfunction!(xb_walk, m)?)?;
    #[pyfn(m)]
    fn walk(path: &str, filter: &str) -> Vec<String> {
        let path = PathBuf::from(path);
        let pattern = Regex::new(filter).unwrap();
        let mut matched_paths: Vec<String> = Vec::new();
    
        for entry in WalkDir::new(path) {
            let entry = match entry {
                Ok(e) => e,
                Err(_err) => {
                    //                println!("Error while walking the directory: {}", err);
                    continue;
                }
            };
    
//            let file_name = entry.file_name().to_string_lossy();
            if pattern.is_match(&entry.file_name().to_string_lossy()) {
                matched_paths.push(entry.path().to_string_lossy().to_string()); // entry.into_path()
            }
        }
    
        matched_paths
    }
    Ok(())
}