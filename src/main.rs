use std::{env, process};
use s_file::{Config, run};

fn main() {
    // let args: Vec<String> = env::args().collect();

    // env::args() 返回一个迭代器 传入迭代器 通过迭代器获取输入参数
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });

    println!("Searching for [{}] In file [{}]", config.query,config.file_path);

    if let Err(e) = run(config) {
        eprintln!("application error {e}");
        process::exit(2);
    }

}