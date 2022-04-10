use crate::matriz::MatrizLCS;

/// Dada una matriz LCS ya calculada, las lineas de dos archivos y los tamaños de estos dos archivos, imprime
/// por pantalla la diferencia entre los dos archivos dados utilizando la matriz del metodo LCS.
/// Ejemplo:
/// ```rust
/// diff::print_diff(&unaMatrizLCS, &lineasArchivo1, &lineasArchivo2, tamanioArchivo1, tamanioArchivo2);
/// ```
pub fn print_diff(matriz: &MatrizLCS, x: &[String], y: &[String], i: usize, j: usize) {
    if i > 0 && j > 0 && x[i - 1] == y[j - 1] {
        print_diff(matriz, x, y, i - 1, j - 1);
        println!("  {}", x[i - 1]);
    } else if j > 0
        && (i == 0 || matriz.elemento_en_pos(i, j - 1) >= matriz.elemento_en_pos(i - 1, j))
    {
        print_diff(matriz, x, y, i, j - 1);
        println!("> {}", y[j - 1]);
    } else if i > 0
        && (j == 0 || matriz.elemento_en_pos(i, j - 1) < matriz.elemento_en_pos(i - 1, j))
    {
        print_diff(matriz, x, y, i - 1, j);
        println!("< {}", x[i - 1]);
    } else {
        println!();
    }
}
