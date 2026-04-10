# 🍒 Sherry Launcher

**Sherry Launcher** es una utilidad ligera escrita en Rust diseñada para ejecutar comandos de terminal en ventanas independientes de forma rápida. 

Es ideal para desarrolladores que necesitan lanzar procesos (servidores, monitores de sistema, scripts) y mantener la terminal principal libre mientras ven la salida de los otros procesos.

---

## 🚀 Características

- **Modo Dual**: Funciona como un comando directo y como una shell interactiva.
- **Auto-pausa**: Al terminar un comando, la ventana no se cierra automáticamente; espera a que presiones `Enter` para que puedas leer la salida o errores.
- **Compatibilidad**: Diseñado específicamente para sistemas que utilizan `gnome-terminal` (Ubuntu, Debian, Fedora, Pop!_OS, etc.).
- **Escrito en Rust**: Rápido, seguro y con un consumo de recursos mínimo.

---

## 🛠️ Instalación

### 1. Requisitos previos
Asegúrate de tener instalado el compilador de Rust (`cargo`) y la terminal de GNOME:
```bash
sudo apt update
sudo apt install cargo gnome-terminal
```

1. Clona este repositorio:

   ```bash
   git clone https://github.com/tu-usuario/orden.git
   cd orden
   ```

2. Instala la herramienta usando Cargo:
   ```bash
   cargo install --path .
   ```

   Esto instalará el binario `orden` en tu PATH, permitiéndote ejecutarlo desde cualquier lugar.

## Ayuda
Puedes consultar la documentación en cualquier momento:
    
    sherry --help
    

## Usos
Si ejecutas el comando sin argumentos, entrarás en la consola de Sherry:
    
    
    sherry

Una vez dentro, verás el prompt "Sherry >". Cualquier comando que escribas ahí se abrirá en una nueva ventana:
    

    Sherry > htop
    Sherry > python3 script.py
    Sherry > ls -la

Dentro del modo interactivo, puedes usar:

    help o --help: Muestra el menú de ayuda.

    salir, exit o q: Cierra Sherry Launcher.

## 🔍 ¿Cómo funciona internamente?

El programa utiliza el módulo std::process::Command para invocar gnome-terminal. 

La magia para mantener la ventana abierta reside en cómo se encapsula el comando:

    bash -c "TU_COMANDO; 
    echo 'Presiona Enter para cerrar'; read"

Esto asegura que, incluso si el proceso hijo falla o termina, la terminal secundaria se mantenga a la espera de una entrada del usuario antes de desaparecer.

## 🤝 Contribuciones
Si deseas añadir soporte para otras terminales (como xterm, konsole o alacritty), siéntete libre de abrir un Pull Request o crear un Issue.

## ⚖️ Licencia
Este proyecto está bajo la licencia MIT. ¡Úsalo y modifícalo como quieras!