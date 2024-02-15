extern crate uint;
use crypto::md5::Md5;
use crypto::sha2::Sha256;
use crypto::digest::Digest;
use rayon::prelude::*;
use uint::construct_uint;
use std::fs::File;
use std::io::prelude::*;

construct_uint! {
	pub struct U256(4);
}

pub struct RainbowTable {
    table: Vec<Vec<String>>,
    chain_len: usize,
    chain_num: usize,
    pwd_len: usize,
    pwd_charset: Vec<char>,
    hash_algo: String, // Change this line
}

impl RainbowTable {
    pub fn new(chain_len: usize, chain_num: usize, pwd_len: usize, pwd_charset: Vec<char>, hash_algo: String) -> RainbowTable { // Change this line
        RainbowTable {
            table: Vec::new(),
            chain_len: chain_len,
            chain_num: chain_num,
            pwd_len: pwd_len,
            pwd_charset: pwd_charset,
            hash_algo: hash_algo, // Change this line
        }
    }

    pub fn hash(&self, pwd: &str) -> String {
        match self.hash_algo.as_str() { // Change this line
            "md5" => {
                let mut md5 = Md5::new();
                md5.input_str(pwd);
                md5.result_str()
            },
            "sha256" => {
                let mut sha256 = Sha256::new();
                sha256.input_str(pwd);
                sha256.result_str()
            },
            _ => {
                let mut md5 = Md5::new();
                md5.input_str(pwd);
                md5.result_str()
            }
        }
    }

    //fn reduction(&self, hash: &str, i: usize) -> String {
    //  let mut p = String::new();
    //  let mut index: U256 = U256::from_str_radix(&hash, 16).unwrap() + U256::from(i);
    //  while index > U256::from(0) {
    //      p.push(self.pwd_charset[(index % U256::from(self.pwd_charset.len())).as_u64() as usize]);
    //      index = index / U256::from(self.pwd_charset.len());
    //  }
    //  p.chars().take(self.pwd_len).collect()
    //}

    fn reduction(&self, hash: &str, i: usize) -> String {
        let mut p = String::new();
        let mut index: U256 = U256::from_str_radix(&hash, 16).unwrap() + U256::from(i);
        while index > U256::from(0) {
            let charset_len = U256::from(self.pwd_charset.len());
            let pos = (index + U256::from(i)) % charset_len;
            p.push(self.pwd_charset[pos.low_u64() as usize]);
            index = index / charset_len;
        }
        p.chars().take(self.pwd_len).collect()
    }
    
    pub fn chain(&self, p: &str) -> Vec<String> { //p: start
        let mut ch = Vec::new();
        let mut p = p.to_string();
        for i in 0..self.chain_len {
            ch.push(p.clone());
            ch.push(self.hash(&p));
            p = self.reduction(&ch[ch.len()-1], i);
        }
        ch.push(p);
        ch
    }

    fn match_tail(&self, p:&str) -> Vec<String> {
        for chain in &self.table {
            if chain[1] == p {
                println!("match_tail: {:?}", chain.clone());
                return chain.clone();
            }
        }
        Vec::new()
    }

    pub fn rainbow_table(&mut self) {
        let chains: Vec<Vec<String>> = (0..self.chain_num).into_par_iter().map(|i| {
            let p = self.reduction(&self.hash(&i.to_string()), i);
            let chain = self.chain(p.as_str());
            let chain_without_tail: Vec<String> = chain.clone().into_iter().take(self.chain_len).collect();
            if chain_without_tail.contains(&chain[chain.len() - 1]) {
                println!("collision");
            }
            //println!("chain: {:?}", chain.clone());
            vec![chain[0].clone(), chain[chain.len() - 1].clone()]
            }).collect();

        
        self.table = chains;
    } 

    pub fn decode(&self, t: &str) {
        if self.table.is_empty() {
            return;
        }
        let mut matched_chain = Vec::new();
        for i in (0..self.chain_len).rev() {
            //println!("i: {}", i);
            let mut p = self.reduction(t, i);
            for j in 0..self.chain_len - 1 - i {
                //println!("j: {}", j);
                let h = self.hash(&p);
                p = self.reduction(&h, i + j + 1);
            }
            matched_chain = self.match_tail(&p);
            if !matched_chain.is_empty() {
                break;
            }
        }
        if matched_chain.is_empty() {
            return;
        }
        matched_chain = self.chain(&matched_chain[0]);
        let ch_bytes = matched_chain.join("\n").into_bytes();

        // Create a new file and write the byte array to it
        let mut file = File::create("table.txt").expect("Unable to create file");
        file.write_all(&ch_bytes).expect("Unable to write file");
        
        // Flush the file to ensure all data is written
        file.flush().expect("Unable to flush file");

        println!("{:?}", matched_chain.iter().position(|x| x == t).and_then(|index| index.checked_sub(1)).map(|index| matched_chain[index].clone()).unwrap_or_else(|| String::from("Not found")));
    }
}

pub fn run(chain_len: usize, chain_num: usize, pwd_len: usize, pwd_charset: String, hash_to_decode: String, hash_algo: String) {
    let pwd_charset: Vec<char> = pwd_charset.chars().collect();
    let hash_to_decode: &str = hash_to_decode.as_str();
    let mut rt = RainbowTable::new(chain_len, chain_num, pwd_len, pwd_charset, hash_algo);
    rt.rainbow_table();
    //println!("hash_to_decode: {:?}", rt.chain("jfJe"));
    rt.decode(hash_to_decode);
}