use zed_extension_api::{
    self as zed, SlashCommand, SlashCommandArgumentCompletion, SlashCommandOutput, Worktree
};
mod commands;
use commands::*;

struct DockerExtension;

impl DockerExtension {
    fn present_containers(&self, containers: Vec<containers::Container>) -> Result<Vec<SlashCommandArgumentCompletion>, String> {
        let completions = containers.iter().map(|container| SlashCommandArgumentCompletion {
            label: container.name.clone(),
            new_text: container.name.clone(),
            run_command: true,
        }).collect();

        return Ok(completions);
    }

    fn present_up_containers(&self) -> Result<Vec<SlashCommandArgumentCompletion>, String> {
       return self.present_containers(containers::list_up_containers()?);
    }

    fn present_down_containers(&self) -> Result<Vec<SlashCommandArgumentCompletion>, String> {
        return self.present_containers(containers::list_down_containers()?);
    }

    fn present_all_containers(&self) -> Result<Vec<SlashCommandArgumentCompletion>, String> {
        return self.present_containers(containers::list_all_containers()?);
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
            "start" => self.present_up_containers(),
            "stop" => self.present_down_containers(),
            "inspect" => self.present_all_containers(),
            "restart" => self.present_up_containers(),
            command => Err(format!("unknown slash command: \"{command}\"")),
        }
    }
}

zed::register_extension!(DockerExtension);