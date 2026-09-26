//#![no_std]
mod alphabet;
pub use crate::alphabet::*;

pub fn str_to_alphabet(str_alphabet:&str)->&[u8]{
    str_alphabet.as_bytes()
}

pub fn alphabet_to_str(alphabet:&[u8])->&str{
    core::str::from_utf8(alphabet).unwrap()
}

pub const fn base<'a>(x:u8)->&'a[u8]{// Limited to u8 and redeclared to usize, because I want to make bases over 255 impossible. Note that base(256) is not possible (but possible via constant BASE256).
    let x:usize=x as usize;
    if x<63{return&BASE62.split_at(x).0}// Might replace that by a match with miscilinious bases (e.g., MORSE) added.
    else if x<95{return&BASE94.split_at(x).0}
    &BASE256.split_at(x).0
}

pub fn convert<'a>(value:&[u8],source_base:&[u8],target_base:&[u8])->&'a[u8]{
    let source_base_len:usize=source_base.len();
    let target_base_len:usize=target_base.len();

    if value.is_empty()||source_base_len==0||target_base_len==0{return&[]}

    b"hello"
}

pub const fn trim_zeros<'a>(value:&'a[u8],base:&[u8])->&'a[u8]{
    if value.is_empty()||base.len()<2{return value}

    let zero:u8=base[0];
    let mut index_counter:usize=0;

    while index_counter<value.len(){
        if value[index_counter]!=zero{return value.split_at(index_counter).1}
        index_counter+=1
    }
    value.split_at(1).0// Returns a slice containing a single zero, if the collection contained zeros only.
}

fn main(){
    println!("{:?}",convert(b"10",&BASE62,&DECIMAL))
}