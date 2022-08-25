use error_chain::error_chain;
use select::document::Document;
use select::predicate::Name;
use regex::Regex;


error_chain! {
      foreign_links {
          ReqError(reqwest::Error);
          IoError(std::io::Error);
      }
}

#[tokio::main]
pub async fn main() -> Result<Vec<String>> {
  let res = reqwest::get("https://newsdig.tbs.co.jp/list/latest")
    .await?
    .text()
    .await?;

    let mut cnt: Vec<String> = vec![];

  Document::from(res.as_str())
    .find(Name("a"))
    .filter_map(|n| n.attr("href"))
    .for_each(|x| cnt.push(x.to_string()));
  Ok(cnt)

}





