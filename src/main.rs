use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Error};
mod diff;
mod matriz;
use crate::matriz::MatrizLCS;

/// Abre el archivo del path recibido por parametro y si falla vuelve a pedirlo.
/// Ejemplo:
/// ```rust
/// abrir_archivo(nombreArchivo);
/// ```
fn abrir_archivo(path: String) -> File {
    match File::open(path) {
        Ok(it) => it,
        Err(_) => {
            println!("Nombre de archivo incorrecto. Por favor vuelva a ingresarlo:");
            abrir_archivo(leer_por_pantalla())
        }
    }
}

/// Lee las lineas del archivo recibido por parametro, las guarda en un vector y lo devuelve.
/// Ejemplo:
/// ```rust
/// read_file_lines(nombreArchivo);
/// ```
fn read_file_lines(path: String) -> Result<Vec<String>, Error> {
    let mut vector = Vec::new();
    let reader = BufReader::new(abrir_archivo(path));
    for line in reader.lines() {
        vector.push(line?);
    }

    Ok(vector)
}

/// Lee por pantalla, guarda lo leido en una variable y la devuelve.
/// Ejemplo:
/// ```rust
/// leer_por_pantall();
/// ```
fn leer_por_pantalla() -> String {
    let mut archivo_ingresado = String::new();

    io::stdin()
        .read_line(&mut archivo_ingresado)
        .expect("Failed to read line");

    archivo_ingresado.trim().to_string()
}

fn main() {
    println!("Bienvenido!");
    println!("Ingrese el nombre del primer archivo a comparar: ");

    let lineas_archivo1 = read_file_lines(leer_por_pantalla());
    let lectura_correcta1 = lineas_archivo1.expect("Error en lectura de archivo");

    println!("Ingrese el nombre del segundo archivo a comparar: ");

    let lineas_archivo2 = read_file_lines(leer_por_pantalla());
    let lectura_correcta2 = lineas_archivo2.expect("Error en lectura de archivo");

    let mut matriz = MatrizLCS::new(lectura_correcta1.len(), lectura_correcta2.len());
    matriz.lcs(&lectura_correcta1, &lectura_correcta2);

    diff::print_diff(
        &matriz,
        &lectura_correcta1,
        &lectura_correcta2,
        lectura_correcta1.len(),
        lectura_correcta2.len(),
    );
}
