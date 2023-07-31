use std::env;
use std::fs::{File, metadata};
use std::io::{self, prelude::*, BufReader, BufWriter};
use gmod_lzma::{compress, decompress};

const CHUNK_SIZE: usize = 1024 * 1024; // 1 MB

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: {} [compress|decompress] [input_file] [output_file]", args[0]);
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Invalid number of arguments",
        ));
    }

    let operation = &args[1];
    let input_file_path = &args[2];
    let output_file_path = &args[3];

    let total_size = metadata(&input_file_path)?.len() as usize;

    let input_file = File::open(input_file_path)?;
    let mut reader = BufReader::new(input_file);

    let output_file = File::create(output_file_path)?;
    let mut writer = BufWriter::new(output_file);

    let mut buffer = vec![0; CHUNK_SIZE];

    let mut processed_size = 0;

    if operation == "compress" {
        while let Ok(n) = reader.read(&mut buffer) {
            if n == 0 {
                break;
            }
            let compressed = compress(&buffer[..n] , 9).map_err(|e| {
                io::Error::new(
                    io::ErrorKind::Other,
                    format!("Compression failed: {}", e),
                )
            })?;
            writer.write_all(&compressed)?;

            processed_size += n;
            print_progress(processed_size, total_size);
        }
    } else if operation == "decompress" {
        while let Ok(n) = reader.read(&mut buffer) {
            if n == 0 {
                break;
            }
            let decompressed = decompress(&buffer[..n]).map_err(|e| {
                io::Error::new(
                    io::ErrorKind::Other,
                    format!("Decompression failed: {}", e),
                )
            })?;
            writer.write_all(&decompressed)?;

            processed_size += n;
            print_progress(processed_size, total_size);
        }
    } else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Invalid operation. Use 'compress' or 'decompress'",
        ));
    }

    Ok(())
}

fn print_progress(processed: usize, total: usize) {
    let percentage = (processed as f32 / total as f32) * 100.0;
    eprintln!("Processed: {}/{} bytes ({:.2}%)", processed, total, percentage);
}
