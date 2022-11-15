use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};

fn main() -> Result<(), Error> {
    let my_filter = "II-МР";
    let mut output_file = File::create("new_data.txt")?;

    let file_open = File::open("data_utf8.txt")?;
    let buffered = BufReader::new(file_open);

    for line in buffered.lines() {
        let result = &line?;
        if result.contains(&my_filter) {
            //println!("{}", result);
            write!(output_file, "{}\n", result)?;
        }
    }
    Ok(())
}
