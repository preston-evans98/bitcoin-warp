use byteorder::{LittleEndian, WriteBytesExt, BigEndian};
use std::net::{IpAddr, Ipv6Addr,Ipv4Addr};

pub trait Serializable {
    fn serialize(&self, target: &mut Vec<u8>);
}
pub trait BigEndianSerializable {
    fn serialize(&self, target: &mut Vec<u8>);
}

impl Serializable for u8 {
    fn serialize(&self, target: &mut Vec<u8>) {
        target.push(*self);
    }
}
impl Serializable for u16 {
    fn serialize(&self, target: &mut Vec<u8>) {
        target.write_u16::<LittleEndian>(*self).unwrap();
    }
}
impl BigEndianSerializable for u16{
    fn serialize(&self, target: &mut Vec<u8>) {
        target.write_u16::<BigEndian>(*self).unwrap();
    }
}
impl Serializable for u32 {
    fn serialize(&self, target: &mut Vec<u8>) {
        target.write_u32::<LittleEndian>(*self).unwrap();
    }
}
impl Serializable for u64 {
    fn serialize(&self, target: &mut Vec<u8>) {
        target.write_u64::<LittleEndian>(*self).unwrap();
    }
}
impl Serializable for std::net::Ipv6Addr{
    fn serialize(&self, target: &mut Vec<u8>){
        target.extend_from_slice(&self.octets());
    }
}
impl Serializable for std::net::Ipv4Addr{
    fn serialize(&self, target: &mut Vec<u8>){
        target.extend_from_slice(&self.octets());
    }
}
impl Serializable for &[u8] {
    fn serialize(&self, target: &mut Vec<u8>) {
        target.extend_from_slice(self)
    }
}
