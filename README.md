# Peekaboo

Peekaboo is a **System Monitoring CLI tool** built in Rust. It allows you to monitor various system parameters such as CPU usage, memory usage, disk usage, and network usage.

![Peekaboo](./assets/system.png)

## Features

- Monitor CPU usage
- Monitor memory usage
- Monitor disk usage
- Monitor network usage
- Save and display user names

## Installation

1. Clone the repository:
    ```sh
    git clone https://github.com/ashmit0920/Peekaboo.git
    ```

2. Extract the cloned directory, and add it to your environment's `PATH` variable for easy access:
    1. Search for "Edit environment variables" in your Windows Search bar and click on the search result.
    2. Under "System Variables" (for all users), select "Path" => "Edit". In case you lack the permissions to edit System variables, just edit the Path variable under "User Variables".
    3. A table pops up showing the directories included in the current PATH. Click on "New" ⇒ "Browse..." to select the cloned Peekaboo directory. Click on ⇒ "OK" (Don't "Cancel") ⇒ "OK" ⇒ "OK".
    4. (For older Windows) If you didn't see a pop-up table, it is time to change your computer.

## Usage

- To display the help menu:
    ```sh
    peek -h
    ```
- To store a user name:
    ```sh
    peek --name <name>
    ```
    The user name will be stored and will be displayed on subsequent runs.

- To display system information
    ```sh
    peek -s
    ```

- To monitor CPU usage:
    ```sh
    peek -c
    ```

- To monitor memory usage:
    ```sh
    peek -m
    ```

- To monitor disk usage:
    ```sh
    peek -d
    ```

- To monitor network usage:
    ```sh
    peek -n
    ```

- To monitor all of the above at once:
    ```sh
    peek --showall
    ```

## Examples

#### CPU Usage:

![CPU usage](./assets/cpu.png)

#### Memory and Disk usage:

![Memory and Disk usage](./assets/memory%20and%20disk.png)

#### Network usage:

![Network Usage](./assets/network.png)

#### Show all parameters

![Show all](./assets/showall.png)