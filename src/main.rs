mod config;

use lambda_http::{
  handler,
  lambda_runtime::{self, Context},
  IntoResponse, Request, RequestExt, Response,
};

use oxigraph::sparql::QueryResultsFormat;
use oxigraph::RocksDbStore as Store;

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
  lambda_runtime::run(handler(func)).await?;
  Ok(())
}

async fn func(event: Request, _: Context) -> Result<impl IntoResponse, Error> {
  Ok(match event.query_string_parameters().get("query") {
    Some(query) => {
      let format = event.query_string_parameters().get("format").unwrap_or_else(|| "json").to_string();
      let json = sparql(&query, &format).unwrap_or("ERROR".to_string());
      Response::builder()
        .status(200)
        .header("Content-Type", 
          match format.as_ref() {
            "xml" => "application/xml",
            "json" => "application/json",
            _ => "text/plain",
          },
        )
        .header("Access-Control-Allow-Methods", "OPTIONS,POST,GET")
        .header("Access-Control-Allow-Credential", "true")
        .header("Access-Control-Allow-Origin", "*")
        .body(json)
        .expect("failed to render response")
    },
    _ => Response::builder()
        .status(400)
        .body("error".into())
        .expect("failed to render response"),
  })
}

fn sparql(query: &str, format: &str) -> Result<String, Error> {
  let dbname = config::DBNAME;
  let store = Store::open_readonly(&dbname)?;
  let format_type = match format.as_ref() {
    "xml" => QueryResultsFormat::Xml,
    _ => QueryResultsFormat::Json,
  };
  let _query = percent_encoding::percent_decode_str(query).decode_utf8().unwrap();
  let mut results = Vec::new();
  store.query(
    _query.as_ref()
  )?.write(&mut results, format_type)?;
  let json = String::from_utf8(results).unwrap();
  Ok(json)
}
