use std::env;
use std::fs;
use std::io::Write;

fn main() {
    let args: Vec<String> = env::args().collect();
    // args[0] = executable.exe = executable
    // args[1] = Selam = executalbe.exe Selam

    match args.len() {
        1 => println!(""), // No argument given, print blank line as it like how echo do
        2 => println!("{}", args[1]), // One argument given, print it
        3 => println!(""),
        4 => write_file(&args[1], &args[3], &args[2]).expect("Invalid operator"),
        _ => println!("Usage: gekko <message> <operator> <file>"),
    }
}

// Overwrites given content into given file name
fn write_file(content: &String, file: &String, operator: &String) -> std::io::Result<()> {
    if operator == ">" {
        let mut f = fs::File::options()
            .create(true)
            .write(true)
            .truncate(true)
            .open(file)?;
        writeln!(&mut f, "{}", content)?;
        Ok(())
    } else if operator == ">>" {
        let mut f = fs::File::options()
            .create(true)
            .write(true)
            .append(true)
            .open(file)?;
        writeln!(&mut f, "{}", content)?;
        Ok(())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Invalid operator",
        ))
    }
}
