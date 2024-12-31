mod monitorize;

use monitorize::{general_information, process, cpu, memory, disks};

fn main () {
    memory::view();
    cpu::view();
    process::view();
    disks::view();
    general_information::view();

}