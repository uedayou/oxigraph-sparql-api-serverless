mod config;

use oxigraph::RocksDbStore as Store;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

fn main() -> Result<(), Box<dyn Error>> {
  let args: Vec<String> = std::env::args().collect();
  let filename = if args.get(1) != None { &args[1] } else { config::FILENAME };
  println!("{}", &filename);
  let dbname = config::DBNAME;
  let mut store = Store::open(&dbname)?;
  let reader = BufReader::new(File::open(&filename)?);
  store.load_graph(reader, config::FORMAT, None, None)?;
  drop(store);

  store = Store::open(&dbname)?;
  let mut results = Vec::new();
  let query = "SELECT (count(?s) as ?count) WHERE { ?s ?p ?o } limit 10";
  store.query(
    query
  )?.write(
    &mut results,
    oxigraph::sparql::QueryResultsFormat::Json
  )?;
  println!("{}", String::from_utf8(results).unwrap());

  Ok(())
}