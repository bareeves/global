pub const RESOURCE_INFO_IDENTIFIER_UNSPENT : u32=1;
pub const RESOURCE_INFO_IDENTIFIER_SPENT : u32=2;

// Define a struct for different ResourceInfo
#[derive(Debug)]
#[derive(Clone)] 
pub struct UnspentResourceInfo {
    //radius: f64,
}
#[derive(Debug)]
#[derive(Clone)] 
pub struct SpentResourceInfo {
    //pub width: f64,
    //pub height: f64,
}

// Define an enum that contains different ResourceInfo
#[derive(Debug)]
#[derive(Clone)]
pub enum ResourceInfo {
    UnspentResourceInfoVariant(UnspentResourceInfo),
    SpentResourceInfoVariant(SpentResourceInfo),
}

// Implement an area function for each ResourceInfo
impl UnspentResourceInfo {
    //pub fn area(&self) -> f64 {
        //std::f64::consts::PI * self.radius * self.radius
    //    0.0
    //}
}

impl SpentResourceInfo {
    //pub fn area(&self) -> f64 {
        //self.width * self.height
    //    1.0
    //}
}

// Implement an area function for the enum
impl ResourceInfo {
    //pub fn area(&self) -> f64 {
    //    match self {
    //        ResourceInfo::UnspentResourceInfoVariant(unspent_resource_info) => unspent_resource_info.area(),
    //        ResourceInfo::SpentResourceInfoVariant(spent_resource_info) => spent_resource_info.area(),
    //    }
    //}
    pub fn is_unspent_resource_info(&self) -> bool {
        match self {
            ResourceInfo::UnspentResourceInfoVariant(unspent_resource_info) => true,
            ResourceInfo::SpentResourceInfoVariant(spent_resource_info) => false,
        }
    }
}

