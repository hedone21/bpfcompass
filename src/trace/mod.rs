use std::error::Error;

pub enum TraceTarget {
    // CPU
    CPU_TICK,
    CPU_USAGE,

    GPU,
    IO,
    NET,
}

pub enum TraceSubject {
    SYSTEM,
    PID,
    TID,
    USER,
}

pub enum TraceOption {
    PID(u32),
    TID(u32),
    USER(u32),
    COLLECTING_MS(u32),
    SAMPLING_MS(u32),
}

pub trait Method {
    fn describe(&self) -> String;
    fn get_filter(&self) -> Result<Box<dyn TraceFilter>, Box<dyn Error>>;
}

pub struct Report {

}

pub trait TraceFilter {
    fn set_target(&mut self, target: TraceTarget) -> Box<dyn TraceFilter>;
    fn set_subject(&mut self, subject: TraceSubject) -> Box<dyn TraceFilter>;
    fn search(&self) -> Result<Vec<Box<dyn Trace>>, Box<dyn Error>>;
}

pub trait Trace {
    fn describe(&self) -> String;
    fn set_option(&mut self, option: TraceOption) -> Result<Box<dyn Trace>, Box<dyn Error>>;
    fn start(&mut self) -> Result<(), Box<dyn Error>>;
    fn stop(&mut self) -> Result<(), Box<dyn Error>>;
    fn publish(&mut self) -> Result<Report, Box<dyn Error>>;
}
