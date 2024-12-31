use sysinfo::{MemoryRefreshKind, System};


struct MemoryInfo {
    system: System,
    free: u64,
    used: u64,
    total: u64
}

impl MemoryInfo {

    pub fn new(system: System) -> Self {

        MemoryInfo {
            system: System::new_all(),
            used: system.used_memory(),
            total: system.total_memory(),
            free: system.total_memory() - system.used_memory(),
        }
    }

    pub fn refresh(&mut self) {
        self.system.refresh_memory_specifics(MemoryRefreshKind::everything());
        self.free = self.system.total_memory() - self.system.used_memory();
        self.used = self.system.used_memory();
    }
}


pub fn view() {

    let sys = System::new_all();
    
    let mut memory = MemoryInfo::new(sys);

    println!("free: {} | used {} | total {} ",
        memory.free / 1024 / 1024, 
        memory.used / 1024 / 1024,
        memory.total / 1024 / 1024
    );

    memory.refresh();

    println!("free: {} | used {} | total {} ",
        memory.free / 1024 / 1024, 
        memory.used / 1024 / 1024,
        memory.total / 1024 / 1024
    );
}
