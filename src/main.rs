use std::{
    env,
    fs::{self, File},
    io::{self, BufReader, BufWriter, Read, Write},
    path::{Path, PathBuf},
};

const DEFAULT_DIRECTORY: &str = ".beep";
const USAGE: &str = r#"
    Beep [options] <filepath>+
    Convert markdown to html files and open them using the default mime application
         Examples: 
            beep a.md
"#;

fn process_file(filepath: &PathBuf, output_file: &PathBuf) -> io::Result<()> {
    let file = File::options().read(true).open(filepath)?;
    let mut file = BufReader::new(file);
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let html = markdown::to_html(&content);

    let file = File::options()
        .create(true)
        .write(true)
        .truncate(true)
        .open(output_file)?;
    let mut file = BufWriter::new(file);
    file.write_all(html.as_bytes())?;
    Ok(())
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0); // strip off program name

    if args.is_empty() {
        println!("{USAGE}");
        return;
    }

    let home = env::home_dir().unwrap();
    let output = Path::new(DEFAULT_DIRECTORY);
    let out_dir = home.join(output);

    if !out_dir.exists() {
        fs::create_dir(&out_dir).expect("create dir failed");
    }

    for arg in args {
        let arg = arg.as_str();
        if arg == "--help" || arg == "-h" {
            println!("{USAGE}");
            return;
        }

        let file = Path::new(arg).to_path_buf();
        if let Some(ext) = file.extension() {
            if ext != "md" {
                continue;
            }
        }

        let filename = format!(
            "{filename}.html",
            filename = file.file_stem().unwrap_or_default().to_string_lossy()
        );
        let filename = Path::new(&filename);
        let output = out_dir.join(filename);

        if let Err(err) = process_file(&file, &output.to_path_buf()) {
            eprintln!("Failed to process file: {err:?}");
            continue;
        };

        let status = match std::process::Command::new("handlr")
            .arg("open")
            .arg(&output)
            .status()
        {
            Ok(st) => st,
            Err(e) => {
                eprintln!("Failed to open handlr: {e}; Is it Installed on your system?");
                return;
            }
        };
        if !status.success() {
            eprintln!("Handlr exited with a non zero code");
        }
        println!("Opened {:?} in default browser application", &output);
    }
}
