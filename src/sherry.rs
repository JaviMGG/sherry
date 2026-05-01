use std::env;
use std::io::{self, Write};
use std::process::Command;
use colored::*;

// ==========================
// BANNER
// ==========================
fn mostrar_banner() {
    let banner = r#"
   ____  _                          
  / ___|| |__   ___ _ __ _ __ _   _ 
  \___ \| '_ \ / _ \ '__| '__| | | |
   ___) | | | |  __/ |  | |  | |_| |
  |____/|_| |_|\___|_|  |_|   \__, |
                               |___/ 
    "#;

    println!("{}", banner.red().bold());
}

// ==========================
// AYUDA
// ==========================
fn mostrar_ayuda() {
    println!("\n📖 SHERRY SECURITY LAUNCHER");
    println!("--------------------------------");
    println!("scan <ip>      -> escaneo de puertos (nmap)");
    println!("whois <dom>    -> info dominio");
    println!("ping <host>    -> ping básico");
    println!("recon <target> -> playbook completo");

    println!("\nComandos:");
    println!("help, -h        -> ayuda");
    println!("salir           -> salir\n");
}

// ==========================
// ENUM
// ==========================
enum AccionSeguridad {
    EscaneoPuertos(String),
    Whois(String),
    Ping(String),
    Recon(String),
    //nuevo comando aqui
}

// ==========================
// PARSER
// ==========================
fn parsear_comando(input: &str) -> Option<AccionSeguridad> {
    let partes: Vec<&str> = input.split_whitespace().collect();

    match partes.as_slice() {
        ["scan", objetivo] => Some(AccionSeguridad::EscaneoPuertos(objetivo.to_string())),
        ["whois", dominio] => Some(AccionSeguridad::Whois(dominio.to_string())),
        ["ping", host] => Some(AccionSeguridad::Ping(host.to_string())),
        ["recon", objetivo] => Some(AccionSeguridad::Recon(objetivo.to_string())),
        //nuevo comando aqui
        _ => None,
    }
}

// ==========================
// VALIDACIÓN
// ==========================
fn validar_input(input: &str) -> bool {
    input.chars().all(|c| c.is_alphanumeric() || ".:-".contains(c))
}

// ==========================
// DETECTAR SI COMANDO EXISTE
// ==========================
fn comando_existe(cmd: &str) -> bool {
    Command::new("which")
        .arg(cmd)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

// ==========================
// CONSTRUCCIÓN
// ==========================
fn construir_comando(accion: &AccionSeguridad) -> Option<String> {
    match accion {
        AccionSeguridad::EscaneoPuertos(obj) => {
            if !validar_input(obj) || !comando_existe("nmap") {
                eprintln!("❌ nmap no está instalado");
                return None;
            }
            Some(format!("nmap -sV {}", obj))
        }

        AccionSeguridad::Whois(dom) => {
            if !validar_input(dom) || !comando_existe("whois") {
                eprintln!("❌ whois no está instalado");
                return None;
            }
            Some(format!("whois {}", dom))
        }

        AccionSeguridad::Ping(host) => {
            if !validar_input(host) {
                return None;
            }
            Some(format!("ping -c 4 {}", host))
        }

        //nuevo comando aqui

        AccionSeguridad::Recon(_) => None,
    }
}

// ==========================
// DETECTAR OS
// ==========================
fn detectar_sistema() -> &'static str {
    if cfg!(target_os = "linux") {
        "linux"
    } else if cfg!(target_os = "windows") {
        "windows"
    } else if cfg!(target_os = "macos") {
        "macos"
    } else {
        "unknown"
    }
}

// ==========================
// LANZADOR GENERAL
// ==========================
fn lanzar_en_nueva_terminal(comando: &str) {
    let sistema = detectar_sistema();

    let resultado = match sistema {
        "linux" => lanzar_linux(comando),
        "macos" => lanzar_macos(comando),
        "windows" => lanzar_windows(comando),
        _ => {
            eprintln!("❌ Sistema no soportado");
            return;
        }
    };

    if let Err(e) = resultado {
        eprintln!("❌ Error: {}", e);
    } else {
        println!("✅ Ejecutado: {}", comando);
    }
}

// ==========================
// LINUX
// ==========================
fn lanzar_linux(comando: &str) -> std::io::Result<()> {
    let instruccion = format!(
        "{}; echo -e '\\n--- FIN, pulsa Enter para salir ---'; read",
        comando
    );

    let terminales = vec![
        ("gnome-terminal", vec!["--", "bash", "-c", &instruccion]),
        ("konsole", vec!["-e", &instruccion]),
        ("xfce4-terminal", vec!["-e", &instruccion]),
        ("xterm", vec!["-e", &instruccion]),
    ];

    for (term, args) in terminales {
        if Command::new("which").arg(term).output()?.status.success() {
            return Command::new(term).args(args).spawn().map(|_| ());
        }
    }

    Err(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "No terminal disponible",
    ))
}

// ==========================
// macOS
// ==========================
fn lanzar_macos(comando: &str) -> std::io::Result<()> {
    let safe = comando.replace("\"", "\\\"");

    Command::new("osascript")
        .arg("-e")
        .arg(format!(
            r#"tell application "Terminal" to do script "{}""#,
            safe
        ))
        .spawn()
        .map(|_| ())
}

// ==========================
// WINDOWS
// ==========================
fn lanzar_windows(comando: &str) -> std::io::Result<()> {
    Command::new("cmd")
        .args(&["/C", "start", "cmd", "/K", comando])
        .spawn()
        .map(|_| ())
}

// ==========================
// RECON PLAYBOOK
// ==========================
fn playbook_recon(obj: &str) {
    if !validar_input(obj) {
        eprintln!("❌ objetivo inválido");
        return;
    }

    let comandos = vec![
        format!("nmap -sV {}", obj),
        format!("whois {}", obj),
        format!("ping -c 4 {}", obj),
    ];

    for cmd in comandos {
        lanzar_en_nueva_terminal(&cmd);
    }
}

// ==========================
// MAIN
// ==========================
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let input = args[1..].join(" ");

        if input == "-h" || input == "--help" {
            mostrar_ayuda();
            return;
        }

        match parsear_comando(&input) {
            Some(AccionSeguridad::Recon(obj)) => playbook_recon(&obj),

            Some(accion) => {
                if let Some(cmd) = construir_comando(&accion) {
                    lanzar_en_nueva_terminal(&cmd);
                }
            }

            None => eprintln!("❌ comando no reconocido"),
        }

        return;
    }

    // ==========================
    // MODO INTERACTIVO
    // ==========================
    mostrar_banner();
    println!("Autor: JaviMGG");

    loop {
        print!("Sherry > ");
        io::stdout().flush().unwrap();

        let mut entrada = String::new();
        io::stdin().read_line(&mut entrada).unwrap();
        let entrada = entrada.trim();

        if entrada == "salir" || entrada == "exit" || entrada == "q" {
            break;
        }

        if entrada == "help" || entrada == "-h" {
            mostrar_ayuda();
            continue;
        }

        match parsear_comando(entrada) {
            Some(AccionSeguridad::Recon(obj)) => playbook_recon(&obj),

            Some(accion) => {
                if let Some(cmd) = construir_comando(&accion) {
                    lanzar_en_nueva_terminal(&cmd);
                }
            }

            None => eprintln!("❌ comando no reconocido"),
        }
    }
}