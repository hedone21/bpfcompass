use std::error::Error;
use super::{Method, Tracer, TraceFilter, Report};

#[derive(Copy, Clone)]
pub struct Bpftrace {

}

impl Bpftrace {
    pub fn new() -> Self {
        Bpftrace {}
    }
}

impl Method for Bpftrace {
    fn describe(&self) -> &str {
        "temp"
    }

    fn get_filter(&self) -> Result<Box<dyn TraceFilter>, Box<dyn Error>> {
        Ok(Box::new(*self))
    }
}

impl TraceFilter for Bpftrace {
    fn set_probe(&mut self, probe: Vec<&str>) -> Box<dyn TraceFilter> {

        Box::new(*self)
    }

    fn list_all_probes(&self) -> Vec<&str> {

        let mut v = Vec::new();
        v.push("temp");
        v
    }

    fn list_matched_probes(&self, probe: Vec<&str>) -> Vec<&str> {

        let mut v = Vec::new();
        v.push("temp");
        v
    }

    fn search(&self) -> Result<Vec<Box<dyn Tracer>>, Box<dyn Error>> {
        let s = Box::new(BpftraceTracer::new());
        let mut v = Vec::<Box<dyn Tracer>>::new();
        v.push(s);

        Ok(v)
    }
}

#[derive(Copy, Clone)]
pub struct BpftraceTracer {

}

impl BpftraceTracer {
    pub fn new() -> Self {
        BpftraceTracer {}
    }
}

impl Tracer for BpftraceTracer {
    fn describe(&self) -> &str {
        "bpftrace tracer"
    }

    fn get_probe(&self) -> Vec<&str> {
        let mut v = Vec::new();
        v.push("temp");
        v
    }

    fn set_output_format(&mut self, format: &str) -> Box<dyn Tracer> {

            Box::new(*self)
    }

    fn list_available_output_format(&self) -> Vec<&str> {

        let mut v = Vec::new();
        v.push("temp");
        v
    }

    fn set_output_vars(&mut self, output: Vec<&str>) -> Box<dyn Tracer> {

            Box::new(*self)
    }

    fn list_avaliable_output_vars(&self) -> Vec<&str> {

        let mut v = Vec::new();
        v.push("temp");
        v
    }

    fn start(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn stop(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn publish(&mut self) -> Result<Report, Box<dyn Error>> {
        Ok(Report {})
    }
}
