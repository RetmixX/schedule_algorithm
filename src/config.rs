use std::env;
use std::fs::File;
use std::io::ErrorKind;
use dotenv::dotenv;
pub fn load_env() -> (String, String, Vec<String>, usize) {
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

fn check_exist_file(path: &String){
    match File::open(&path) {
        Ok(_) => {}
        Err(error_kind) => match error_kind.kind(){
            ErrorKind::NotFound => panic!("File not found by path - {}", &path),
            ErrorKind::PermissionDenied => println!("Current user not has permission for work file"),
            other=> panic!("Cannot open file - {}", other.to_string())
        }
    }
}

fn arg_to_vec(args:String)->Vec<String>{
    args.split(",").map(|arg| arg.to_string())
        .collect::<Vec<String>>()
        .iter().filter(|item| !item.is_empty()).cloned()
        .collect::<Vec<String>>()
}