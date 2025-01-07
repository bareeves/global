use utility::hash::hash::Hash;
use utility::buffer::buffer_writer::BufferWriter;

use crate::wallet_v1::resource_info::ResourceInfo;
use crate::wallet_v1::resource_info::UnspentResourceInfo;
use crate::wallet_v1::resource_info::SpentResourceInfo;
//
#[derive(Debug)]
#[derive(Clone)] 
pub struct Resource {
    pub hash: Hash,
    pub index: u32,
    pub value:u64,
    pub key_index:usize,
    pub available:bool,
    //pub status: Vec<u8>,
    pub info:ResourceInfo,
}
//
impl Resource {
    /*
    pub fn is_eq(&self,h:Hash,index:u32)->bool {

    }*/
    pub fn update_resource_to_spent(&mut self){
        ////let mut tmpbw = BufferWriter::new();
        ////tmpbw.put_u32(ASSET_STATUS_IDENTIFIER_SPENT);
        //tmpbw.put_var_u64(pubkeycompressedbytes.len() as u64);
        //tmpbw.put_bytes(pubkeycompressedbytes);
        //tmpbw.put_var_u64(0); // TODO extend to support extradata
        //
        ////self.status.clear();
        ////self.status.extend_from_slice(&tmpbw.get_bytes());
        self.info=ResourceInfo::SpentResourceInfoVariant(SpentResourceInfo{});
    }
    pub fn is_unspent_resource(&self)-> bool{
        self.info.is_unspent_resource_info()
    }
}

//
pub fn new_unspent_resource(h:Hash,tmpindex:u32,value:u64,key_index: usize) ->Resource {

    let mut new_resource= Resource {
        hash: h,
        index: tmpindex,
        value,
        key_index,
        available:true,
        //status: Vec::new(),
        info:ResourceInfo::UnspentResourceInfoVariant(UnspentResourceInfo{}),
    };
    //let mut tmpbw = BufferWriter::new();
    //tmpbw.put_u32(ASSET_STATUS_IDENTIFIER_UNSPENT);
    //tmpbw.put_var_u64(pubkeycompressedbytes.len() as u64);
    //tmpbw.put_bytes(pubkeycompressedbytes);
    //tmpbw.put_var_u64(0); // TODO extend to support extradata
    //
    ////new_resource.status.extend_from_slice(&tmpbw.get_bytes());
    return new_resource
}