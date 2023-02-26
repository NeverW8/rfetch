use std::process::Command;

fn main() {
    // Get system information
    let hostname = get_command_output("hostname".to_string());
    let os = get_command_output("cat /etc/os-release | grep PRETTY_NAME | cut -d= -f2 | tr -d '\"'".to_string());
    let uptime = get_command_output("uptime -p | sed 's/up //g'".to_string());
    let shell = get_command_output("echo $SHELL".to_string());
    let terminal = get_command_output("echo $TERM".to_string());
    let cpu = get_command_output("cat /proc/cpuinfo | grep 'model name' | uniq | cut -d: -f2 | tr -d ' '".to_string());
    let gpu = get_command_output("lspci | grep VGA | cut -d: -f3".to_string());
    let memory = get_command_output("free -h | awk 'FNR == 2 {print $2,$3}'".to_string());

    // Format the output
    let hostname = format!("Hostname: {}", hostname.trim());
    let os = format!("Operating System: {}", os.trim());
    let uptime = format!("Uptime: {}", uptime.trim());
    let shell = format!("Shell: {}", shell.trim());
    let terminal = format!("Terminal: {}", terminal.trim());
    let cpu = format!("CPU: {}", cpu.trim());
    let gpu = format!("GPU: {}", gpu.trim());
    let memory = format!("Memory: {}", memory.trim());

    // Determine the maximum length of the lines
    let lengths = [hostname.len(), os.len(), uptime.len(), shell.len(), terminal.len(), cpu.len(), gpu.len(), memory.len()];
    let max_length = lengths.iter().max().unwrap();

    // Calculate the padding
    let padding = (max_length - 2) / 2;

    // Print the box
    let separator = "+".to_string() + &"-".repeat(max_length + 2) + "+";
    let padding_spaces = " ".repeat(padding);
    let padding_chars = "-".repeat(padding);
    println!("{}", separator);
    println!("|{}{}{}|", padding_spaces, " ".repeat(max_length % 2), padding_spaces);
    println!("|{}{}{}|", padding_chars, "-".repeat(*max_length), padding_chars);
    println!("| {}{}{} |", hostname, " ".repeat(max_length - hostname.len()), padding_chars);
    println!("| {}{}{} |", os, " ".repeat(max_length - os.len()), padding_chars);
    println!("| {}{}{} |", uptime, " ".repeat(max_length - uptime.len()), padding_chars);
    println!("| {}{}{} |", shell, " ".repeat(max_length - shell.len()), padding_chars);
    println!("| {}{}{} |", terminal, " ".repeat(max_length - terminal.len()), padding_chars);
    println!("| {}{}{} |", cpu, " ".repeat(max_length - cpu.len()), padding_chars);
    println!("| {}{}{} |", gpu, " ".repeat(max_length - gpu.len()), padding_chars);
    println!("| {}{}{} |", memory, " ".repeat(max_length - memory.len()), padding_chars);
    println!("|{}{}{}|", padding_chars, "-".repeat(*max_length), padding_chars);
    println!("|{}{}{}|", padding_spaces, " ".repeat(max_length % 2), padding_spaces);
    println!("{}", separator);
}

fn get_command_output(command: String) -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("Failed to execute command");

    String::from_utf8_lossy(&output.stdout).to_string()
}
