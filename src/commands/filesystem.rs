// File System Commands Implementation
// This file contains pwd, cd, and cat commands that interact with the backend server

use super::{Command, CommandResult};
use serde::{Deserialize, Serialize};
use wasm_bindgen::{JsValue, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

#[derive(Serialize)]
struct CommandRequest {
    session_id: String,
    command: String,
    args: Vec<String>,
}

#[derive(Deserialize)]
struct CommandResponse {
    success: bool,
    output: String,
    current_dir: String,
}

/// Generate a session ID for the terminal (should be called once per terminal instance)
pub fn generate_unique_session_id() -> String {
    // Generate a unique session ID per browser tab/window using timestamp + random
    // This ensures each terminal instance has its own virtual file system state
    use web_sys::window;
    
    // Use Date.now() + some randomness for unique session ID
    let timestamp = js_sys::Date::now();
    
    let random = js_sys::Math::random();
    format!("session_{}_{}", timestamp as u64, (random * 1000000.0) as u64)
}

/// Make HTTP request to the backend server
async fn execute_server_command(command: &str, args: &[String], session_id: &str) -> Result<CommandResponse, String> {
    let request_body = CommandRequest {
        session_id: session_id.to_string(),
        command: command.to_string(),
        args: args.to_vec(),
    };
    
    let body = serde_json::to_string(&request_body)
        .map_err(|e| format!("Failed to serialize request: {}", e))?;
    
    let mut opts = RequestInit::new();
    opts.set_method("POST");
    opts.set_mode(RequestMode::Cors);
    opts.set_body(&JsValue::from_str(&body));
    
    let url = "http://localhost:8081/execute";
    let request = Request::new_with_str_and_init(url, &opts)
        .map_err(|_| "Failed to create request".to_string())?;
    
    request.headers().set("Content-Type", "application/json")
        .map_err(|_| "Failed to set headers".to_string())?;
    
    let window = web_sys::window().ok_or("No window object")?;
    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|_| "Network request failed - is the server running at localhost:8081?".to_string())?;
    
    let resp: Response = resp_value.dyn_into()
        .map_err(|_| "Failed to cast response".to_string())?;
    
    if !resp.ok() {
        return Err(format!("Server returned error: {}", resp.status()));
    }
    
    let json = JsFuture::from(resp.json().map_err(|_| "Failed to get JSON")?).await
        .map_err(|_| "Failed to parse JSON response".to_string())?;
    
    let response: CommandResponse = serde_wasm_bindgen::from_value(json)
        .map_err(|e| format!("Failed to deserialize response: {}", e))?;
    
    Ok(response)
}

/// PWD Command - Print Working Directory
pub struct PwdCommand;

impl Command for PwdCommand {
    fn execute(&self, args: &[String]) -> CommandResult {
        if !args.is_empty() && (args[0] == "--help" || args[0] == "-h") {
            let help_text = r#"pwd - Print name of current/working directory

Usage:
  pwd                     Show current directory path
  pwd --help              Show this help message

Description:
  The pwd command displays the full pathname of the current directory.
  The path is shown relative to the project root directory.

Examples:
  pwd"#;
            return CommandResult::Success(help_text.to_string());
        }
        
        // PWD is handled synchronously by returning a placeholder
        // The actual async operation will be handled by the frontend
        CommandResult::Success("__FILESYSTEM_PWD__".to_string())
    }
    
    fn description(&self) -> &'static str {
        "Print name of current directory"
    }
    
    fn usage(&self) -> &'static str {
        "pwd"
    }
}

/// CD Command - Change Directory
pub struct CdCommand;

impl Command for CdCommand {
    fn execute(&self, args: &[String]) -> CommandResult {
        if !args.is_empty() && (args[0] == "--help" || args[0] == "-h") {
            let help_text = r#"cd - Change the current directory

Usage:
  cd [directory]          Change to directory
  cd                      Change to root directory  
  cd --help               Show this help message

Description:
  The cd command changes the current working directory to the specified
  directory. If no directory is specified, changes to the root directory.
  All paths are relative to the project root directory.

Examples:
  cd                      Change to root directory
  cd src                  Change to src directory
  cd src/components       Change to src/components directory
  cd ..                   Go up one directory level"#;
            return CommandResult::Success(help_text.to_string());
        }
        
        // Package arguments for async processing
        let args_json = serde_json::to_string(args).unwrap_or_default();
        CommandResult::Success(format!("__FILESYSTEM_CD__:{}", args_json))
    }
    
