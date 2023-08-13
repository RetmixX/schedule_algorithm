mod config;
use std::process::{Command, Output};
use std::time::{Duration};
use job_scheduler::{Job, JobScheduler};
use crate::config::load_env;
use chrono::Local;

fn main() {
    let (location, command, args, duration) = load_env();
    let schedule_timeline = format!("1/{} * * * * *", duration);
    let mut schedule = JobScheduler::new();
    schedule.add(Job::new(schedule_timeline.parse().unwrap(),||{
        execute_command(&command, &args, &location);
    }));

    loop {
        schedule.tick();
        std::thread::sleep(Duration::from_secs(duration as u64));
    }
}

fn execute_command(command:&String, args:&Vec<String>, path_script:&String){
    let mut args_to_execute = args.clone();
    args_to_execute.push(path_script.clone());
    let result_execute = Command::new(command)
        .args(args_to_execute)
        .output()
        .unwrap();

    let time = Local::now();
    let message = get_message(result_execute);
    println!("[{}]  - {}", time.format("%Y-%m-%d %H:%M:%S") ,message);
}

fn get_message(output: Output) -> String{
    return if !output.stderr.is_empty() {
        std::str::from_utf8(&output.stderr).unwrap().to_string()

    } else if !output.stdout.is_empty() {
        std::str::from_utf8(&output.stdout).unwrap().to_string()

    } else {
        "Script work done".to_string()
    };
}