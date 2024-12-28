use std::{thread, time::Duration};
use sysinfo::System;

#[derive(Debug, Clone, PartialEq)]
pub struct Thread {
    pub name: String,
    pub cpu_usage: f32,
}

impl Thread {
    pub fn display(&self) -> String {
        format!("Thread: {} | Cpu usage {:.2}%", self.name, self.cpu_usage)
    }
}

pub struct CpuMetrics {
    pub system: System,
    pub cpu_brand: String,
    pub frequency: u64,
    pub threads: Vec<Thread>,
}

impl CpuMetrics {
    pub fn new(system: System) -> Self {
        let cpu_brand = system
            .cpus()
            .first()
            .map(|cpu| cpu.brand().to_string())
            .unwrap_or_else(|| "Unknown".to_string());

        let frequency = system
            .cpus()
            .first()
            .map(|cpu| cpu.frequency())
            .unwrap_or_else(|| 0);

        let threads = system
            .cpus()
            .iter()
            .map(|cpu| Thread {
                name: cpu.name().to_string(),
                cpu_usage: cpu.cpu_usage(),
            })
            .collect();

        CpuMetrics {
            system,
            cpu_brand,
            frequency,
            threads,
        }
    }

    pub fn update_threads(&mut self) {
        self.system.refresh_cpu_all();
        self.threads = self
            .system
            .cpus()
            .iter()
            .map(|cpu| Thread {
                name: cpu.name().to_string(),
                cpu_usage: cpu.cpu_usage(),
            })
            .collect();
    }
}

pub fn view() {
    let system = System::new_all();
    let mut cpus = CpuMetrics::new(system);
    let mut contador: i128 = 0;

    loop {
        contador += 1;
        println!(
            "Iteración #{} | Procesador: {} | Frecuencia: {} MHz",
            contador, cpus.cpu_brand, cpus.frequency
        );
        cpus.update_threads();

        for i in &cpus.threads {
            println!("{}", i.display());
        }

        thread::sleep(Duration::from_secs(1));
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    }
}
