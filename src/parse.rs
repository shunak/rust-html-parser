// use clap::Parser;
use error_chain::error_chain;
use select::document::Document;
use select::predicate::Name;


error_chain! {
      foreign_links {
          ReqError(reqwest::Error);
          IoError(std::io::Error);
      }
}

#[tokio::main]
pub async fn main() -> Result<Vec<String>> {
// pub async fn main() -> Result<()> {
  let res = reqwest::get("https://newsdig.tbs.co.jp/list/latest")
    .await?
    .text()
    .await?;

    let mut cnt: Vec<String> = vec![];

  Document::from(res.as_str())
    .find(Name("a"))
    .filter_map(|n| n.attr("href"))
    .for_each(|x| cnt.push(x.to_string()));
    // .for_each(|x| println!("{}", x));
    //
    //
    // println!("{:?}", cnt);
    // Result<Vec<String>>
  Ok(cnt)
  // Ok(())
    // return cnt;
    //
    // Some(cnt) => {
    //     Ok(cnt)
    // }
}





