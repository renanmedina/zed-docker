use zed_extension_api::{
    self as zed, SlashCommand, SlashCommandArgumentCompletion, SlashCommandOutput, Worktree
};

struct DockerExtension;

impl DockerExtension {
    fn load_containers(&self) -> Result<Vec<SlashCommandArgumentCompletion>, String> {
        return Ok(vec![
            SlashCommandArgumentCompletion {
                label: "Container 1".to_string(),
                new_text: "container-1".to_string(),
                run_command: true,
            },
            SlashCommandArgumentCompletion {
                label: "Container 2".to_string(),
                new_text: "container-2".to_string(),
                run_command: true,
            },
            SlashCommandArgumentCompletion {
                label: "Container 3".to_string(),
                new_text: "container-3".to_string(),
                run_command: true,
            },
        ]);
    }

    fn start_container(&self, command: SlashCommand) -> Result<SlashCommandOutput, String> {
        println!("Starting container: {:?}", command);
        return Ok(SlashCommandOutput{
            text: "Container started".to_string(),
            sections: vec![],
        });
    }

    fn stop_container(&self, command: SlashCommand) -> Result<SlashCommandOutput, String> {
        println!("Stopping container: {:?}", command);
        return Ok(SlashCommandOutput {
            text: "Container stopped".to_string(),
            sections: vec![],
        });
    }

    fn inspect_container(&self, command: SlashCommand) -> Result<SlashCommandOutput, String> {
        println!("Inspecting container: {:?}", command);
        return Ok(SlashCommandOutput {
            text: "Container inspected".to_string(),
            sections: vec![],
        });
    }

    fn restart_container(&self, command: SlashCommand) -> Result<SlashCommandOutput, String> {
        println!("Restarting container: {:?}", command);
        return Ok(SlashCommandOutput {
            text: "Container restarted".to_string(),
            sections: vec![],
        });
    }
}

impl zed::Extension for DockerExtension {
    fn new() -> Self {
        DockerExtension
    }

    fn run_slash_command(&self, command: SlashCommand, _args: Vec<String>, _worktree: Option<&Worktree>) -> Result<SlashCommandOutput, String> {
        match command.name.as_str() {
            "start" => self.start_container(command),
            "stop" => self.stop_container(command),
            "inspect" => self.inspect_container(command),
            "restart" => self.restart_container(command),
            command => Err(format!("unknown slash command: \"{command}\"")),
        }
    }

    fn complete_slash_command_argument(&self, command: SlashCommand, _args: Vec<String>) 
        -> Result<Vec<SlashCommandArgumentCompletion>, String> {
        match command.name.as_str() {
            "start" => self.load_containers(),
            "stop" => self.load_containers(),
            "inspect" => self.load_containers(),
            "restart" => self.load_containers(),
            command => Err(format!("unknown slash command: \"{command}\"")),
        }
    }
}

zed::register_extension!(DockerExtension);