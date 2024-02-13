use crypto::md5::Md5;
use crypto::digest::Digest;

struct RainbowTable {
    table: Vec<Vec<String>>,
    chain_len: usize,
    chain_num: usize,
    pwd_len: usize,
    pwd_charset: Vec<char>,
}

impl RainbowTable {
    fn new(chain_len: usize, chain_num: usize, pwd_len: usize, pwd_charset: Vec<char>) -> RainbowTable {
        RainbowTable {
            table: Vec::new(),
            chain_len: chain_len,
            chain_num: chain_num,
            pwd_len: pwd_len,
            pwd_charset: pwd_charset,
        }
    }

    fn hash(&self, pwd: &str) -> String {
        let mut md5 = Md5::new();
        md5.input_str(pwd);
        md5.result_str()
    }

    fn reduction(&self, hash: &str, i: usize) -> String {
        let mut p = String::new();
        let mut index: u128 = u128::from_str_radix(&hash, 16).unwrap() + i as u128;
        while index > 0 {
            p.push(self.pwd_charset[(index % self.pwd_charset.len() as u128) as usize]);
            index = index / self.pwd_charset.len() as u128;
        }
        p.chars().take(self.pwd_len).collect()
    }

    fn chain(&self, p: &str) -> Vec<String> { //p: start
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
                //println!("Matched: {}", chain[0]);
                return chain.clone();
            }
        }
        Vec::new()
    }

    fn rainbow_table(&mut self) {
        for i in 0..self.chain_num {
            let p = self.reduction(&self.hash(&i.to_string()), i);
            let chain = self.chain(p.as_str());
            self.table.push(vec![chain[0].clone(), chain[chain.len() - 1].clone()]);
        }
    }

    fn decode(&self, t: &str) {
        if self.table.is_empty() {
            return;
        }
        let mut matched_chain = Vec::new();
        for i in (0..self.chain_len).rev() {
            let mut p = self.reduction(t, i);
            for j in 0..self.chain_len - 1 - i {
                let h = self.hash(&p);
                p = self.reduction(&h, i + j + 1);
            }
            matched_chain = self.match_tail(&p);
            if !matched_chain.is_empty() {
                //println!("Matched chain1: {:?}", matched_chain);
                break;
            }
        }
        //println!("Matched chain2: {:?}", matched_chain);
        if matched_chain.is_empty() {
            return;
        }
        matched_chain = self.chain(&matched_chain[0]);
        //println!("Matched chain3: {:?}", matched_chain);
        println!("{:?}", matched_chain.iter().position(|x| x == t).and_then(|index| index.checked_sub(1)).map(|index| matched_chain[index].clone()));
    }
}

fn main() {
    let mut rt = RainbowTable::new(150, 150, 1, vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p','q','r','s','t','u','v','w','x','y','z','0','1','2','3','4','5','6','7','8','9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P','Q','R','S','T','U','V','W','X','Y','Z']);
    rt.rainbow_table();
    //println!("{:?}", rt.chain(&rt.hash("a")));
    //rt.decode(&rt.hash(""));
    rt.decode("0cc175b9c0f1b6a831c399e269772661");
}