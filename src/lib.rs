use std::error::Error;
use std::{env, fs};

pub struct Config{
    pub query: String,
    pub file_path : String,
    pub ignore_case : bool
}

pub struct LineResult {
    pub row: usize,
    pub line: String
}

// 从输入中获取参数
impl Config{
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config,&'static str>{

        // 第一个参数是可执行程序所在路径
        let _program_path = args.next().unwrap();
        println!("program path {:?}",_program_path);

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("no arguments: query")
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("no arguments: file_path")
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config{query,file_path,ignore_case})
    }
}

// 从文件中读取内容到 String
pub fn run(config: Config) -> Result<(),Box<dyn Error>>{
    let  contents = fs::read_to_string(config.file_path)?;
    // println!("text: \n{contents}");

    let search_result_vec = search(&config.query,&contents,config.ignore_case);
    if search_result_vec.is_empty(){
        println!("search no result")
    }else{
        println!("search result => {}",search_result_vec.len());
        for line in search_result_vec.iter(){
            println!("in row {} , line content => {}",line.row,line.line);
        }
    }

    println!("search end");
    Ok(())
}

// 从文件内容中搜索指定内容
// 使用函数放回集合的好处是方便判断结果是否为空
fn search(query: &str , contents: &str , ignore_case : bool ) -> Vec<LineResult>{
    // let mut res_vec: Vec<LineResult> = vec![];

    // for (row,line) in contents.lines().enumerate(){
    //     let mut line_copy = line.to_string().clone();
    //     if ignore_case{
    //         line_copy = line_copy.to_lowercase();
    //     }
    //     if line_copy.contains(query){
    //         let line_result = LineResult{
    //             row,
    //             line: line.to_string()
    //         };
    //         res_vec.push(line_result);
    //     }
    // }

   let res_vec: Vec<LineResult> = contents
        .lines()
        .enumerate()
        .filter(|(_, line)| {
            let mut line_copy = line.to_string();
            if ignore_case {
                line_copy = line_copy.to_lowercase();
            }
            line_copy.contains(query)
        })
        .map(|(row, line)| LineResult {
            row: row+1,
            line: line.to_string(),
        })
        .collect();

    res_vec
}