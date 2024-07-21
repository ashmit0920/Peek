use clap::{Parser};
use sysinfo::{System, Networks, Disks};

#[derive(Parser)]
#[clap(version = "1.0", author = "Your Name <youremail@example.com>", about = "Displays system information")]

struct Cli {
    #[clap(short, long, help = "Show CPU usage")]
    cpu: bool,
    #[clap(short, long, help = "Show memory usage")]
    memory: bool,
    #[clap(short, long, help = "Show disk usage")]
    disk: bool,
    #[clap(short, long, help = "Show network usage")]
    network: bool,
}

fn main() {
    let args = Cli::parse();

    let mut system = System::new_all();
    system.refresh_all();

    if args.cpu {
        println!("\nCPU Usage:");
        for processor in system.cpus() {
            println!("{}: {} %", processor.name(), processor.cpu_usage());
        }
    }

    if args.memory {
        println!("\nMemory Usage:");
        println!("Total: {} KB", system.total_memory());
        println!("Used: {} KB", system.used_memory());
        println!("Free: {} KB", system.free_memory());
    }

    if args.disk {
        println!("\nDisk Usage:");
        let disks = Disks::new_with_refreshed_list();

        for disk in &disks {
            println!("{}: {} / {} bytes", disk.name().to_str().unwrap(), disk.available_space(), disk.total_space());
        }
    }

    if args.network {
        println!("\nNetwork Usage:");
        let networks = Networks::new_with_refreshed_list();

        for (interface_name, data) in &networks {
            println!("{}: received {} bytes, transmitted {} bytes", interface_name, data.total_received(), data.total_transmitted());
        }
    }
}
