use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::env;

type Err = Box<dyn Error + Send + Sync>;

// TODO 文中の,をエスケープする
// TODO 先に読み込み先のCSVの先頭をみて、Headerに要素があれば、その要素を優先して、順番通りに表示するようにする

fn main() -> Result<(), Err> {

    // コマンドの引数を取得する
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        panic!("引数は2つです");
    }

    // 読み込み元
    let read_file_name = &args[1];
    let read_file = File::open(read_file_name)?;
    let reader = BufReader::new(read_file);

    // 出力先
    let write_file_name = &args[2];
    let mut write_file = File::create(write_file_name)?;

    // カラム
    let mut calumn_set: HashSet<String> = HashSet::new();

    // DataのVec
    let mut data_list: Vec<HashMap<String, Value>> = Vec::new();

    // 読み込み
    for (_, line) in reader.lines().enumerate() {
        let line = line?;

        let m: HashMap<String, Value> = serde_json::from_str(&line)?;

        for (k, _) in &m {
            calumn_set.insert(k.to_string());
        }

        data_list.push(m);
    }


    // 書き込み
    // header書き込み
    for calumn in &calumn_set {
        write_file.write_all(format!("{},",calumn).as_bytes())?;
    }
    write_file.write_all("\n".as_bytes())?;

    // データの書き込み
    for data_map in data_list {

        for calumn in &calumn_set {
            let data = match data_map.get(calumn) {
                Some(data) => {
                    format!("{},", data)
                },
                None => ",".to_string(),
            };

            write_file.write_all(format!("{}", data).as_bytes())?;
        }

        // 改行
        write_file.write_all("\n".as_bytes())?;
    }

    Ok(())
}
