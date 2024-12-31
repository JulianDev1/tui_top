use sysinfo::System;

#[derive(Debug)]
pub struct SystemMetrics {
    pub cpu_usage: f32,
    pub memory_used: f32,
    pub memory_total: f32,
    pub top_processes: Vec<ProcessInfo>,
}

#[derive(Debug)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub memory_usage: u64,
}

#[derive(Debug)]
pub struct SystemInfoCollector {
    system: System,
}

impl SystemInfoCollector {
    // Constructor
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();
        SystemInfoCollector { system }
    }

    pub fn collect_metrics(&mut self) -> SystemMetrics {
        self.system.refresh_all();

        let cpu_usage = self
            .system
            .cpus()
            .iter()
            .map(|cpu| cpu.cpu_usage())
            .sum::<f32>()
            / self.system.cpus().len() as f32;

        let memory_used = self.system.used_memory() as f32;
        let memory_total = self.system.total_memory() as f32;

        let top_processes = self
            .system
            .processes()
            .iter()
            .map(|(pid, process)| ProcessInfo {
                pid: pid.as_u32(),
                name: process.name().to_string_lossy().into_owned(),
                cpu_usage: process.cpu_usage(),
                memory_usage: process.memory(),
            })
            .collect::<Vec<_>>();

        SystemMetrics {
            cpu_usage,
            memory_used,
            memory_total,
            top_processes,
        }
    }
}
