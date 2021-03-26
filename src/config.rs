use oxigraph::io::GraphFormat;

#[allow(dead_code)]
pub const FILENAME: &str = "./rdf/dump.ttl";
//pub const FILENAME: &str = "./rdf/dump.xml";
//pub const FILENAME: &str = "./rdf/dump.nt";
#[allow(dead_code)]
pub const FORMAT: GraphFormat = GraphFormat::Turtle;
//pub const FORMAT: GraphFormat = GraphFormat::RdfXml;
//pub const FORMAT: GraphFormat = GraphFormat::NTriples;

pub const DBNAME: &str = "sparql.db";
