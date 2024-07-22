use clap::{Parser};
use sysinfo::{System, Networks, Disks};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use colored::*;

#[derive(Parser)]
#[clap(version = "1.0.0", author = "Ashmit athawait.work@gmail.com", about = "Displays system information")]

struct Cli {
    #[clap(long, help = "Store your name")]
    name: Option<String>,
    #[clap(short, long, help = "Show full PC Information")]
    info: bool,
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
    let mut args = Cli::parse();

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

    if args.info {
        println!("\n{}\n", "System Information:".red().bold());

        if let Some(sys_name) = System::name() {
            println!("System Name: {}", sys_name);
        }
        if let Some(kernel_version) = System::kernel_version() {
            println!("System kernel version: {}", kernel_version);
        }
        if let Some(os_version) = System::os_version() {
            println!("System OS version: {}", os_version);
        }
        if let Some(host_name) = System::host_name() {
            println!("System host name: {}", host_name);
        }

        args.cpu = true;
        args.memory = true;
        args.disk = true;
        args.network = true;
    }

    if args.cpu {
        println!("\n{}\n", "CPU Usage:".blue().bold());
        for processor in system.cpus() {
            println!("{}: {:.2} %", processor.name(), processor.cpu_usage());
        }
    }

    if args.memory {
        println!("\n{}\n", "Memory Usage:".cyan().bold());
        println!("Total: {} GB", system.total_memory() / (1024*1024*1024));
        println!("Used: {} GB", system.used_memory() / (1024*1024*1024));
        println!("Free: {} GB", system.free_memory() / (1024*1024*1024));
    }

    if args.disk {
        println!("\n{}\n", "Disk Usage:".green().bold());
        let disks = Disks::new_with_refreshed_list();

        for disk in &disks {
            println!("{}: {} / {} GB available", disk.name().to_str().unwrap(), disk.available_space() / (1024*1024*1024), disk.total_space() / (1024*1024*1024));
        }
    }

    if args.network {
        println!("\n{}\n", "Network Usage:".yellow().bold());
        let networks = Networks::new_with_refreshed_list();

        for (interface_name, data) in &networks {
            println!("{}: received {} bytes, transmitted {} bytes", interface_name, data.total_received() , data.total_transmitted());
        }
    }
    println!("")
}
