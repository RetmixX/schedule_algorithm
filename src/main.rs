mod config;
mod utils;

use std::process::Command;
use std::time::{Duration};
use job_scheduler::{Job, JobScheduler};
use crate::config::load_env;
use chrono::Local;
use crate::utils::{get_message, prepare_args};

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
    let args_to_execute = prepare_args(&args, &path_script);

    let result_execute = Command::new(command)
        .args(args_to_execute)
        .output()
        .unwrap();

    let time = Local::now();
    let message = get_message(result_execute);
    println!("[{}]  - {}", time.format("%Y-%m-%d %H:%M:%S") ,message);
}