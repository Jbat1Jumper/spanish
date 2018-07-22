
Spanish, o Español
==================

Instrucciones para correrlo
---------------------------

El punto de entrada para esto es el script de python `./run`. Se puede ver los
comandos disponibles y que hacen utilizando:

```
$ ./run --help

#  Usage: run [OPTIONS] COMMAND [ARGS]...
#  
#    This script is a tool to interact easily with the proyect and prepare the
#    data.
#  
#  Options:
#    --help  Show this message and exit.
#  
#  Commands:
#    clean    Delete the files created with 'prepare'.
#    fixer    This should run the rust code to parse an XML...
#    prepare  Prepare the data to process it with rust.
```

**Notas**: No freezé las dependencias por lo que puede que haya que instalar algunos
paquetes. También hay algunas excepciones que ocurren a la hora de hacer el
clean si el build quedó por la mitad, debidos a que espera que todos los
archivos a borrar existan. Creo que arreglar eso es más rápido que explicarlo
por escrito como ahora, pero se fixeará en algún momento.

Log
===

22 de Julio
-----------

Bueno, en principio esto arranca como un intento de aplicación para prácticar
ortografía, principalmente reglas de acentuación. Para ello necesité un listado
de palabras del español para utilizar como punto de partida. Buscando en la
internet me crucé con un [corpus interesante][1] de muchos artículos del
español y me distraje intentando leerlo.

Hasta ahora me topé con varias dificultades, sobretodo en el proceso de parseo.
Leer un archivo zip en memoria desde rust fue bastante ameno utilizando
[zip-rs][2]. Con tan solo ...

```rust
extern crate zip;
use std::fs::File;
use zip::read::ZipArchive;

let file = File::open("my_file.zip").unwrap();
let mut zip = ZipArchive::new(file).unwrap();

for i in 0..zip.len() {
    let mut file = zip.by_index(i).unwrap();
};
```

... se puede iterar por todos los archivos del zip. Los problemas con el corpus
comenzaron a aparecer con el parseo de xml. En primer lugar la librería [xml-rs][3]
solo soporta UTF-8 como encoding. El corpus en cuestión estaba en ISO-8859-1 por lo
que hizo falta reencodearlo. En el script `./run` el comando `prepare` se encarga
de reencodear el data corpus. 

Luego de cambiarle el encoding, comenzaron a haber errores de formato del XML.
Uno de los problemas principales son los caracteres no escapeados. Se agregó al
comando `prepare` un procesamiento del archivo para reemplazar ocurrencias de
caracteres molestos. Cómo el formato del archivo, aparte de ser XML, sigue un
patrón sencillo (una linea para abrir un tag de XML, contenido del articulo,
una linea que dice _ENDOFARTICLE._, y una linea para cerrar el tag) es fácil de
identificar y extraer el contenido sin utilizar XML.

Sin embargo para acceder comodamente a la metadata de los articulos hay que
parsear los tags. Desafortunadamente también ocurre que en las lineas de los
tags tambien hay caracteres sin escapear (dobles comillas, simbolos de mayor o
menor en los attributos, etc).

La idea ahora es utilizar los errores de parseo de la librería para arreglar,
utilizando rust mismo, el archivo XML para a futuro poder procesarlo
directamente desde el archivo zip.




[1]: https://www.kaggle.com/rtatman/120-million-word-spanish-corpus#spanish_corpus.zip
[2]: https://github.com/mvdnes/zip-rs
[3]: https://github.com/netvl/xml-rs

