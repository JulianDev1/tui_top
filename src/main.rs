use system_info::SystemMetrics;

mod event;
mod system_info;

mod monitorize;

use monitorize::{general_information, process, cpu, memoria, disks};

fn main () {
    memoria::view();
    cpu::view();
    process::view();
    disks::view();
    general_information::view();

}


//fn main() {
//    let mut sis = system_info::SystemInfoCollector::new();
//
//    //println!("{:?}", sis.collect_metrics());
//    let memory_used = sis.collect_metrics().memory_used;
//    let memory_used = (memory_used / 1024.0 / 1024.0);
//    println!("{}", memory_used);
//
//}
