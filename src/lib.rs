mod alphabet;
pub use crate::alphabet::*;

pub fn str_to_base(str_alphabet:&str)->&[u8]{
    str_alphabet.as_bytes()
}

pub fn base_to_str(alphabet:&[u8])->&str{
    core::str::from_utf8(alphabet).unwrap()
}

pub const fn base<'a>(x:u8)->&'a[u8]{// Limited to u8 and redeclared to usize, because I want to make bases over 255 impossible. Note that base(256) is not possible (but possible via constant BASE256).
    let x:usize=x as usize;
    if x<63{return&BASE62.split_at(x).0}// Might replace that by a match with miscilinious bases (e.g., MORSE) added.
    else if x<96{return&BASE95.split_at(x).0}
    &BASE256.split_at(x).0
}

pub fn convert<'a>(value:&[u8],source_base:&[u8],target_base:&[u8])->Vec<u8>{// Might replace Vec by a no_std alloc::vec::Vec or by another solution.
    let source_base_len:usize=source_base.len();
    let target_base_len:usize=target_base.len();

    if value.is_empty()||source_base_len==0||target_base_len==0{return Vec::new()}
    
    let value:&[u8]=trim_zeros(value,source_base);
    
    let mut source_digits:Vec<usize>=value
        .iter().map(|&b|{
            source_base.iter().position(|&sb|sb==b).expect("[!] INVALID DIGIT")
        })
        .collect();

    let mut target_indices:Vec<usize>=Vec::new();
    let mut start:usize=0;

    while start<source_digits.len(){
        let mut current_carry:usize=0;
        let mut new_start:usize=start;
        let mut leading_zero:bool=true;

        for ix in start..source_digits.len(){
            let working_value:usize=current_carry*source_base_len+source_digits[ix];
            let quotient_digit:usize=working_value/target_base_len;
            current_carry=working_value%target_base_len;

            source_digits[ix]=quotient_digit;

            if leading_zero{
                if quotient_digit==0{new_start+=1}
                else{leading_zero=false}
            }
        }

        start=new_start;
        target_indices.push(current_carry);
    }

    target_indices.into_iter().rev().map(|ix:usize|target_base[ix]).collect()
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