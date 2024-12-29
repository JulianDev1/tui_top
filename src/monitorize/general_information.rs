use sysinfo::System;

pub struct GeneralInfo {
    name: String,
    os_version: String,
    long_os_version: String,
    kernel_version: String,
    host_name: String,
}

impl GeneralInfo {
    pub fn new() -> Self {
        GeneralInfo {
            name: System::name().unwrap(),
            os_version: System::os_version().unwrap(),
            long_os_version: System::long_os_version().unwrap(),
            kernel_version: System::kernel_version().unwrap(),
            host_name: System::host_name().unwrap(),
        }
    }
}

pub fn view() {
    let info = GeneralInfo::new();

    println!("System Name:             {:?}", info.name);
    println!("System kernel version:   {:?}", info.kernel_version);
    println!("System OS version:       {:?}", info.os_version);
    println!("System host name:        {:?}", info.host_name);
    println!("System long OS version:  {:?}", info.long_os_version);
}
