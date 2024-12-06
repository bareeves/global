
use tokio::fs::{self, File};//, OpenOptions};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StorageDirectoryError {
    #[error("I/O error occurred: {0}")]
    Io(#[from] io::Error),

    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("Invalid operation: {0}")]
    InvalidOperation(String),

    #[error("Failed to read a portion of the file: {0}")]
    ReadPortionError(io::Error), // Manually specify io::Error here

    #[error("File creation failed: {0}")]
    FileCreationError(String),
}

pub struct StorageDirectory{
    path: PathBuf,
    category:String,
    storage_files_count:Option<u32>,
}

const MAX_STORAGE_FILE_INDEX:u32=50000000;

impl StorageDirectory{
    /// Creates a new `StorageDirectory` instance.
    pub async fn new<P: AsRef<Path>>(path: P,category: String) -> Result<Self,StorageDirectoryError> {
        let path = path.as_ref().to_path_buf();
        if !path.exists() {
            fs::create_dir_all(&path).await?;
        }
        println!("StorageDirectory - path:{:?} category {:?}", path,category);

        Ok(Self { path, category,storage_files_count:None})
    }

    /// Lists all files in the directory.
    pub async fn list_files(&self) -> Result<Vec<PathBuf>,StorageDirectoryError> {
        let mut files = Vec::new();
        let mut entries = fs::read_dir(&self.path).await?;
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.is_file() {
                files.push(path);
            }
        }
        Ok(files)
    }


    /// load the contents of a file in the directory.
    pub async fn load_bytes_from_file(&self, filename: &str) -> Result<Vec<u8>,StorageDirectoryError> {
        let file_path = self.path.join(filename);
        let mut file = File::open(file_path.clone()).await.map_err(|e| match e.kind() {
            io::ErrorKind::NotFound => StorageDirectoryError::FileNotFound((file_path.clone()).to_str().expect("Failed to convert path to string").to_string()),
            _ => StorageDirectoryError::Io(e),
        })?;
    
        let mut contents = vec![];
        file.read_to_end(&mut contents).await.map_err(StorageDirectoryError::Io)?;
        Ok(contents)
    }

    /// save data to a file in the directory (creates the file if it doesn't exist).
    pub async fn save_bytes_to_file(&self, filename: &str, data: &[u8]) -> Result<(),StorageDirectoryError> {
        let file_path = self.path.join(filename);
        let mut file = File::create(file_path).await.map_err(|e| StorageDirectoryError::FileCreationError(e.to_string()))?;
        file.write_all(data).await.map_err(StorageDirectoryError::Io)?;
        Ok(())
    }
    
    /// Deletes a file in the directory.
    pub async fn delete_file(&self, filename: &str) -> Result<(),StorageDirectoryError> {
        let file_path = self.path.join(filename);
        if !file_path.exists() {
            return Err(StorageDirectoryError::FileNotFound(filename.to_string()));
        }
        if file_path.is_file() {
            fs::remove_file(file_path).await?;
        }
        Ok(())
    }

    /// Checks if a file exists in the directory.
    pub async fn file_exists(&self, filename: &str) -> bool {
        println!("does {:?} file_exists",filename);
        self.path.join(filename).exists()
    }
    pub fn get_storage_files_count(&self) -> Option<u32> { 
        self.storage_files_count
    }
    pub async fn init_storage_files_count(&mut self) {
        let mut i:u32=0;     
        while i<MAX_STORAGE_FILE_INDEX {
            let formatted_string = format!("{}{}", self.category,i);
            let string_slice: &str = formatted_string.as_str();
            if !(self.file_exists(string_slice).await) {
                break
            }
            i+=1;
        }
        self.storage_files_count=Some(i);
    }

}
