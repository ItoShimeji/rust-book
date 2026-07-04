use minigrep::search;
use minigrep::search_case_insensitive;
use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // Ok 時の値はないため、上の unwrap_or_else の方法ではなく、if let を使う
    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

// 関連する値はひとまとめにしよう
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            // ここで文字列リテラル自体を返し、文字列はコンパイル済みバイナリに埋め込まれる。
            // そのため、プログラム中常に存在する static な lifetime になる
            return Err("not enough arguments");
        }

        // index 0 は実行ファイルのパス
        let query = args[1].clone();
        let file_path = args[2].clone();

        // 環境変数の取得
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

// Box<dyn Error> : ここでは Error trait を実装した値を返すと思えば良い
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // ? 演算子により、Err のときはその値を早期 return する
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}
