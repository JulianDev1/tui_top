use sysinfo::{System, ProcessesToUpdate, ProcessRefreshKind};

struct ProcessInfo {
    pid: u32,
    name: String,
    cpu: f32,
    memory: u64,
    user: String
}

struct ProcessesList {
    pub system: System,
    pub processes: Vec<ProcessInfo>
}

impl ProcessesList {

    pub fn new(system: System) -> Self {
        
        let processes = system.processes();
        
        // ! ERROR: cannot borrow `system` as mutable because it is also borrowed as immutable
        ProcessesList {
            system,
            processes: processes.iter().map(|(_pid, process)| {
                ProcessInfo {
                    pid: process.pid().as_u32(),
                    name: process.name().to_string_lossy().into_owned(),
                    cpu: process.cpu_usage(),
                    memory: process.memory(),
                    user: process.user_id().map_or_else(|| "Unknown".to_string(), |uid| uid.to_string())
                }
            }).collect()
        }
    }
    
    fn get_processes(&self) -> &Vec<ProcessInfo> {
        &self.processes
    }
    
    fn refresh_processes(&mut self) {
        self.system.refresh_processes_specifics(ProcessesToUpdate::All, true, ProcessRefreshKind::everything() );
    }

}




pub fn view() {
    let mut sys = System::new_all();
    
    let mut processes_list = ProcessesList::new(sys);

    for process in processes_list.get_processes() {
        println!("PID {} | Nombre {} | CPU {} | memoria {} | user {}", 
            process.pid,
            process.name,
            process.cpu,
            process.memory,
            process.user
        );
    }
    processes_list.refresh_processes();

    for process in processes_list.get_processes() {
        println!("PID {} | Nombre {} | CPU {} | memoria {} | user {}", 
            process.pid,
            process.name,
            process.cpu,
            process.memory,
            process.user
        );
    }
}
