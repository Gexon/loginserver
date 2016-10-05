extern crate crypto;

pub fn StringToSHA2(inString: &str) -> String {}

pub fn usize_to_byte(size: usize) -> u8 {
    let i: i64 = 12345;
    let ip: *const i64 = &i;
    let bp: *const u8 = ip as *const _;
    let bs: &[u8] = unsafe {
        slice::from_raw_parts(
            bp,
            mem::size_of::<i64>()
        )
    };
}

pub fn encode_u8(v: u8) -> [u8; 1] {
    unsafe { transmute::<u8, [u8; 1]>(v.to_be()) }
}

pub fn decode_u8(v: &[u8]) -> u8 {
    let mut buf = [0u8; 1];
    buf.copy_from_slice(&v[..1]);
    u8::from_be(unsafe { transmute::<[u8; 1], u8>(buf) })
}


pub fn encode_u16(v: u16) -> [u8; 2] {
    unsafe { transmute::<u16, [u8; 2]>(v.to_be()) }
}

pub fn decode_u16(v: &[u8]) -> u16 {
    let mut buf = [0u8; 2];
    buf.copy_from_slice(&v[..2]);
    u16::from_be(unsafe { transmute::<[u8; 2], u16>(buf) })
}

fn to_network_order(v: u32) -> [u8; 4] {
    unsafe { ::std::mem::transmute(v.to_be()) }
}

fn main() {
    println!("{:?}", to_network_order(1));
}