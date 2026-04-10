use std::io::{self, Write};
use std::process::Command;
use std::env; // Importante para leer argumentos externos

fn mostrar_ayuda() {
    println!("\n📖 AYUDA DE SHERRY LAUNCHER");
    println!("---------------------------");
    println!("Uso: sherry [comando]");
    println!("Si se ejecuta sin comandos, entra en modo interactivo.");
    println!("\nOpciones:");
    println!("  -h, --help    Muestra esta ayuda.");
    println!("  salir, q      Cierra el programa (en modo interactivo).\n");
}

fn main() {
    // 1. CAPTURAR ARGUMENTOS EXTERNOS
    // args[0] es el nombre del programa, args[1] es el primer argumento
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let primer_arg = &args[1];
        
        if primer_arg == "--help" || primer_arg == "-h" {
            mostrar_ayuda();
            return; // Cerramos el programa después de mostrar la ayuda
        }
        
        // OPCIONAL: Si quieres que 'sherry ls' funcione directamente 
        // sin entrar al menú, podrías llamar a lanzar_en_nueva_terminal aquí.
        lanzar_en_nueva_terminal(primer_arg);
    }

    // 2. MODO INTERACTIVO (tu código original)
    println!("--- SHERRY LAUNCHER ---");
    println!("Escribe 'help' para ver instrucciones o 'salir' para cerrar.");

    loop {
        print!("Sherry > ");
        io::stdout().flush().unwrap();

        let mut entrada = String::new();
        io::stdin().read_line(&mut entrada).expect("Error al leer");
        let entrada = entrada.trim();

        if entrada.is_empty() { continue; }

        // Ayuda dentro del modo interactivo
        if entrada == "--help" || entrada == "-h" || entrada == "help" {
            mostrar_ayuda();
            continue;
        }

        if entrada == "salir" || entrada == "exit" || entrada == "q" {
            break;
        }

        lanzar_en_nueva_terminal(entrada);
    }
}

fn lanzar_en_nueva_terminal(comando: &str) {
    let instruccion_final = format!("{}; echo -e '\n--- Proceso terminado. Presiona Enter para cerrar ---'; read", comando);

    let proceso = Command::new("gnome-terminal")
        .args(["--", "bash", "-c", &instruccion_final])
        .spawn();

    match proceso {
        Ok(_) => println!("✅ Lanzado: '{}'.", comando),
        Err(e) => eprintln!("❌ Error al intentar abrir la terminal: {}", e),
    }
}