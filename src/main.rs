use std::env;

#[macro_use]
extern crate serde_derive;

use ferris_says::say;
use rand::Rng;
use std::fmt::Error;
use std::fs::File;
use std::io::{stdout, BufReader, BufWriter};
use std::path::Path;

const PATH: &str = "content.json";
const DEFAULT_CONTENT: &str = "Hello Rustaceans!";
const CONTENT_LIST: &[&str] = &[
    "休对故人思故国，且将新火试新茶。诗酒趁年华。",
    "我见青山多妩媚，料青山见我应如是。",
    "黄沙百战穿金甲，不破楼兰终不还。",
    "宁可枝头抱香死，何曾吹落北风中。",
    "纸上得来终觉浅，绝知此事要躬行。",
    "男儿何不带吴钩，收取关山五十州。",
    "时人不识凌云木，直待凌云始道高。",
    "粗缯大布裹生涯，腹有诗书气自华。",
    "大鹏一日同风起，扶摇直上九万里。",
    "不经一番寒彻骨，怎得梅花扑鼻香。",
    "苟利国家生死以，岂因祸福避趋之。",
    "长风破浪会有时，直挂云帆济沧海。",
    "小文哥，针不戳啊～",
];

#[derive(Debug, Serialize, Deserialize)]
struct Content {
    content: String,
}

fn main() {
    let args: Vec<String> = env::args().filter(|s| !s.contains("ferris-say")).collect();
    let mut input = String::new();
    let content = get_from_file();
    match content {
        None => {
            for arg in args {
                input = input + &*arg + &*String::from(" ");
            }
        }
        Some(item) => input = item,
    }
    let width = 24;
    let mut writer = BufWriter::new(stdout());
    if !input.is_empty() {
        say(input.as_bytes(), width, &mut writer).unwrap();
    } else if let Some(content) = get_from_file() {
        say(content.as_bytes(), width, &mut writer).unwrap();
    } else {
        if let Some(content) = get_from_const() {
            say(content.as_ref(), width, &mut writer).unwrap();
        } else {
            say(DEFAULT_CONTENT.as_ref(), width, &mut writer).unwrap();
        }
    }
}

fn read_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<Content>, Box<Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path).expect("File open error");
    let reader = BufReader::new(file);
    // Read the JSON contents of the file as an instance of `Vec<Content>`.
    let u = serde_json::from_reader(reader).expect("Json error");
    // Return the `Vec<Content>`.
    Ok(u)
}

fn get_from_file() -> Option<String> {
    if !Path::new(PATH).exists() {
        return Option::None;
    }
    let content = read_from_file(PATH);
    let vec_content = content.expect("JSON ERROR");
    let rand_index = rand::thread_rng().gen_range(0..(vec_content.len() - 1));
    let out = vec_content.get(rand_index);
    out.map(|out| out.content.clone())
}

fn get_from_const() -> Option<String> {
    let rand_index = rand::thread_rng().gen_range(0..(CONTENT_LIST.len() - 1));
    let out = CONTENT_LIST.get(rand_index);
    out.map(|out| out.to_string())
}
