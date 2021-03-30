pub fn sudomkdir(path: &str) -> (i32, String, String){
    let options = run_script::ScriptOptions::new();
    let (code, output, error) = run_script::run_script!(
        &format!(r#"sudo mkdir {}"#, path),
        &vec![],
        &options
    ).unwrap();

    (code, output, error)
}

pub fn ls(path: &str) -> (i32, String, String){
    let options = run_script::ScriptOptions::new();
    let (code, output, error) = run_script::run_script!(
        &format!(r#"ls -lah {}"#, path),
        &vec![],
        &options
    ).unwrap();

    (code, output, error)
}

pub fn backgroundls(path: &str) -> String{
    let options = run_script::ScriptOptions::new();
    let _child = run_script::spawn_script!(
        &format!(r#"ls -lah {}"#, path),
        &vec![],
        &options
    ).unwrap();

    format!("PID: {}", _child.id())
}