use tokio::fs;
use tokio::io;
use std::path::Path;
use std::path::PathBuf;
use thiserror::Error; 
use utility::storage::storage_directory::StorageDirectory;
use utility::storage::storage_directory::StorageDirectoryError;
use utility::hash::bigint;
use crate::mainheader::mainheader::Mainheader;
use crate::mainblock::mainblock::Mainblock;
use crate::mainblock::mainblock::unserialize_mainblock;
use crate::mainblock::mainblock::MainblockError;


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
    #[error("Mainblock error: {0}")]
    MainblockError(#[from] MainblockError),
}

// Define the MainCoreInner struct
//#[derive(Debug)] // Implementing Debug trait for MainCoreInner
pub struct MaincoreInner {
    mci_path: PathBuf,
    main_sd: StorageDirectory,
    header_vector:Vec<Mainheader>,
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

        let sd_sub_path_buf=PathBuf::from("Mainblocks");// can be string but should be PathBuf
        let sd_path_buf=mci_path.join(sd_sub_path_buf);
        let main_sd= StorageDirectory::new(sd_path_buf,String::from("Mainblock")).await?;

        //Ok(Self { mci_path,main_sd, confimation_depth:100})

        Ok(Self { 
            mci_path,
            main_sd,
            header_vector: Vec::new(),//
            confimation_depth: 6,
            //syncpool:Syncpool::new(),
            //mainstate:tmp_ms,
            //txspool:Maintxspool::new(),
            //miner:Miner::new(),
        })
    }
    pub async fn init(&mut self)-> Result<(),MaincoreInnerError> {
        self.init_storage_directory().await?;
        //self.syncpool.init()?;
        Ok(())
    }
    pub async fn init_storage_directory(&mut self)-> Result<(),MaincoreInnerError> {
        match self.main_sd.init().await {
            Ok(_)=> {
                println!("StorageDirectory init success");
                //
                let index_result=self.main_sd.get_storage_files_last_index();
                match index_result {
                    Some(index) =>{
                        println!("get_storage_files_last_index: {}", index);
                    },
                    None => {
                        println!("storage files last index not initialized");
                        //TODO add genesis block here
                    }, 
                }        
                return Ok(());
            }
            Err(e)=> {
                println!("StorageDirectory init error {:?}",e);
                return Err(MaincoreInnerError::StorageDirectoryError(e))
            }
        }
        Ok(())
    }
    pub fn get_mainblocks_count(&self) -> usize {
        let index_result=self.main_sd.get_storage_files_last_index();
        match index_result {
            Some(index) => return index+1,//println!("get_storage_files_last_index: {}", index),
            None => return 0,//println!("storage files last index not initialized"), 
        }
    }
    pub async fn add_confirmed_mainblock(&mut self,mb: Mainblock)-> Result<(),MaincoreInnerError> {
        println!("************ add_confirmed_mainblock");
        let tmpheader=mb.get_mainheader();
        self.header_vector.push(tmpheader);
        if self.header_vector.len()>1 {
            /*
            for i in 1..self.header_vector.len(){
                println!("i {}",i);
                println!("self.header_vector[i].get_timestamp() {} self.header_vector[i-1].get_timestamp() {}",self.header_vector[i].get_timestamp(),self.header_vector[i-1].get_timestamp());
                process::exit(1);
            }
            */
            println!("self.header_vector.len() {}",self.header_vector.len());
            let tmp_height=self.header_vector.len()-1;
            let deltatimestamp=self.header_vector[tmp_height].get_timestamp()-self.header_vector[tmp_height-1].get_timestamp();
            println!("deltatimestamp {}",deltatimestamp);
            /*
            if deltatimestamp==0 {
                process::exit(1);
            }
            */
        }
        //All the txs that have been included in a confimred block will be removed from the txspoll (self.txspool.remove(tmp_hash))
        //TODONOW with self.txspool.remove(tmp_hash)

        //All the txs that have been frozen because they have been included in a certain block height WILL BE reset (reset_tx_with_mainblock_height)
        //TODONOW with reset_tx_with_mainblock_height 

        let mb_rawbytes=mb.serialize();
        match self.main_sd.add_chunk(mb_rawbytes.as_slice()).await {
            Ok(_)=> {
                println!("StorageDirectory add_chunk success");
                Ok(())
            }
            Err(e)=> {
                println!("StorageDirectory add_chunk error {:?}",e);
                return Err(MaincoreInnerError::StorageDirectoryError(e))
            }
        }
    }
    pub async fn get_mainblock(&mut self,block_height: usize)-> Result<Mainblock,MaincoreInnerError>{
        match self.main_sd.get_chunk(block_height).await {
            Ok(mb_rawbytes)=> {
                println!("StorageDirectory get_chunk success");
                println!("mb_rawbytes {:?}",mb_rawbytes);
                let mb=unserialize_mainblock(mb_rawbytes)?;
                Ok(mb)
            }
            Err(e)=> {
                println!("StorageDirectory get_chunk error {:?}",e);
                Err(MaincoreInnerError::StorageDirectoryError(e))
            }
        }
    }
    //
    pub async fn load_mainheaders(&mut self) -> Result<(),MaincoreInnerError> {
        let tmpblocks_count=self.get_mainblocks_count();
        println!("loading mainheaders - number of mainblocks {}",tmpblocks_count);
        if tmpblocks_count==0 {
            //Err(e) => {
                //return Err(eprintln!("))
                //self.check_genesis().await?;
                println!("load_headers finished-no mainheaders");
                return Ok(());
                //return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Maincore ChunckStorage is empty get_blocks_count()==0")))
           // }
        }
        for i in 0..tmpblocks_count {

            match self.get_mainheader(i).await {
                Ok(tmpheader)=> {
                    self.header_vector.push(tmpheader);
                    println!("loaded mainheader {}",i);
                    //return Ok(());
                }
                Err(e)=> {
                    println!("load_headers error {:?}",e);
                    return Err(e);
                }
            }
        }
        println!("tmpblocks_count {}",tmpblocks_count);
        let tmp_height=self.header_vector.len();
        println!("load_headers finished with {} mainheaders loaded",tmp_height);
        Ok(())
    }
    //
    pub async fn get_mainheader(&mut self,header_height: usize)-> Result<Mainheader, MaincoreInnerError> {
        match self.main_sd.get_chunk(header_height).await {
            Ok(mb_rawbytes)=> {
                //println!("ChunksStorage get_chunk success");
                let mb=unserialize_mainblock(mb_rawbytes)?;
                Ok(mb.get_mainheader())
            }
            Err(e)=> {
                println!("ChunksStorage get_chunk error {:?}",e);
                Err(MaincoreInnerError::StorageDirectoryError(e))
            }
        }
    }
    pub fn get_last_inmem_mainheader(&self)-> Result<Mainheader, MaincoreInnerError> {
        let last_block_height=(self.header_vector.len())-1;
        //let last_block_height1=self.get_blocks_count()-1;
        //println!("*********** last_block_height {} {}",last_block_height1,last_block_height);
        Ok(self.header_vector[last_block_height].clone())
        
    }
    pub fn get_inmem_mainheader(&self,header_height: usize)-> Result<Mainheader, MaincoreInnerError> {
        Ok(self.header_vector[header_height].clone()) 
    }
    pub fn get_newbits(&mut self)-> u32 {
        //let newbits:u32;
        
        let tmp_height=self.header_vector.len();
        let prev_mainheader=self.header_vector[tmp_height-1].clone();
        if tmp_height % 4032 ==0 {
            let mut summedtimestamp:i64=0;
            for i in (tmp_height-4031)..(tmp_height) {
                let deltatimestamp=self.header_vector[i].get_timestamp()-self.header_vector[i-1].get_timestamp();
                if deltatimestamp==0 {
                    println!("check deltatimestamp {} for i {}",deltatimestamp,i);
                    println!("self.header_vector[i].get_timestamp() {} self.header_vector[i-1].get_timestamp() {}",self.header_vector[i].get_timestamp(),self.header_vector[i-1].get_timestamp());
                    //process::exit(1);
                }
                summedtimestamp+=deltatimestamp;
            }
            let bitsbigint=bigint::bigint_from_compact(prev_mainheader.get_bits());
            let newbitsbigint=bitsbigint.clone()*bigint::bigint_from_u64(summedtimestamp as u64)/bigint::bigint_from_u64((4031*300) as u64);
            /*
            println!("summed over interval from {} to {}",tmp_height-4031,tmp_height-1);
            println!("summedtimestamp {} idealtime {}",summedtimestamp,4031*300);
            println!("check deltatimestamp {}",self.header_vector[1].get_timestamp()-self.header_vector[0].get_timestamp());
            println!("oldbits {} newbits {}",bitsbigint,newbitsbigint);
            println!("oldbits compact {} newbits compact {}",bigint::compact_from_bigint(&bitsbigint),bigint::compact_from_bigint(&newbitsbigint));
            println!("newbits {}",bigint::compact_from_bigint(&newbitsbigint));
            println!("oldbits {}",bigint::compact_from_bigint(&bitsbigint));
            process::exit(1);
            */
            return bigint::compact_from_bigint(&newbitsbigint);

        } else {
            return prev_mainheader.get_bits();
        }
    }

}