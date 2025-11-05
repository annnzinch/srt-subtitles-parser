use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct SrtSubtitleParser;

pub fn parse_index(input: &str) -> Result<u32, pest::error::Error<Rule>> {
    let pairs = SrtSubtitleParser::parse(Rule::index, input)?;
    
    let index_pair = pairs.into_iter().next().unwrap();

    let index_value = index_pair.as_str().parse::<u32>().unwrap();
    
    Ok(index_value)
}