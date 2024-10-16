use gadget_sdk::executor::process::manager::GadgetProcessManager;
use std::collections::HashMap;
use std::error::Error;

/// Function to run multiple commands and focus on the output of each command.
///
/// This function takes a GadgetProcessManager and a list of commands to run.
/// It runs each command using the manager and focuses on the output of each command.
/// The output of each command is stored in a HashMap with the command name as the key.
///
/// # Arguments
///
/// * `manager` - A mutable reference to the GadgetProcessManager used to run the commands.
/// * `commands` - A vector of tuples containing the command name and the command to run.
///
/// # Returns
///
/// Returns a Result containing a HashMap with the output of each command, or an error.
///
/// # Example
///
/// ```
/// let mut manager = GadgetProcessManager::new();
/// let commands = vec![
///    ("command1", "echo 'Hello World'"),
///    ("command2", "ls -l"),
/// ];
/// let outputs = run_and_focus_multiple(&mut manager, commands).await?;
/// ```
async fn run_and_focus_multiple<'a>(
    manager: &mut GadgetProcessManager,
    commands: Vec<(&'a str, &'a str)>,
) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let mut outputs = HashMap::new();
    for (name, command) in commands {
        let service = manager.run(name.to_string(), command).await?;
        let output = manager.focus_service_to_completion(service).await?;
        outputs.insert(name.to_string(), output);
    }
    Ok(outputs)
}

/// Runs a Gaia node and returns the outputs of each step along with the public URL.
///
/// This function performs the following steps:
/// 1. Installs the Gaia node binary
/// 2. Sources the updated bashrc
/// 3. Initializes the Gaia node
/// 4. Starts the Gaia node
///
/// # Returns
///
/// Returns a tuple containing:
/// - `()`: An empty tuple as the first element
/// - `HashMap<String, String>`: A map of step names to their outputs
///
/// The HashMap includes a "public_url" key with the Gaia node's public URL.
///
/// # Errors
///
/// This function will return an error if:
/// - Any of the commands fail to execute
/// - The public URL cannot be extracted from the output
///
/// # Example
///
/// ```
/// let (_, outputs) = run_gaia_node().await?;
/// println!("Gaia node public URL: {}", outputs.get("public_url").unwrap());
/// ```
pub async fn run_gaia_node(
    manager: &mut GadgetProcessManager,
) -> Result<((), HashMap<String, String>), Box<dyn Error>> {
    let commands = vec![
        ("binary_install", "curl -sSfL 'https://github.com/GaiaNet-AI/gaianet-node/releases/latest/download/install.sh' | bash"),
        ("source_dir", "source ~/.bashrc"),
        ("init_agai", "gaianet init"),
        ("start_gaia", "gaianet start"),
    ];

    let mut outputs = run_and_focus_multiple(manager, commands).await?;

    // Extract the public URL from the start_gaia output
    let public_url = outputs
        .get("start_gaia")
        .and_then(|output: &String| {
            output
                .lines()
                .find(|line| line.contains("https://") && line.contains(".gaianet.xyz"))
                .map(|line| line.trim().to_string())
        })
        .ok_or_else(|| Box::<dyn Error>::from("Failed to extract public URL"))?;

    println!("Gaia node public URL: {}", public_url);

    // You can return the public_url if needed
    outputs.insert("public_url".to_string(), public_url);

    Ok(((), outputs))
}

/// Stops the Gaia node using the GadgetProcessManager.
///
/// This function executes the 'gaianet stop' command to stop the Gaia node.
///
/// # Arguments
///
/// * `manager` - A mutable reference to the GadgetProcessManager used to run the command.
///
/// # Returns
///
/// Returns a Result containing:
/// - `()`: An empty tuple as the first element
/// - `HashMap<String, String>`: A map of step names to their outputs
///
/// The HashMap includes a "stop_gaia" key with the output of the stop command.
///
/// # Errors
///
/// This function will return an error if:
/// - The 'gaianet stop' command fails to execute
///
/// # Example
///
/// ```
/// let mut manager = GadgetProcessManager::new();
/// let (_, outputs) = stop_gaia_node(&mut manager).await?;
/// println!("Stop command output: {}", outputs.get("stop_gaia").unwrap());
/// ```
pub async fn stop_gaia_node(
    manager: &mut GadgetProcessManager,
) -> Result<((), HashMap<String, String>), Box<dyn Error>> {
    let commands = vec![("stop_gaia", "gaianet stop")];

    let outputs = run_and_focus_multiple(manager, commands).await?;
    Ok(((), outputs))
}

pub async fn upgrade_gaia_node(
    manager: &mut GadgetProcessManager,
) -> Result<((), HashMap<String, String>), Box<dyn Error>> {
    let commands = vec![
        ("stop_gaia", "gaianet stop"),
        ("upgrade_gaia_node", "curl -sSfL 'https://github.com/GaiaNet-AI/gaianet-node/releases/latest/download/install.sh' | bash -s -- --upgrade"),
        ("init_agai", "gaianet init"),
        ("start_gaia", "gaianet start"),
    ];

    let outputs = run_and_focus_multiple(manager, commands).await?;
    Ok(((), outputs))
}

