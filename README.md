# Proyectos avanzados con Rust

## Table of Contents

- [Proyectos avanzados con Rust](#proyectos-avanzados-con-rust)
  - [Table of Contents](#table-of-contents)
  - [Acerca del proyecto](#acerca-del-proyecto)
  - [Como iniciarlo](#como-iniciarlo)
    - [Requisitos](#requisitos)
    - [Instalación](#instalación)

## Acerca del proyecto

Proyectos en rust para aprender conceptos avanzados.

Scenario 1

Este proyecto es el primero, se trata de un espacio de trabajo aka (workspace)
configurado en el archivo Cargo.toml el cual nos permitirá relacionar el scenario 1
con los 4 sub proyectos

1. Definición del workspace para _scenario1_

```toml
[workspace]
members = ["tcpserver", "tcpclient", "http", "httpserver"]
```

1. tcpserver y tcpclient son ejemplos en crudo de una comunicación básica entre cliente y servidor en Rust.

2. http es un librería que contiene estructuras de datos y métodos que realizan lo siguiente
   1. Interpretar un flujo de bytes y convertirlo en un mensaje de solicitud HTTP.
   2. Construir un mensaje de respuesta HTTP y convertirlo a un flujo de bytes para transmitirlos.
   3. El modulo httprequest en _http/src/lib.rs_ sirve de apoyo para la librería.
3. La librería http se divide en varias partes. (Estructuras/Enums)
   1. HttpRequest - struct - Representa una petición HTTP.
   2. Method - enum - Representa los métodos HTTP permitidos.
   3. Version - enum - Representa las versiones permitidas de HTTP.
4. La segunda parte son los trait (Métodos o funciones implementadas)
   1. From<&str> - Habilitar al conversion de un string slice a una estructura HttpRequest.
   2. Debug - Imprimir mensajes de depuración.
   3. PartialEq - Usado para comparar valores como parte del parseo y pruebas automáticas.
5. La librería Http se puede usar como si fuese un paquete, en el archivo toml de HttpServer se agrego como dependencia.
   1. http = {path = "../http"} // De esta forma, creamos una referencia a la librería Http en el proyecto HttpServer

## Como iniciarlo

Clonar, cargo run y listo.

### Requisitos

Cargo
Rust

```
cargo -V
rustc -V
```

### Instalación

```bash
git clone 'this repo.git'
cd 'this repo dir'
cargo run
```
