use encoding_rs::SHIFT_JIS;
use std::io;
use std::io::{BufWriter, Read, Write};

const BUF_SIZE: usize = 0x10000;

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let stdout = io::stdout();
    let stdout = stdout.lock();
    let mut stdout = BufWriter::new(stdout);

    let mut decoder = SHIFT_JIS.new_decoder();

    let mut input = vec![0; BUF_SIZE];

    let mut output_buf = vec![0; BUF_SIZE * 3];
    let mut output: &mut str = std::str::from_utf8_mut(&mut output_buf).unwrap();

    let mut left = 0;
    loop {
        let len = stdin.read(&mut input[left..])?;
        if len == 0 {
            break;
        }

        let len = left + len;
        let (_, read, written, _) = decoder.decode_to_str(&input[..len], &mut output, false);

        input.copy_within(read..len, 0);
        left = len - read;

        stdout.write(output[..written].as_bytes())?;
        stdout.flush()?;
    }

    Ok(())
}
