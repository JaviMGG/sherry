# 🍒 Sherry Security Launcher

**Sherry** es una herramienta escrita en Rust orientada a ciberseguridad que permite ejecutar comandos de reconocimiento y auditoría en terminales separadas de forma rápida y estructurada.

Está diseñada como un **launcher de herramientas de pentesting**, ideal para flujos de trabajo de:

- reconocimiento (recon)
- enumeración de red
- OSINT básico
- ejecución de herramientas externas (nmap, whois, ping, etc.)

---

## 🚀 Características

- 🧠 **CLI + modo interactivo**
  - Ejecución directa: `sherry scan 192.168.1.1`
  - Shell interactiva: `sherry`

- 🔐 **Enfoque en ciberseguridad**
  - Integración con herramientas como `nmap`, `whois`, `ping`
  - Playbook automático de reconocimiento (`recon`)

- 🪟 **Ejecución en terminales separadas**
  - Cada tarea se abre en una ventana independiente
  - Soporte para Linux, macOS y Windows

- 🧩 **Arquitectura extensible**
  - Fácil añadir nuevos comandos de seguridad
  - Sistema basado en `enum` y parser modular

- ⚠️ **Validación básica de inputs**
  - Prevención de caracteres peligrosos en parámetros

---

### 1. Requisitos previos
Asegúrate de tener instalado el compilador de Rust (`cargo`) y la terminal de GNOME:
```bash
sudo apt update
sudo apt install cargo gnome-terminal
```

1. Clona este repositorio:

   ```bash
   git clone https://github.com/JaviMGG/sherry.git
   cd sherry
   ```

2. Instala la herramienta usando Cargo:
   ```bash
   cargo install --path .
   ```

   Esto instalará el binario `sherry` en tu PATH, permitiéndote ejecutarlo desde cualquier lugar.

## Ayuda
Puedes consultar la documentación en cualquier momento:
    
    sherry --help
    

## Usos
Si ejecutas el comando sin argumentos, entrarás en la consola de Sherry:
    
    sherry

Una vez dentro, verás el prompt "Sherry >". Cualquier comando que escribas ahí se abrirá en una nueva ventana:
    

    Sherry > scan 192.168.1.1
    Sherry > whois google.com
    Sherry > ping 8.8.8.8
    Sherry > recon target.com

Modo directo:

    Sherry scan 192.168.1.1
    Sherry whois example.com
    Sherry ping 8.8.8.8
    Sherry recon target.com

Dentro del modo interactivo, puedes usar:

    help o --help: Muestra el menú de ayuda.

    salir, exit o q: Cierra Sherry Launcher.

## 🔍 ¿Cómo funciona internamente?

Sherry actúa como un orquestador de herramientas del sistema.

Usa std::process::Command para ejecutar herramientas externas. Detecta el sistema operativo automáticamente:
    Linux
    macOS
    Windows
    Abre cada ejecución en una terminal independiente

Ejemplo:

    nmap -sV target; echo "--- FIN ---"; read

Esto asegura que, incluso si el proceso hijo falla o termina, la terminal secundaria se mantenga a la espera de una entrada del usuario antes de desaparecer.

## ⚠️ Dependencias importantes

Sherry NO incluye herramientas internas.

Debe tener instaladas en el sistema:

    nmap
    whois
    ping (incluido en la mayoría de sistemas)

Si falta alguna, el comando fallará.

## 🧠 Filosofía del proyecto

Sherry no reemplaza herramientas como Nmap o Metasploit.

Es un: Launcher ligero para automatizar y organizar flujos de ciberseguridad

## 🤝 Contribuciones
Se aceptan mejoras:

    Nuevos módulos OSINT
    Soporte para más herramientas
    Sistema de plugins
    Interfaz TUI

## ⚖️ Licencia
Este proyecto está bajo la licencia MIT. ¡Úsalo y modifícalo como quieras!