use clap::{Parser};
use sysinfo::{System, Networks, Disks};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Parser)]
#[clap(version = "1.0.0", author = "Ashmit athawait.work@gmail.com", about = "Displays system information")]

struct Cli {
    #[clap(long, help = "Store your name")]
    name: Option<String>,
    #[clap(short, long, help = "Show CPU usage")]
    cpu: bool,
    #[clap(short, long, help = "Show memory usage")]
    memory: bool,
    #[clap(short, long, help = "Show disk usage")]
    disk: bool,
    #[clap(short, long, help = "Show network usage")]
    network: bool,
}

#[derive(Serialize, Deserialize)]
struct UserData {
    name: String,
}

fn main() {
    let args = Cli::parse();

    let mut system = System::new_all();
    system.refresh_all();

    if let Some(name) = args.name {
        let user_data = UserData { name };
        let file_path = "user_data.json";

        // Save the name to a JSON file
        fs::write(file_path, serde_json::to_string(&user_data).unwrap()).expect("Unable to write file");

        println!("\nNice to meet you {}!", user_data.name);
    }

    // Display stored name if it exists
    let file_path = "user_data.json";
    if Path::new(file_path).exists() {
        let data: UserData = serde_json::from_str(&fs::read_to_string(file_path).expect("Unable to read file")).expect("Unable to parse JSON");
        println!("\n---------- Welcome to Peekaboo! ----------");
        println!("\nMonitoring {}'s System...", data.name);
    }
    else {
        println!("\n---------- Welcome to Peekaboo! ----------");
        println!("\nTip - Peekaboo can remember your name for personalised outputs! Just run 'peek --name YOUR_NAME'.");
    }

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
