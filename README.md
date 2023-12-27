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
