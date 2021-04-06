pub fn sudomkdir(path: &str) -> (i32, String, String){
    let options = run_script::ScriptOptions::new();
    let _command = r#"sudo mkdir path"#;
    let command = _command.replace("path", path);
    let (code, output, error) = run_script::run_script!(
        &format!("{}", command),
        &vec![],
        &options
    ).unwrap();

    (code, output, error)
}

pub fn ls(path: &str) -> (i32, String, String){
    let options = run_script::ScriptOptions::new();
    let _command = r#"ls -lah path"#;
    let command = _command.replace("path", path);
    let (code, output, error) = run_script::run_script!(
        &format!("{}", command),
        &vec![],
        &options
    ).unwrap();

    (code, output, error)
}

pub fn backgroundls(path: &str) -> String{
    let options = run_script::ScriptOptions::new();
    let _command = r#"ls -lah path"#;
    let command = _command.replace("path", path);
    let _child = run_script::spawn_script!(
        &format!("{}", command),
        &vec![],
        &options
    ).unwrap();

    format!("PID: {}", _child.id())
}