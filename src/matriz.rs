use std::cmp;
pub struct MatrizLCS {
    m: usize, //columnas
    n: usize, //filas
    elementos: Vec<u32>,
}

impl MatrizLCS {
    /// Constructor de la matriz LCS.
    /// Ejemplo:
    /// ```rust
    /// let matriz = MatrizLCS::new(cantLineasArchivo1, cantLineasArchivo2);
    /// ```
    pub fn new(fil: usize, col: usize) -> Self {
        MatrizLCS {
            elementos: vec![0; (col + 1) * (fil + 1)],
            m: col,
            n: fil,
        }
    }

    /// Carga la matriz calculando cada posicion con el metodo LCS a partir de las lineas de dos archivos.
    /// Ejemplo:
    /// ```rust
    /// lcs(lineasArchivo1, lineasArchivo2);
    /// ```
    pub fn lcs(&mut self, x: &[String], y: &[String]) {
        for (i, string1) in x.iter().enumerate().take(self.m - 1) {
            for (j, string2) in y.iter().enumerate().take(self.n - 1) {
                if string1 == string2 {
                    self.elementos[(j + 1) * (self.m - 1) + (i + 1)] =
                        self.elementos[(j) * (self.m - 1) + (i)] + 1;
                } else {
                    self.elementos[(j + 1) * (self.m - 1) + (i + 1)] = cmp::max(
                        self.elementos[j * (self.m - 1) + (i + 1)],
                        self.elementos[(j + 1) * (self.m - 1) + i],
                    )
                }
            }
        }
    }

    /// Devuelve el elemento que está en la posicion pedida de la matriz.
    /// Ejemplo:
    /// ```rust
    /// elemento_en_pos(numfil, numCol);
    /// ```
    pub fn elemento_en_pos(&self, x: usize, y: usize) -> u32 {
        self.elementos[y * (self.m - 1) + x]
    }
}
