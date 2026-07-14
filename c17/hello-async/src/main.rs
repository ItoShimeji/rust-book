use ::trpl::Html;

async fn page_title(url: &str) -> Option<String> {
    // ここでは Response 構造体が作成できるまで待機する
    // そのため、Status Code などは同期的に取得できるが、body などは取得中
    let response = trpl::get(url).await;
    // ここで body の取得を終わらせる
    // body の取得を2段階に分けることで、body のストリーミング処理や、status code による早期 return などのメリットがある
    // もし、get と text の間にループを設置し、cpu 時間を消費すると、通信自体が終了しているもしくは、
    // バッファが満杯になり TCP 通信が抑制されるなどとなる
    let response_text = response.text().await;
    Html::parse(&response_text)
        .select_first("title")
        .map(|title| title.inner_html())
}

// main を async にすると、スタートポイントである main を管理する存在が必要になる
// そのため、ここは通常の関数ではないといけない
// macro によって、async fn main() {} を見かけ上達成するランタイムもある
fn main() {
    let args: Vec<String> = std::env::args().collect();

    trpl::block_on(async {
        let url = &args[1];
        match page_title(url).await {
            Some(title) => println!("The title from {url} was {title}"),
            None => println!("{url} had no title"),
        }
    })
}
