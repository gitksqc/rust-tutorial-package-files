use std::{
    env, 
    fs::{self, DirEntry}, 
    io::{self, Error, ErrorKind}, 
    path::{PathBuf}, 
    collections::HashMap, 
    time::{UNIX_EPOCH},
};
use chrono::{NaiveDateTime, DateTime, Utc, Local};  
use chrono::Datelike;

// 配置
pub struct Config {
    pub dir_path: String,
    pub record_path: String,
    pub record: bool,
}

 
impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("请指定要归类的目录");
        }

        let dir_path = args[1].clone();
        let record_path: String = String::from("./");
        let record = env::var("RECORD").is_ok();
        
        Ok(Config { 
            dir_path: dir_path, 
            record_path: record_path,
            record: record })
    }
}

// 任务执行
fn run(config: Config) -> Result<(), Error> {
    if config.record {
        // package_files_record(&config.dir_path, &config.record);
    } else {
        _ = package_files(&config.dir_path);
    }

    println!("处理完成！");
    Ok(())
}

fn package_files(dir_path: &str) -> Result<(), Error> {
    let mut current_dir = PathBuf::new();
    current_dir.push(dir_path);
    let current_dir_copy = current_dir.clone();

    let result = fs::read_dir(current_dir);
    if let Err(ref e) = result {  
        println!("Error reading directory: {}", e);  
        return Ok(());  
    };
    
    // 文件路径和修改时间
    let mut dirname = HashMap::new();
    // 遍历所有目录
    for file in result?.into_iter() {  
        let f = file.unwrap().path();
        if f.is_dir() {
            // println!("目录: {:?}", f);
            // println!("目录: {:?}", f.file_name());
            // println!("目录: {:?}", f.parent());
            
            let name = String::from(f.file_name().unwrap().to_str().unwrap().to_string());
            dirname.insert(name, 1);
            dirname.insert(String::from("src"), 2);
        }
    }
    // for (k, v) in dirname.iter() {
    //     println!("{k:?} - {v:?}");
    //     println!("{current_dir_copy:?}/{k:?}");
    // }

    let result_copyt = fs::read_dir(&current_dir_copy);
    for file in result_copyt?.into_iter() {
        let file_path = file.unwrap().path();  
        if file_path.is_file() {
            // files.push(file_path.to_str().unwrap().to_string());
            let metadata = fs::metadata(&file_path)?;
            // println!("{:?}", metadata.modified()?.elapsed().unwrap().as_secs());
            let period = metadata.modified().unwrap().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
            // println!("period：{:?}", period);
            
            let native_datetime = NaiveDateTime::from_timestamp_millis(period).unwrap();
            // println!("nnnn: {native_datetime:?}");
            // 月份 
            let month;
            if native_datetime.month() > 9 {
                month = native_datetime.month().to_string();
            } else {
                month = String::from("0") + (&native_datetime.month().to_string());
            }
            
            let dest_dir = native_datetime.year().to_string() + "-" + &month;
            println!("dest dir: {dest_dir:?}");
            if !dirname.contains_key(&dest_dir) {
                fs::create_dir(current_dir_copy.join(&dest_dir)).unwrap();
            }

            let src_file = file_path.to_str().unwrap().to_string();
            // println!("from: {:?}", src_file);
            let dest_filename = file_path.file_name().unwrap();
            let dest_file = current_dir_copy.join(dest_dir).join(dest_filename);
            // println!("to: {:?}", dest_file);

            // 移动文件
            match fs::rename(src_file, dest_file) {
                Ok(()) => println!("移动成功"),
                Err(e) => println!("失败: {:?}", e),
            }
        }     
    }  
    
    Ok(())
}

//
fn main() ->Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    // let s = String::from_utf8(&args).unwrap();
    // let config = match Config::build(&args) {
    //     Config(c) => c,
    //     String(&s) => s,
    // };
    let config = Config::build(&args);//.unwrap();
    match config {
            Ok(c) => {
                println!("整理目录为：{:?}", c.dir_path);
                return run(c);
            },
            Err(e) => {
                println!("{:?}", e);
                return Ok(());
            },
    };
}