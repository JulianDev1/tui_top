use sysinfo::{System, Disks};

pub fn view(){
    let mut disks = Disks::new_with_refreshed_list();

    for disk in &disks {
        println!(
            "{disk:?}", 
        );
    }
}
