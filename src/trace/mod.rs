use std::error::Error;

mod bpftrace;

pub enum TraceTarget {
    // CPU
    CPU,
    GPU,
    IO,
    NET,
}

pub enum TraceProbe {
    KPROBE,
    TRACEPOINT,
}

pub enum TraceSubject {
    TIME,
    PID,
    TID,
    USER,
}

/*
pub enum TraceOption {
    PID(u32),
    TID(u32),
    USER(u32),
}
*/

pub enum TraceOutput {
    HIST,
    LHIST,
    SUM,
    AVG,
}

pub struct Factory {
}

impl Factory {
    fn new() -> Self {
        Factory {}
    }
}

pub trait Method {
    fn describe(&self) -> String;
    fn get_filter(&self) -> Result<Box<dyn TraceFilter>, Box<dyn Error>>;
}

pub struct Report {

}

pub trait TraceFilter {
    fn set_probe(&mut self, target: TraceProbe) -> Box<dyn TraceFilter>;
    fn set_target(&mut self, target: TraceTarget) -> Box<dyn TraceFilter>;
    fn set_subject(&mut self, subject: TraceSubject) -> Box<dyn TraceFilter>;
    fn set_output(&mut self, output: TraceOutput) -> Box<dyn TraceFilter>;
    fn search(&self) -> Result<Vec<Box<dyn Tracer>>, Box<dyn Error>>;
}

pub trait Tracer {
    fn describe(&self) -> String;
    // fn set_option(&mut self, option: TraceOption) -> Result<Box<dyn Trace>, Box<dyn Error>>;
    fn start(&mut self) -> Result<(), Box<dyn Error>>;
    fn stop(&mut self) -> Result<(), Box<dyn Error>>;
    fn publish(&mut self) -> Result<Report, Box<dyn Error>>;
}
