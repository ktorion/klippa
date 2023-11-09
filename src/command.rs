use std::io::Read;
use std::process::Command;
use clipboard::{ClipboardContext, ClipboardProvider};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Write};
// use std::path::Path;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use structopt::StructOpt;

#[derive(StructOpt, Debug, Deserialize, Serialize)]
#[structopt(name = "klippa", about = "Un programa para gestionar comandos")]
pub struct Opt {
    #[structopt(subcommand)]
    pub subcommand: Subcommand,
}

#[derive(StructOpt, Debug, Deserialize, Serialize)]
pub enum Subcommand {
    #[structopt(name = "create", about = "Crea un nuevo comando")]
    Create {
        #[structopt(name = "nombre_comando", help = "Nombre del nuevo comando")]
        command_name: String,

        #[structopt(name = "comando_shell", help = "Comando de la shell")]
        shell_command: String,
    },

    #[structopt(name = "script", about = "Crea un nuevo comando desde un archivo de script")]
    Script {
        #[structopt(name = "nombre_comando", help = "Nombre del nuevo comando")]
        command_name: String,

        #[structopt(name = "ruta_script", help = "Ruta al archivo de script")]
        script_path: String,
    },


    #[structopt(name = "list", about = "Muestra la lista de comandos creados")]
    List,

    #[structopt(name = "execute", about = "Ejecuta un comando")]
    Execute {
        #[structopt(name = "nombre_comando", help = "Nombre del comando a ejecutar")]
        command_name: String,

        #[structopt(short = "s", long = "sticky", help = "Copia la salida al portapapeles")]
        sticky: bool,
    },
}

pub fn get_command_history() -> HashMap<String, String> {
    match read_from_file() {
        Ok(content) => content,
        Err(_) => HashMap::new(),
    }
}

pub fn save_command_history(command_history: &HashMap<String, String>) {
    if let Err(err) = write_to_file(command_history) {
        eprintln!("Error al guardar el historial de comandos: {}", err);
    }
}

fn get_command_history_file_path() -> &'static str {
    "command_history.txt"
}
fn read_from_file() -> io::Result<HashMap<String, String>> {
    let file_path = get_command_history_file_path();
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(_) => return Ok(HashMap::new()),
    };

    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let command_history: HashMap<String, String> = match serde_json::from_str(&content) {
        Ok(history) => history,
        Err(_) => HashMap::new(),
    };

    Ok(command_history)
}

fn write_to_file(command_history: &HashMap<String, String>) -> io::Result<()> {
    let file_path = get_command_history_file_path();
    let file = OpenOptions::new().write(true).create(true).truncate(true).open(file_path)?;

    serde_json::to_writer(file, &command_history)?;

    Ok(())
}
pub fn create_command(args: &[String]) {
    // Verifica si se proporciona el nombre del comando y el script
    if args.len() < 5 {
        eprintln!("Usage: {} create <command_name> <shell_command>", args[0]);
        std::process::exit(1);
    }

    // Nombre del comando y comando de la shell
    let command_name = &args[2];
    let command_content: String;  // Declare una variable para almacenar el resultado

    // Almacena el resultado en la variable
    if args[3] == "--script" {
        command_content = args[4..].join(" ");
    } else {
        command_content = args[3..].join(" ");
    }

    // Guarda el comando en la lista
    let mut command_history = get_command_history();
    command_history.insert(command_name.clone(), command_content.clone());
    save_command_history(&command_history);
    println!("Comando '{}' creado exitosamente.", command_name);
}



pub fn list_commands() {
    // Muestra la lista de comandos guardados
    let command_history = get_command_history();
    println!("Comandos guardados:");
    for name in command_history.keys() {
        println!("{}", name);
    }
}

pub fn execute_command(args: &[String]) {
    // Verifica si el comando existe en la lista
    if let Some(command_content) = get_command_history().get(&args[1]) {
        // Ejecuta el comando y captura la salida
        let output = Command::new("sh")
            .arg("-c")
            .arg(command_content)
            .output()
            .expect("Error al ejecutar el comando");

        // Convierte la salida a una cadena UTF-8
        let command_output = String::from_utf8_lossy(&output.stdout).trim().to_string();

        // Copia la información al portapapeles si se especifica --sticky
        if args.len() > 2 && args[2] == "--sticky" {
            let mut ctx: ClipboardContext = ClipboardProvider::new().expect("Error al acceder al portapapeles");
            ctx.set_contents(command_output.clone()).expect("Error al establecer el contenido del portapapeles");
        }

        // Imprime la salida del comando
        println!("{}", command_output);
    } else {
        eprintln!("Comando '{}' no encontrado.", args[1]);
        std::process::exit(1);
    }
}
pub fn read_script_file(script_path: &str) -> Result<String, io::Error> {
    let mut file_content = String::new();
    let mut file = File::open(script_path)?;
    file.read_to_string(&mut file_content)?;
    Ok(file_content)
}

// Modifica el subcomando Script para utilizar las mejoras
pub fn create_script_command(command_name: &str, script_path: &str) {
    let script_content = match read_script_file(script_path) {
        Ok(content) => content,
        Err(err) => {
            println!("Error al leer el archivo de script: {}", err);
            return;
        }
    };

    // Resto de la lógica del programa aquí...
    let mut command_history = get_command_history();
    command_history.insert(command_name.to_string(), script_content.clone());
    save_command_history(&command_history);

    println!("Comando '{}' creado exitosamente desde el archivo '{}'.", command_name, script_path);

    println!("Contenido del script:\n{}", script_content);
}

