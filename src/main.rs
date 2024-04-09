use std::fs;
use std::fs::File;
use std::io::Write;
use clap::Parser;
use scraper::{Html, Selector};
use indicatif::{ProgressBar, ProgressStyle};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// EPUB file to parse
    #[arg(short, long)]
    epub_path: String,

    /// output txt file to generate the definitions of each word
    #[arg(short, long)]
    output_path: String,
}

// Sample args: --epub_path "~/Desktop/Diccionario de\ la\ Lengua\ Española.epub" --output_path "output.txt"
// /Users/danielvela/Desktop/Diccionario de la Lengua Española.epub/OEBPS/Text
fn main() -> std::io::Result<()>  {
    let args = Args::parse();
    let files_path: String = args.epub_path + "/OEBPS/Text/";

    let mut output_file = File::create(args.output_path)?;

    // println!("Parsing {} file\nGenerating {}", files_path, args.output_path);

    // Read each html file in the path
    let paths: Vec<_> = fs::read_dir(files_path).unwrap().collect();
    
    let count = paths.len();
    println!("Número de archivos/directorios: {}", count);

    // Crea una nueva barra de progreso con un total de 100
    let bar = ProgressBar::new(paths.len() as u64);

    let count = paths.len();
    println!("Número de archivos/directorios: {}", count);

    // Configura el estilo de la barra de progreso
    bar.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos:>7}/{len:7} ({eta})")
        .expect("Error al configurar el template")
        .progress_chars("#>-"));

    for path in paths {
        let path = path.unwrap().path();
        let file_name = path.file_name().unwrap().to_str().unwrap();
        // println!("{}", file_name);
        if file_name.ends_with(".xhtml") {
            let file = fs::read_to_string(path).unwrap();
            let document = Html::parse_document(&file);

            // Selector para el párrafo completo
            let p_selector = Selector::parse(".asangre").unwrap();
            // Selector para la palabra
            let word_selector = Selector::parse("b.masnegrita").unwrap();

            // Buscar el párrafo
            for p_element in document.select(&p_selector) {
                // Extraer la palabra
                let palabra = p_element.select(&word_selector)
                                    .next()
                                    .map(|n| n.inner_html())
                                    .unwrap_or_default();
                
                // Extraer todo el texto del párrafo y luego limpiarlo
                let mut definicion = p_element.text().collect::<Vec<_>>().join("");
                // Remover la palabra de la definición
                definicion = definicion.replace(&format!("-&gt;{}", palabra), "").trim().to_string();
                
                writeln!(output_file, "{},{}", palabra, definicion)?;
            }
        }
        bar.inc(1);
        // std::thread::sleep(std::time::Duration::from_millis(20000000));
    }
    bar.finish_with_message("Process completed!");
    Ok(())
}