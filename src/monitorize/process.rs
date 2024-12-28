
use sysinfo;

struct ProcessInfo {
    pid: u8,
    name: String,
    cpu_usage: f32,
    user_id: u8,

}

struct ProcessesList {
    processes: Vec<ProcessInfo>
}

impl ProcessInfo {

    
}


pub fn view() {
    let mut sys = sysinfo::System::new_all();
    sys.refresh_all();


    for proc in sys.processes() {
        let process = proc.1; // select the &Process
        println!(
            "PID: {} | Nombre: {} | CPU: {} | memoria: {} | user {:?} ",
            process.pid(),    
            process.name().to_string_lossy(),
            process.cpu_usage(),
            process.memory(),
            process.user_id()
            
        );
    }
}