/// Updates the Gaia node configuration and restarts the node.
///
/// This function updates the specified configuration parameters of the Gaia node,
/// reinitializes it, and then starts it again.
///
/// # Arguments
///
/// * `manager` - A mutable reference to the GadgetProcessManager used to run the commands.
/// * `config_updates` - A slice of tuples containing the configuration keys and their new values.
///
/// # Returns
///
/// Returns a Result containing:
/// - `()`: An empty tuple as the first element
/// - `HashMap<String, String>`: A map of step names to their outputs
///
/// The HashMap includes keys for each updated configuration parameter, as well as
/// "init_gaia" and "start_gaia" with their respective command outputs.
///
/// # Errors
///
/// This function will return an error if:
/// - Any of the configuration updates are invalid (checked by `validate_config_command`)
/// - Any of the `gaianet` commands fail to execute
///
/// # Example
///
/// ```
/// let mut manager = GadgetProcessManager::new();
/// let config_updates = vec![
///     ("chat-url", "https://new-chat-url.com"),
///     ("embedding-ctx-size", "1024"),
/// ];
/// let (_, outputs) = update_gaia_config(&mut manager, &config_updates).await?;
/// println!("Update outputs: {:?}", outputs);
/// ```

pub async fn update_gaia_config(
    manager: &mut GadgetProcessManager,
    config_updates: &[(&str, &str)],
) -> Result<((), HashMap<String, String>), Box<dyn Error>> {
    let mut commands: Vec<(String, String)> = Vec::new();

    // Validate all config commands
    for (key, value) in config_updates {
        validate_config_command(key, value)?;
    }

    // Generate a single config command with all updates
    let mut config_command = String::from("gaianet config");
    for (key, value) in config_updates {
        config_command.push_str(&format!(" \\\n  --{} {}", key, value));
    }

    commands.push(("update_config".to_string(), config_command));

    commands.push(("init_gaia".to_string(), "gaianet init".to_string()));
    commands.push(("start_gaia".to_string(), "gaianet start".to_string()));

    // Convert commands into a Vec<(&str, &str)>
    let commands: Vec<(&str, &str)> = commands
        .iter()
        .map(|(key, value)| (key.as_str(), value.as_str()))
        .collect();

    let outputs = run_and_focus_multiple(manager, commands).await?;
    Ok(((), outputs))
}

/// Validates a configuration command for the Gaia node.
///
/// This function validates the key and value of a configuration command for the Gaia node.
/// It checks if the key is a known configuration parameter and if the value is valid.
///
/// # Arguments
///
/// * `key` - A string slice containing the configuration key.
/// * `value` - A string slice containing the configuration value.
///
/// # Returns
///
/// Returns a Result containing:
/// - `()` if the configuration command is valid.
///
/// # Errors
///
/// This function will return an error if:
/// - The key is not a known configuration parameter.
/// - The value is invalid for the specified key.
///
/// # Example
///
/// ```
/// let key = "chat-url";
/// let value = "https://new-chat-url.com";
/// validate_config_command(key, value)?;
/// ```
pub fn validate_config_command(key: &str, value: &str) -> Result<(), Box<dyn Error>> {
    match key {
        "chat-url" | "embedding-url" | "snapshot" => {
            if value.starts_with("http://") || value.starts_with("https://") {
                // Validate URL structure
                if let Err(_) = url::Url::parse(value) {
                    return Err(format!("Invalid URL structure for {}: {}", key, value).into());
                }
            } else {
                // Check if it's a local file under $HOME/gaianet
                let home_dir = std::env::var("HOME").unwrap_or_default();
                let gaia_path = std::path::Path::new(&home_dir).join("gaianet");
                let file_path = std::path::Path::new(value);
                if !file_path.exists() || !file_path.starts_with(&gaia_path) {
                    return Err(format!("Invalid value for {}: {}. It should be a valid URL or a local file under $HOME/gaianet", key, value).into());
                }
            }
        }
        "chat-ctx-size" | "embedding-ctx-size" | "port" => {
            value
                .parse::<u32>()
                .map_err(|_| format!("Invalid number for {}: {}", key, value))?;
        }
        "prompt-template" | "system-prompt" | "rag-prompt" | "reverse-prompt" => {
            // These are strings, so no validation needed
        }
        "base" => {
            // Validate if the path exists
            if !std::path::Path::new(value).exists() {
                return Err(format!("Invalid path for base: {}", value).into());
            }
        }
        "qdrant-limit" => {
            let limit = value
                .parse::<u32>()
                .map_err(|_| format!("Invalid number for qdrant-limit: {}", value))?;
            if limit == 0 {
                return Err("qdrant-limit must be greater than 0".into());
            }
        }
        "qdrant-score-threshold" => {
            let threshold = value
                .parse::<f32>()
                .map_err(|_| format!("Invalid number for qdrant-score-threshold: {}", value))?;
            if threshold < 0.0 || threshold > 1.0 {
                return Err("qdrant-score-threshold must be between 0.0 and 1.0".into());
            }
        }
        "rag-policy" => {
            match value {
                "system-message" | "last-user-message" => {
                    // These are valid options, no further validation needed
                }
                _ => return Err(format!("Invalid rag-policy value: {}. Must be either 'system-message' or 'last-user-message'", value).into()),
            }
        }
        _ => return Err(format!("Unknown config key: {}", key).into()),
    }
    Ok(())
}
