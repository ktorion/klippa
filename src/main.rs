// main.rs

mod command;
use command::{get_command_history, read_script_file, save_command_history, Opt, Subcommand};
//use serde::Deserialize;  // Agrega esta línea
use structopt::StructOpt;
use clipboard::{ClipboardContext, ClipboardProvider};

fn main() {
    let opt: Opt = Opt::from_args();

    match opt.subcommand {
        Subcommand::Create { command_name, shell_command } => {
            let mut command_history = get_command_history();
            command_history.insert(command_name.clone(), shell_command);
            save_command_history(&command_history);
            println!("Comando '{}' creado exitosamente.", command_name);
        }
        Subcommand::Script { command_name, script_path } => {
            let script_content = match read_script_file(&script_path) {
                Ok(content) => content,
                Err(err) => {
                    println!("Error al leer el archivo de script: {}", err);
                    return;
                }
            };

            let mut command_history = get_command_history();
            command_history.insert(command_name.clone(), script_content);
            save_command_history(&command_history);
            println!("Comando '{}' creado exitosamente desde el archivo '{}'.", command_name, script_path);
        }
        Subcommand::List => {
            let command_history = get_command_history();
            println!("Comandos guardados:");
            for name in command_history.keys() {
                println!("{}", name);
            }
        }
        Subcommand::Execute { command_name, sticky } => {
            let command_history = get_command_history();
            if let Some(command_content) = command_history.get(&command_name) {
                let output = std::process::Command::new("sh")
                    .arg("-c")
                    .arg(command_content)
                    .output()
                    .expect("Error al ejecutar el comando");

                let command_output = String::from_utf8_lossy(&output.stdout).trim().to_string();

                if sticky {
                    let mut ctx: ClipboardContext = clipboard::ClipboardProvider::new().expect("Error al acceder al portapapeles");
                    ctx.set_contents(command_output.clone()).expect("Error al establecer el contenido del portapapeles");
                }

                println!("{}", command_output);
            } else {
                eprintln!("Comando '{}' no encontrado.", command_name);
                std::process::exit(1);
            }
        }
    }
}
