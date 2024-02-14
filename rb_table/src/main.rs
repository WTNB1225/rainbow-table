extern crate rb_table;
use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short = 'l', long, default_value_t = 1000)]
    chain_len: usize,
    #[arg(short = 'n', long, default_value_t = 1000)]
    chain_num: usize,
    #[arg(short = 'p', long)]
    pwd_len: usize,
    #[arg(short = 'c', long, default_value = "abcdefghijklmnopqrstuvwxyz0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ")]
    pwd_charset: String,
    #[arg(short = 'd', long)]
    hash_to_decode: String,
    #[arg(short = 'a', long, default_value = "md5")]
    hash_algo: String,
}

fn main() {
    let args = Args::parse();
    rb_table::run(args.chain_len, args.chain_num, args.pwd_len, args.pwd_charset, args.hash_to_decode, args.hash_algo);
}

//fn main() {
//    rb_table::run(
//        1000,
//        1000,
//        1, 
// 
//        "abcdefghijklmnopqrstuvwxyz0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string(),
//        "0cc175b9c0f1b6a831c399e269772661".to_string(),
//    )
//}
