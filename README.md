# Tutorial y Ejercicios de Sintaxis en Rust

Este repositorio contiene explicaciones y ejercicios prácticos para aprender la sintaxis básica de Rust. Cada archivo `.rs` incluye ejemplos explicativos y ejercicios diseñados para fortalecer tus habilidades en Rust. También encontrarás scripts individuales para practicar temas como manejo de errores, referencias, lifetimes, traits y más.

---

## **Estructura del Repositorio**

- Cada archivo `.rs` aborda un tema específico de la sintaxis de Rust.
- Los archivos están numerados para guiar el orden de aprendizaje. Por ejemplo:
  - `00_hello_world.rs`: Introducción a Rust.
  - `01_variables.rs`: Variables y mutabilidad.
  - `02_control_flow.rs`: Flujo de control.
  - `03_structs_and_traits.rs`: Structs y Traits.
  - `...`

Cada script contiene:

1. Explicaciones detalladas del tema.
2. Ejercicios prácticos para aplicar los conceptos.
3. Soluciones en archivos separados para referencia.

---

## **Requisitos Previos**

1. **Sistema Operativo**: Compatible con Linux, macOS y Windows.
2. **Herramientas**:
   - [Rust y Cargo](https://www.rust-lang.org/tools/install).
   - Cargo Script para ejecutar scripts individuales directamente.

---

## **Instalación**

Sigue estos pasos para instalar Rust y preparar tu entorno de desarrollo:

1. **Instalar Rust y Cargo**:

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env

   ```

2. **Crear un proyecto con Cargo**:

   ```bash
    cargo new nombre_del_proyecto
    cd nombre_del_proyecto
    cargo run
   ```

3. **Instalar Cargo Script (para ejecutar scripts individuales):**

   ```bash
   cargo install cargo-script

   ```

4. **Corre un script**
   ```bash
   cd src
   cargo script 03_statements_and_expressions.rs
   ```
