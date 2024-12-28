use sysinfo::{
    System, Cpu
};

use std::thread;
use std::time::Duration;

pub fn view() {
    let mut sys = System::new_all();

    // get the brand (marca) of processor
    let cpu_brand = sys.cpus()[0].brand();

    println!("{}\n", cpu_brand);

    thread::sleep(Duration::from_secs(1)); // Espera 1 segundo

    for _ in 0..3 {
        // Segunda actualización después del intervalo
        sys.refresh_all();
        for cpu in sys.cpus() {
           println!("cpu: {} | usage: {} | frequency: {} ", 
               cpu.name(), 
               cpu.cpu_usage(), 
               cpu.frequency(),
           );
        }
        // le da 1 seg de tiempo al programa antes de la siguiente iteracion
        thread::sleep(Duration::from_secs(1));
    } 
}
