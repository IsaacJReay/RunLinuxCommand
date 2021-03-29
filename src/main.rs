pub fn ls(arguments: &str) -> (i32, String, String){
    let options = run_script::ScriptOptions::new();
    let (code, output, error) = run_script::run_script!(
        &format!("ls -lah {}", arguments),
        &vec![],
        &options
    ).unwrap();

    (code, output, error)
}

fn main() {
    let path: &str = "/usr/bin";
    let (code, output, error) = ls(path);

    match &code {
        1 => println!("{}", &error),
        0 => println!("{}", &output),
    }


}