    fn description(&self) -> &'static str {
        "Change the current directory"
    }
    
    fn usage(&self) -> &'static str {
        "cd [directory]"
    }
}

/// LS Command - List directory contents
pub struct LsCommand;

impl Command for LsCommand {
    fn execute(&self, args: &[String]) -> CommandResult {
        if !args.is_empty() && (args[0] == "--help" || args[0] == "-h") {
            let help_text = r#"ls - List directory contents

Usage:
  ls [directory]          List contents of directory
  ls                      List contents of current directory
  ls --help               Show this help message

Description:
  The ls command lists the contents of the specified directory.
  If no directory is specified, lists the current directory.
  All paths are relative to the current working directory.

Examples:
  ls                      List current directory contents
  ls src                  List contents of src directory
  ls src/components       List contents of src/components directory"#;
            return CommandResult::Success(help_text.to_string());
        }
        
        // Package arguments for async processing
        let args_json = serde_json::to_string(args).unwrap_or_default();
        CommandResult::Success(format!("__FILESYSTEM_LS__:{}", args_json))
    }
    
    fn description(&self) -> &'static str {
        "List directory contents"
    }
    
    fn usage(&self) -> &'static str {
        "ls [directory]"
    }
}

/// VIEW Command - Render markdown files
pub struct ViewCommand;

impl Command for ViewCommand {
    fn execute(&self, args: &[String]) -> CommandResult {
        if !args.is_empty() && (args[0] == "--help" || args[0] == "-h") {
            let help_text = r#"view - Render markdown files

Usage:
  view <file.md>          Render and display markdown file
  view --help             Show this help message

Description:
  The view command renders markdown files and displays them with formatting.
  Only markdown files (.md, .markdown) are supported.
  The output includes colored text, headers, lists, and other markdown elements.

Examples:
  view README.md          Render and display README.md
  view posts/article.md   Render and display posts/article.md"#;
            return CommandResult::Success(help_text.to_string());
        }
        
        if args.is_empty() {
            return CommandResult::Error("view: missing filename".to_string());
        }
        
        // Package arguments for async processing
        let args_json = serde_json::to_string(args).unwrap_or_default();
        CommandResult::Success(format!("__FILESYSTEM_VIEW__:{}", args_json))
    }
    
    fn description(&self) -> &'static str {
        "Render markdown files"
    }
    
    fn usage(&self) -> &'static str {
        "view <file.md>"
    }
}

/// CAT Command - Display file contents
pub struct CatCommand;

impl Command for CatCommand {
    fn execute(&self, args: &[String]) -> CommandResult {
        if !args.is_empty() && (args[0] == "--help" || args[0] == "-h") {
            let help_text = r#"cat - Display file contents

Usage:
  cat <file>              Display contents of file
  cat --help              Show this help message

Description:
  The cat command displays the contents of the specified file.
  The file path is relative to the current directory.

Examples:
  cat README.md           Display contents of README.md
  cat src/main.rs         Display contents of src/main.rs"#;
            return CommandResult::Success(help_text.to_string());
        }
        
        if args.is_empty() {
            return CommandResult::Error("cat: missing filename".to_string());
        }
        
        // Package arguments for async processing
        let args_json = serde_json::to_string(args).unwrap_or_default();
        CommandResult::Success(format!("__FILESYSTEM_CAT__:{}", args_json))
    }
    
    fn description(&self) -> &'static str {
        "Display file contents"
    }
    
    fn usage(&self) -> &'static str {
        "cat <file>"
    }
}

// Export async execution function for use by the terminal component
pub async fn execute_filesystem_command(command: &str, args: &[String], session_id: &str) -> String {
    match execute_server_command(command, args, session_id).await {
        Ok(response) => {
            if response.success {
                response.output
            } else {
                response.output
            }
        }
        Err(error) => error,
    }
}