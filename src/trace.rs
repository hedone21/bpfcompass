use std::error::Error;
use std::fmt;

mod bpftrace;

#[derive(Debug, Clone)]
struct FactoryError;

impl fmt::Display for FactoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cannot find method")
    }
}

impl Error for FactoryError {}

pub struct Factory {
}

impl Factory {
    pub fn new() -> Self {
        Factory {}
    }

    pub fn create(&self, method: &str) -> Result<Box<dyn Method>, Box<dyn Error>> {
        if method == "BPFTRACE" {
            Ok(Box::new(bpftrace::Bpftrace::new()))
        }else {
            Err(FactoryError.into())
        }
    }
}

pub trait Method {
    fn describe(&self) -> &str;
    fn get_filter(&self) -> Result<Box<dyn TraceFilter>, Box<dyn Error>>;
}

pub struct Report {

}

pub trait TraceFilter {
    fn set_probe(&mut self, probe: Vec<&str>) -> Box<dyn TraceFilter>;
    fn list_all_probes(&self) -> Vec<&str>;
    fn list_matched_probes(&self, probe: Vec<&str>) -> Vec<&str>;
    fn search(&self) -> Result<Vec<Box<dyn Tracer>>, Box<dyn Error>>;
}

pub trait Tracer {
    fn describe(&self) -> &str;
    fn get_probe(&self) -> Vec<&str>;
    fn set_output_format(&mut self, format: &str) -> Box<dyn Tracer>;
    fn list_available_output_format(&self) -> Vec<&str>;
    fn set_output_vars(&mut self, output: Vec<&str>) -> Box<dyn Tracer>;
    fn list_avaliable_output_vars(&self) -> Vec<&str>;
    fn start(&mut self) -> Result<(), Box<dyn Error>>;
    fn stop(&mut self) -> Result<(), Box<dyn Error>>;
    fn publish(&mut self) -> Result<Report, Box<dyn Error>>;
}
