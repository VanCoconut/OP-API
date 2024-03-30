// Enumcon “valore”
// A differenza del C le enum sono tipi che possono ospitare all’interno valore associato
// agli elementi della enum.
// Ogni elemento di una enum può essere di un tipo diverso e con match si può estrarre
// il contenuto della enum in una variabile.
// Definire quindi una enum Error con dentro due valori: Simple(SystemTime) e
// Complex(SystemTime, String) e fare una funzione print_error(e: Error) che
// stampi il tipo di errore e le informazioni content (senza usare {:?} debug, ma
// gestendo i valori della enum in modo opportuno

use std::time::SystemTime;

pub  enum Error{
    Simple(SystemTime),
    Complex(SystemTime, String)
}

fn print_error(e: Error)->