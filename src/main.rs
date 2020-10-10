use color_eyre::eyre::Result;
use eyre::WrapErr;

fn main() -> Result<()> {
    color_eyre::install()?;

    let argv = clap::App::new("app")
        .about(clap::crate_description!())
        .arg(
            clap::Arg::with_name("exit code")
                .help("value returned by the exit code")
                .required(true),
        )
        .get_matches();

    let input: &str = argv.value_of("exit code").unwrap();
    eyre::ensure!(!input.is_empty(), "the exit code must be non empty");
    let exit_code = input.parse::<isize>().wrap_err("the exit code must be a number")?;

    println!("  .globl main\n");
    println!("main:\n");
    println!("  mov ${}, %rax\n", exit_code);
    println!("  ret\n");

    Ok(())
}
