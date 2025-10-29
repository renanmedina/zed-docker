use crate::commands::docker_cli;

pub struct Container {
    pub id: String,
    pub name: String,
    pub image: String,
}

const CONTAINER_FORMAT: &str = "{{.ID}} {{.Names}} {{.Image}}";

fn parse_container_output(output: &str) -> Vec<Container> {
    return output
    .lines()
    .filter(|line| !line.is_empty())
    .map(|line| {
        let parts = line
        .split_whitespace()
        .filter(|part| !part.is_empty())
        .map(|part| part.to_string())
        .collect::<Vec<String>>();
        Container {
            id: parts[0].to_string(),
            name: parts[1].to_string(),
            image: parts[2].to_string(),
        }
    })
    .collect::<Vec<Container>>();
}

fn list_containers(command_params: &str) -> Result<Vec<Container>, String> {
    let containers_output = docker_cli::get_docker_output(&command_params);
    match containers_output {
        Ok(output) => {
            let containers = parse_container_output(&output);
            return Ok(containers);
        }
        Err(error) => {
            return Err(error);
        }
    }
}

pub fn list_up_containers() -> Result<Vec<Container>, String> {
    return list_containers(&format!("ps --format '{}'", CONTAINER_FORMAT));
}

pub fn list_all_containers() -> Result<Vec<Container>, String> {
    return list_containers(&format!("ps -a --format '{}'", CONTAINER_FORMAT));
}

pub fn list_down_containers() -> Result<Vec<Container>, String> {
    return list_containers(&format!("ps -f status=exited --format '{}'", CONTAINER_FORMAT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_container_output() {
        let output = "1234567890 container1 image1\n1234567890 container2 image2\n";
        let containers = parse_container_output(&output);
        assert_eq!(containers.len(), 2);
        assert_eq!(containers[0].id, "1234567890");
        assert_eq!(containers[0].name, "container1");
        assert_eq!(containers[0].image, "image1");
        assert_eq!(containers[1].id, "1234567890");
        assert_eq!(containers[1].name, "container2");
        assert_eq!(containers[1].image, "image2");
    }
}
