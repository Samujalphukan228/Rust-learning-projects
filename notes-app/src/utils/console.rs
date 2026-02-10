use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use std::io::Write;

pub fn print_color(text: &str, color: Color, bold: bool) {
    let mut stdout = StandardStream::stdout(ColorChoice::Auto);

    let mut spec = ColorSpec::new();
    spec.set_fg(Some(color)).set_bold(bold);

    stdout.set_color(&spec).unwrap();
    writeln!(&mut stdout, "{text}").unwrap();
    stdout.reset().unwrap();
}
