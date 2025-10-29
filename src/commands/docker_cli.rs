use std::process::Command;

pub fn check_docker_installation() -> Result<bool, String> {
    let docker_output = Command::new("docker")
        .arg("version")
        .output()
        .expect("Failed to execute docker command");

    if !docker_output.status.success() {
        return Err("Docker is not installed or is not running".to_string());
    }

    return Ok(true);
}

pub fn get_docker_output(command: &str) -> Result<String, String> {
    check_docker_installation()?;

    let docker_output = Command::new("docker")
        .arg(command)
        .output()
        .expect("Failed to execute docker command");

    if docker_output.status.success() {
        return Ok(String::from_utf8(docker_output.stdout).expect("Failed to convert docker output to string"));
    } else {
        return Err(String::from_utf8(docker_output.stderr).expect("Failed to convert docker output to string"));
    }
}