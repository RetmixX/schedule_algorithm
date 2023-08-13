use std::env;
use std::path::Path;
use dotenv::dotenv;
pub fn load_env() -> (String, String, Vec<String>, usize) {
    return get_value_from_env();
}

fn check_exist_file(path: &String){
    match Path::new(&path).exists() {
        true => {}
        false => panic!("File by path - {} not found", path)
    }
}

fn arg_to_vec(args:String)->Vec<String>{
    args.split(",").map(|arg| arg.to_string())
        .collect::<Vec<String>>()
        .iter().filter(|item| !item.is_empty()).cloned()
        .collect::<Vec<String>>()
}

fn get_value_from_env() -> (String, String, Vec<String>, usize) {
    dotenv().ok();
    let script_location = env::var("EXECUTE_FILE")
        .expect("EXECUTE_FILE not set");

    let duration_second = env::var("DURATION_SECOND")
        .expect("DURATION_SECOND not set").parse::<usize>()
        .expect("DURATION_SECOND not number");

    let command = env::var("COMMAND")
        .expect("COMMAND not set");

    let args = env::var("ARGS")
        .expect("ARGS not set");
    check_exist_file(&script_location);

    return (script_location, command, arg_to_vec(args), duration_second);
}