use sysinfo::System;

pub fn view() {

    let mut sys = System::new_all();
    sys.refresh_all();
    
    println!(
        "free: {} | used: {} | Total {} ",
        sys.free_memory() / 1024 / 1024,
        sys.used_memory() / 1024 / 1024,
        sys.total_memory() / 1024/ 1024,
    );
}
