use clap::{Parser};
use sysinfo::{System, Networks, Disks};

#[derive(Parser)]
#[clap(version = "1.0.0", author = "Ashmit athawait.work@gmail.com", about = "Displays system information")]

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
        println!("Total: {} GB", system.total_memory() / (1024*1024*1024));
        println!("Used: {} GB", system.used_memory() / (1024*1024*1024));
        println!("Free: {} GB", system.free_memory() / (1024*1024*1024));
    }

    if args.disk {
        println!("\nDisk Usage:");
        let disks = Disks::new_with_refreshed_list();

        for disk in &disks {
            println!("{}: {} / {} GB available", disk.name().to_str().unwrap(), disk.available_space() / (1024*1024*1024), disk.total_space() / (1024*1024*1024));
        }
    }

    if args.network {
        println!("\nNetwork Usage:");
        let networks = Networks::new_with_refreshed_list();

        for (interface_name, data) in &networks {
            println!("{}: received {} bytes, transmitted {} bytes", interface_name, data.total_received() , data.total_transmitted());
        }
    }
    println!("")
}
