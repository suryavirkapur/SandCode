use actix_web::HttpResponse;
use actix_web::web;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::io::Write;
use std::process::Command;

fn debug_docker_environment() -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("docker")
        .args(&[
            "run",
            "--rm",
            "executor",
            "sh",
            "-c",
            "echo $PATH && which bun && bun --version",
        ])
        .output()?;

    if !output.status.success() {
        return Err(format!(
            "Docker command failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

#[derive(Deserialize, Clone)]
pub struct CodeInput {
    language: String,
    code: String,
    input: Option<String>,
}

#[derive(Serialize, Clone)]
struct CodeOutput {
    output: String,
    error: String,
}

pub async fn execute_code(input: web::Json<CodeInput>) -> HttpResponse {
    let language = input.language.to_lowercase();
    let code = input.code.clone();
    let stdin = input.input.clone().unwrap_or_default();

    let current_dir = env::current_dir()
        .expect("Failed to get current directory")
        .to_string_lossy()
        .to_string();

    let (file_name, compile_cmd, run_cmd) = match language.as_str() {
        "python" => ("code.py", None, Some(vec!["python3", "code.py"])),
        "rust" => (
            "code.rs",
            Some(vec!["rustc", "code.rs"]),
            Some(vec!["./code"]),
        ),
        "cpp" => (
            "code.cpp",
            Some(vec!["g++", "code.cpp", "-o", "code"]),
            Some(vec!["./code"]),
        ),
        "typescript" | "javascript" => {
            // Debug Docker environment
            match debug_docker_environment() {
                Ok(debug_output) => println!("Docker environment debug:\n{}", debug_output),
                Err(e) => println!("Failed to debug Docker environment: {}", e),
            }
            ("code.ts", None, Some(vec!["bun", "run", "code.ts"]))
        }
        _ => return HttpResponse::BadRequest().json("Unsupported language"),
    };

    // Write code to file
    if let Err(e) = fs::write(file_name, code) {
        return HttpResponse::InternalServerError().json(CodeOutput {
            output: "".to_string(),
            error: format!("Failed to write code file: {}", e),
        });
    }

    let volume_mount = format!("{}:/app", current_dir);

    // Compile if necessary
    if let Some(cmd) = compile_cmd {
        let output = Command::new("docker")
            .args(&["run", "--rm", "-v", &volume_mount, "-w", "/app", "executor"])
            .args(cmd)
            .output();

        match output {
            Ok(output) if !output.status.success() => {
                return HttpResponse::Ok().json(CodeOutput {
                    output: "".to_string(),
                    error: String::from_utf8_lossy(&output.stderr).to_string(),
                });
            }
            Err(e) => {
                return HttpResponse::InternalServerError().json(CodeOutput {
                    output: "".to_string(),
                    error: format!("Compilation failed: {}", e),
                });
            }
            _ => {}
        }
    }

    // Run the code
    if let Some(cmd) = run_cmd {
        let mut docker_cmd = vec![
            "run",
            "--rm",
            "-i", // Added -i flag here
            "-v",
            &volume_mount,
            "-w",
            "/app",
            "executor",
        ];
        docker_cmd.extend(cmd);

        let mut child = Command::new("docker")
            .args(&docker_cmd)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .expect("Failed to spawn child process");

        if let Some(mut stdin_writer) = child.stdin.take() {
            let stdin_clone = stdin.clone();
            std::thread::spawn(move || {
                stdin_writer
                    .write_all(stdin_clone.as_bytes())
                    .expect("Failed to write to stdin");
            });
        }

        let output = child.wait_with_output().expect("Failed to wait on child");

        HttpResponse::Ok().json(CodeOutput {
            output: String::from_utf8_lossy(&output.stdout).to_string(),
            error: String::from_utf8_lossy(&output.stderr).to_string(),
        })
    } else {
        HttpResponse::InternalServerError().json(CodeOutput {
            output: "".to_string(),
            error: "Failed to run code".to_string(),
        })
    }
}
