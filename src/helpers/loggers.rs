#[macro_export]
macro_rules! info {
    ( $($arg: tt)* ) => {
        {
            let mut stdout = termcolor::StandardStream::stdout(termcolor::ColorChoice::Always);
            if stdout.set_color(termcolor::ColorSpec::new().set_fg(Some(termcolor::Color::Cyan))).is_ok() {
                writeln!(&mut stdout, $($arg)*).expect("Something wrong has happened!");
                stdout.reset().expect("Something wrong has happened!");
            }
        }
    };
}

#[macro_export]
macro_rules! error {
    ( $($arg: tt)* ) => {
        {
            let mut stdout = termcolor::StandardStream::stdout(termcolor::ColorChoice::Always);
            if stdout.set_color(termcolor::ColorSpec::new().set_fg(Some(termcolor::Color::Rgb(255, 51, 51)))).is_ok() {
                writeln!(&mut stdout, $($arg)*).expect("Something wrong has happened!");
                stdout.reset().expect("Something wrong has happened!");
            }
        }
    };
}

#[macro_export]
macro_rules! reset {
    ( $($arg: tt)* ) => {
        {
            termcolor::StandardStream::stdout(termcolor::ColorChoice::Always).reset().expect("Something wrong has happened!");
        }
    };
}
