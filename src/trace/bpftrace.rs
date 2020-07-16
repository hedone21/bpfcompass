use std::error::Error;
use super::{Method, Tracer, TraceFilter, TraceOutput, TraceTarget, TraceSubject, Report};


pub struct Bpftrace {

}

impl Bpftrace {
    fn new() -> Self {
        Bpftrace {}
    }
}

impl Method for Bpftrace {
    fn describe(&self) -> String {
        String::new()
    }

    fn get_filter(&self) -> Result<Box<dyn TraceFilter>, Box<dyn Error>> {
        Ok(Box::new(*self))
    }
}

impl TraceFilter for Bpftrace {
    fn set_target(&mut self, target: TraceTarget) -> Box<dyn TraceFilter> {
        Box::new(*self)
    }
    fn set_subject(&mut self, subject: TraceSubject) -> Box<dyn TraceFilter> {
        Box::new(*self)
    }
    fn set_output(&mut self, output: TraceOutput) -> Box<dyn TraceFilter> {
        Box::new(*self)
    }
    fn search(&self) -> Result<Vec<Box<dyn Tracer>>, Box<dyn Error>> {
        Ok(Box::new(BpftraceTracer::new()))
    }
}

pub struct BpftraceTracer {

}

impl BpftraceTracer {
    fn new() -> Self {
        BpftraceTracer {}
    }
}

impl Tracer for BpftraceTracer {
    fn describe(&self) -> String {
        String::new()
    }

    // fn set_option(&mut self, option: TraceOption) -> Result<Box<dyn Trace>, Box<dyn Error>>;

    fn start(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn stop(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn publish(&mut self) -> Result<Report, Box<dyn Error>> {
        Report {}
    }
}
