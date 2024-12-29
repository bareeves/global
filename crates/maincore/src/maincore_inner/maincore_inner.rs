use tokio::fs;
use tokio::io;
use std::path::Path;
use std::path::PathBuf;
use thiserror::Error; 
use utility::storage::storage_directory::StorageDirectory;
use utility::storage::storage_directory::StorageDirectoryError;

// MainCoreError Definition
#[derive(Debug, Error)]
pub enum MaincoreInnerError {
    #[error("I/O error: {0}")]
    IoError(#[from] io::Error),

    //#[error("Custom error: {0}")]
    //Custom(String),

    //#[error("BigInt error: {0}")]
    //BigIntError(String),

    //#[error("Chunk storage error: {0}")]
    //ChunkStorageError(String),

    //#[error("Header serialization error: {0}")]
    //SerializationError(String),
    //////////////////////////////////////////////
    /////////////////////////////////////////////
    //#[error("Buffer reader error: {0}")]
    //BufferReaderError(#[from] BufferReaderError),
    #[error("Storage directory error: {0}")]
    StorageDirectoryError(#[from] StorageDirectoryError),
}

// Define the MainCoreInner struct
//#[derive(Debug)] // Implementing Debug trait for MainCoreInner
pub struct MaincoreInner {
    mci_path: PathBuf,
    main_sd: StorageDirectory,
    //header_vector:Vec<MainHeader>,
    confimation_depth:usize,
    //syncpool:Syncpool,
    //mainstate:Mainstate,
    //txspool:Maintxspool,
    //miner:Miner,
    
}

impl MaincoreInner{
    /// Creates a new `StorageDirectory` instance.
    pub async fn new<P: AsRef<Path>>(mci_path: P) -> Result<Self,MaincoreInnerError> {
        let mci_path = mci_path.as_ref().to_path_buf();
        if !mci_path.exists() {
            fs::create_dir_all(&mci_path).await?;
        }
        
        println!("MaincoreInner - path:{:?} ", mci_path);

        let sd_sub_path_buf=PathBuf::from("MainBlocks");// can be string but should be PathBuf
        let sd_path_buf=mci_path.join(sd_sub_path_buf);
        let main_sd= StorageDirectory::new(sd_path_buf,String::from("MainBlock")).await?;

        Ok(Self { mci_path,main_sd, confimation_depth:100})
    }
}