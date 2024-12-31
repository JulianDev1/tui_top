use sysinfo::Disks;

pub struct LocalDisk {
    pub mount_point: String,
    pub name: String,
    pub kind: String,
    pub total_space: u64,
    pub available_space: u64,
}

pub struct DisksList {
    pub disks: Vec<LocalDisk>,
}

impl DisksList {
    pub fn new(disks: Disks) -> Self {
        let disks = disks
            .iter()
            .map(|disk| LocalDisk {
                mount_point: disk.mount_point().to_str().unwrap().to_owned(),
                name: disk
                    .name()
                    .to_str()
                    .unwrap_or_else(|| "Local Disk")
                    .to_owned(),
                kind: disk.kind().to_string(),
                total_space: disk.total_space(),
                available_space: disk.available_space(),
            })
            .collect();

        DisksList { disks }
    }

    pub fn update_disks(&mut self) {
        let disks = Disks::new_with_refreshed_list();
        self.disks = disks
            .iter()
            .map(|disk| LocalDisk {
                mount_point: disk.mount_point().to_str().unwrap().to_owned(),
                name: disk
                    .name()
                    .to_str()
                    .unwrap_or_else(|| "Local Disk")
                    .to_owned(),
                kind: disk.kind().to_string(),
                total_space: disk.total_space(),
                available_space: disk.available_space(),
            })
            .collect();
    }
}

pub fn view() {
    let disks = Disks::new_with_refreshed_list();
    let disks_list = DisksList::new(disks);
    
    for disk in disks_list.disks.iter() {
        println!(
            "Disk: {}{} | Kind: {} | Total Space: {:.2} GB | Available Space: {:.2} GB",
            disk.mount_point,
            disk.name,
            disk.kind,
            bytes_to_gb(disk.total_space),
            bytes_to_gb(disk.available_space)
        );
    }
}

pub fn bytes_to_gb(bytes: u64) -> f32 {
    bytes as f32 / 1024.0 / 1024.0 / 1024.0
}
