use std::process::Output;

pub fn prepare_args(args:&Vec<String>, path:&String) -> Vec<String>{
    let mut args_to_execute = args.clone();
    args_to_execute.push(path.clone());
    return args_to_execute;
}

pub fn get_message(output: Output) -> String{
    return if !output.stderr.is_empty() {
        std::str::from_utf8(&output.stderr).unwrap().to_string()

    } else if !output.stdout.is_empty() {
        std::str::from_utf8(&output.stdout).unwrap().to_string()

    } else {
        "Script work done".to_string()
    };
}