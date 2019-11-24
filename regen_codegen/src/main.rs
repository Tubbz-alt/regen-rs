extern crate pest;
#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "dsl.pest"] // relative to src
struct DSLParser;

fn main() {
//    DSLParser::parse(Rule::module, "test.dsl")
}