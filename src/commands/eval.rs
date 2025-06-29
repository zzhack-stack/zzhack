// EVAL Command Implementation
// Execute JavaScript code from files or inline strings

use super::{Command, CommandResult, TerminalContext};
use crate::filesystem::FileSystem;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

pub struct EvalCommand {
    pub filesystem: Rc<RefCell<FileSystem>>,
}

impl Command for EvalCommand {
    fn execute(&self, args: &[String], _context: &TerminalContext) -> CommandResult {
        if args.is_empty() {
            return CommandResult::Error(
                "eval: missing arguments. Use 'eval file.js' or 'eval -e \"code\"'".to_string(),
            );
        }

        if args[0] == "-e" {
            // Execute inline JavaScript code
            if args.len() < 2 {
                return CommandResult::Error(
                    "eval: missing JavaScript code after -e flag".to_string(),
                );
            }

            let js_code = &args[1];
            Self::execute_javascript(js_code)
        } else {
            // Execute JavaScript file
            let filename = &args[0];
            let fs = self.filesystem.borrow();

            match fs.get_file_info(filename) {
                Ok(node) => {
                    // Check if it's a JavaScript file
                    let is_js_file = node
                        .extension
                        .as_ref()
                        .map(|ext| ext == "js" || ext == "mjs")
                        .unwrap_or(false);

                    if !is_js_file {
                        CommandResult::Error(format!("eval: not a JavaScript file: {}", filename))
                    } else {
                        let file_path = node.path.clone();

                        // Return async future to fetch and execute the JavaScript file
                        let future = Box::pin(async move {
                            match crate::utils::fetch_file_content(&file_path).await {
                                Ok(js_content) => Self::execute_javascript(&js_content),
                                Err(error) => CommandResult::Error(format!(
                                    "eval: Error reading file: {}",
                                    error
                                )),
                            }
                        });

                        CommandResult::Async(future)
                    }
                }
                Err(error) => CommandResult::Error(format!("eval: {}", error)),
            }
        }
    }

    fn description(&self) -> &'static str {
        "Execute JavaScript code"
    }

    fn usage(&self) -> &'static str {
        "eval <file.js> | eval -e \"code\""
    }

    fn help(&self) -> Option<&'static str> {
        Some(
            r#"eval - Execute JavaScript code

Usage:
  eval <file.js>              Execute JavaScript from file
  eval -e "javascript code"   Execute inline JavaScript code
  eval --help                 Show this help message

Description:
  The eval command executes JavaScript code either from a file or inline.
  It captures console.log output and returns the result or any errors.
  Only JavaScript files (.js, .mjs) are supported for file execution.

Examples:
  eval script.js              Execute script.js file
  eval -e "2 + 2"             Execute inline expression
  eval -e "console.log('Hi')" Execute inline with console output"#,
        )
    }
}

impl EvalCommand {
    fn execute_javascript(js_code: &str) -> CommandResult {
        // Get the global window object
        let _window = match web_sys::window() {
            Some(w) => w,
            None => return CommandResult::Error("eval: No window object available".to_string()),
        };

        // Create a capture mechanism for console.log
        let logs = Rc::new(RefCell::new(Vec::<String>::new()));
        let logs_clone = logs.clone();

        // Override console.log to capture output
        let console_log_override = Closure::wrap(Box::new(move |msg: JsValue| {
            let msg_str = match msg.as_string() {
                Some(s) => s,
                None => format!("{:?}", msg),
            };
            logs_clone.borrow_mut().push(msg_str);
        }) as Box<dyn FnMut(JsValue)>);

        // Get the console object through the global object
        let global = js_sys::global();
        let console_obj =
            js_sys::Reflect::get(&global, &JsValue::from_str("console")).unwrap_or(JsValue::NULL);
        let original_log =
            js_sys::Reflect::get(&console_obj, &JsValue::from_str("log")).unwrap_or(JsValue::NULL);

        // Set our override
        js_sys::Reflect::set(
            &console_obj,
            &JsValue::from_str("log"),
            console_log_override.as_ref(),
        )
        .ok();

        // Execute the JavaScript code using js_sys::eval
        let result = match js_sys::eval(js_code) {
            Ok(result) => {
                // Restore original console.log
                js_sys::Reflect::set(&console_obj, &JsValue::from_str("log"), &original_log).ok();

                let captured_logs = logs.borrow().clone();
                let mut output_parts = Vec::new();

                // Add console.log output if any
                if !captured_logs.is_empty() {
                    output_parts.extend(captured_logs);
                }

                // Add the result if it's not undefined
                if !result.is_undefined() {
                    let result_str = if result.is_null() {
                        "null".to_string()
                    } else if let Some(s) = result.as_string() {
                        s
                    } else if let Some(b) = result.as_bool() {
                        b.to_string()
                    } else if let Some(n) = result.as_f64() {
                        n.to_string()
                    } else {
                        // Try to stringify the object
                        match js_sys::JSON::stringify(&result) {
                            Ok(json_str) => json_str
                                .as_string()
                                .unwrap_or_else(|| format!("{:?}", result)),
                            Err(_) => format!("{:?}", result),
                        }
                    };

                    if !result_str.is_empty() && result_str != "undefined" {
                        output_parts.push(result_str);
                    }
                }

                if output_parts.is_empty() {
                    CommandResult::Success(String::new())
                } else {
                    CommandResult::Success(output_parts.join("\n"))
                }
            }
            Err(error) => {
                // Restore original console.log
                js_sys::Reflect::set(&console_obj, &JsValue::from_str("log"), &original_log).ok();

                let error_msg = if let Some(error_str) = error.as_string() {
                    error_str
                } else {
                    "JavaScript execution error".to_string()
                };
                CommandResult::Error(format!("eval: {}", error_msg))
            }
        };

        // Clean up the closure
        console_log_override.forget();

        result
    }
}
