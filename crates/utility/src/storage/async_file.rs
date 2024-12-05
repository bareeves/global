//use std::io::{self, Write};
use std::io::{self};
use tokio::fs::{self, File, OpenOptions};
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AsyncFileError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("Failed to read a portion of the file: {0}")]
    ReadPortionError(io::Error), // Manually specify io::Error here

    #[error("File creation failed: {0}")]
    FileCreationError(String),
}

pub async fn save_bytes_to_file(data: &[u8], input_path: &str) -> Result<(), AsyncFileError> {
    let mut file = File::create(input_path).await.map_err(|e| AsyncFileError::FileCreationError(e.to_string()))?;
    file.write_all(data).await.map_err(AsyncFileError::Io)?;
    Ok(())
}

pub async fn load_bytes_from_file(input_path: &str) -> Result<Vec<u8>, AsyncFileError> {
    let mut file = File::open(input_path).await.map_err(|e| match e.kind() {
        io::ErrorKind::NotFound => AsyncFileError::FileNotFound(input_path.to_string()),
        _ => AsyncFileError::Io(e),
    })?;

    let mut contents = vec![];
    file.read_to_end(&mut contents).await.map_err(AsyncFileError::Io)?;
    Ok(contents)
}

pub async fn read_portion_of_file(file_path: &str, start: u64, end: u64) -> Result<Vec<u8>, AsyncFileError> {
    let mut file = File::open(file_path).await.map_err(AsyncFileError::Io)?;
    let mut buffer = vec![0; (end - start) as usize];

    file.seek(io::SeekFrom::Start(start)).await.map_err(AsyncFileError::Io)?;
    file.read_exact(&mut buffer).await.map_err(AsyncFileError::ReadPortionError)?;

    Ok(buffer)
}

pub async fn file_exists(file_path: &str) -> bool {
    fs::metadata(file_path).await.is_ok()
}

pub async fn get_file_size(file_path: &str) -> u64 {
    fs::metadata(file_path)
        .await
        .map(|metadata| metadata.len())
        .unwrap_or(0)
}

pub async fn append_to_file(
    file_path: &str,
    data: &[u8],
    create_file: bool,
    add_bytes_size: bool,
) -> Result<(), AsyncFileError> {
    // Open or create the file with write and append options
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(create_file)
        .open(file_path)
        .await
        .map_err(|e| AsyncFileError::Io(e))?;

    // If requested, write the size of the data before appending the actual data
    if add_bytes_size {
        let data_length = data.len() as u32;
        let mut tmp_buffer = [0u8; 4];

        // Copy the data length into a buffer as little-endian bytes
        tmp_buffer.copy_from_slice(&data_length.to_le_bytes());

        // Write the data length as a 4-byte prefix before the actual data
        file.write_all(&tmp_buffer).await.map_err(AsyncFileError::Io)?;
    }

    // Write the actual data to the file
    file.write_all(data).await.map_err(AsyncFileError::Io)?;

    Ok(())
}
