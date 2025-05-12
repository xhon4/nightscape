# Nightscape

[![Crates.io](https://img.shields.io/crates/v/nightscape.svg)](https://crates.io/crates/nightscape)

Animación de un cielo nocturno en la terminal, con luna, estrellas y eventos aleatorios.

## Instalación

### Opción rápida (recomendado)

```sh
cargo install nightscape
```

### Otras opciones

### Opción 1: Usando Cargo (recomendado, multiplataforma)

```sh
cargo install nightscape
```
> Si no está publicado aún, puedes instalarlo directamente desde GitHub:
```sh
cargo install --git https://github.com/tu_usuario/nightscape
```

### Opción 2: Descargar binarios precompilados

Ve a la sección [Releases](https://github.com/tu_usuario/nightscape/releases) y descarga el binario para tu sistema operativo.

### Opción 3: Compilar manualmente

#### 1. Instala Rust

- **Linux/macOS:**  
  ```sh
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- **Windows:**  
  Descarga e instala desde [rustup.rs](https://rustup.rs/).

#### 2. Clona este repositorio

```sh
git clone https://github.com/tu_usuario/nightscape.git
cd nightscape
```

#### 3. Compila y ejecuta

```sh
cargo run
```

O para compilar el binario:

```sh
cargo build --release
./target/release/nightscape
```

## Compatibilidad

Funciona en Linux, macOS y Windows (en terminales compatibles con Unicode y ANSI).

## Licencia

MIT
