 Klippa 📎

Klippa es una herramienta simple para gestionar scripts y comandos largos. ¡Guarda tus tareas repetitivas y ejecútalas fácilmente!

## Funciones Destacadas 🌟

- **Creación Rápida:** Guarda nuevos comandos y scripts de forma sencilla.
- **Ejecución Instantánea:** Ejecuta tus comandos almacenados y guardalos en una lista.
- **Portapapeles Sticky:** Copia la salida al portapapeles para un acceso fácil.

## Instalación 🚀

```bash
cargo build --release
```

## Uso Básico 🛠️

### Crear un Nuevo Comando

```bash
./klippa create <nombre_comando> <comando_shell> (usa comillas dobles)
```

### Crear un Comando desde un Archivo de Script

```bash
./klippa script <nombre_comando> <ruta_script>
```

### Listar Comandos Guardados

```bash
./klippa list
```

### Ejecutar un Comando

```bash
./klippa execute <nombre_comando> [-s/--sticky]
```
### Uso como Comando de Sistema 🌐

1. Copia el ejecutable de Klippa a un directorio en tu `PATH`:

```bash
cp target/release/klippa /usr/local/bin/
```

2. Ahora puedes usar Klippa como un comando del sistema:

```bash
klippa create <nombre_comando> <comando_shell>
```
### Integración con Zellij 🪟

¡Usa Klippa en los layouts de Zellij para una experiencia de terminal más eficiente! Agrega lo siguiente a tu archivo de configuración de Zellij (`~/.config/zellij/zellij.toml`):

```toml
[[layouts]]
    parent = "even-horizontal"
    split = "even-vertical"
    [[layouts.panes]]
        command = "klippa list"
    [[layouts.panes]]
        command = "klippa execute <nombre_comando>"
```

## Contribuciones 🤝

¡Contribuciones y sugerencias son bienvenidas! Si encuentras algún problema o tienes ideas para mejorar Klippa, por favor abre un problema o envía una PR.
- tareas por hacer borrar de la lista
- integrar con zellij para usar layout y llamar a tus scripts de manera mas simple y organizada
  

## Licencia 📄

Este proyecto está bajo la Licencia MIT - consulta el archivo [LICENSE](LICENSE) para más detalles.

¡Gracias por usar Klippa! 🚀
