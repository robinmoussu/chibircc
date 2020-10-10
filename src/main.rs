use color_eyre::eyre::Result;
use eyre::WrapErr;

fn parse_number(input: &str) -> Result<(isize, usize), std::num::ParseIntError> {
    let len = input.bytes().take_while(|c| c.is_ascii_digit()).count();
    let number = input[..len].parse::<isize>()?;
    Ok((number, len))
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let argv = clap::App::new("app")
        .about(clap::crate_description!())
        .about("Compute an expression, and return it as the exit code")
        .arg(
            clap::Arg::with_name("expression")
                .help("the mathematical expression that is going to be computed")
                .required(true),
        )
        .get_matches();

    let input: &str = argv.value_of("expression").unwrap();
    eyre::ensure!(!input.is_empty(), "the expression must be non empty");

    let mut index = 0;

    println!("  .globl main");
    println!("main:");

    let (number, len) = parse_number(&input[index..]).wrap_err("expecting a number at the beggining of the expression")?;
    index += len;
    println!("  mov ${}, %rax\n", number);

    while index < input.len() {
        if '+' == input.bytes().nth(index).unwrap().into() {
            index += 1;
            eyre::ensure!(index < input.len(), "unexpected end of input after '+'");
            let (number, len) = parse_number(&input[index..])
                    .wrap_err_with(|| format!("expecting a number after '+' at index {}", index))?;
            index += len;
            println!( "  add ${}, %rax", number);
            continue;
        }

        if '-' == input.bytes().nth(index).unwrap().into() {
            index += 1;
            eyre::ensure!(index < input.len(), "unexpected end of input after '-'");
            let (number, len) = parse_number(&input[index..])
                    .wrap_err_with(|| format!("expecting a number after '-' at index {}", index))?;
            index += len;
            println!( "  sub ${}, %rax", number);
            continue;
        }

        eprintln!(
            "unexpected character: '{}'",
            char::from(input.bytes().nth(index).unwrap())
        );
        std::process::exit(-1);
    }

    println!("  ret");

    Ok(())
}
