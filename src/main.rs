#[macro_use]
use log;
use pretty_env_logger;

mod trace;

fn main() {
    pretty_env_logger::init();

    let bpf_factory = trace::Factory::new();
    let bpf_method = bpf_factory.create("BPFTRACE").unwrap();
    log::info!("bpf_method desc: {}", bpf_method.describe());
    let mut bpf_filter = bpf_method.get_filter().unwrap();
    let probes = bpf_filter.list_all_probes();

    log::info!("list all probes");
    for probe in probes {
        log::info!(" => {}", probe)
    }

    log::info!("find exec tracer with kprobe");
    let mut probe = Vec::new();
    probe.push("kprobe");
    probe.push("_do_fork");
    bpf_filter.set_probe(probe);
    let bpf_tracers = bpf_filter.search().unwrap();
    if bpf_tracers.len() != 1 {
        log::error!("kprobe::_do_fork bpf should be only one");
    }

    let mut bpf_tracer = bpf_tracers.get(0).unwrap();
    log::info!("find matched bpf tracer");
    log::info!(" => {}", bpf_tracer.describe());

}
