use std::collections::HashMap;
use crate::lexer::token::Keyword;

pub fn build_keyword_map() -> HashMap<&'static str, Keyword> {
let mut m = HashMap::new();

m.insert("FEATURE", Keyword::Structural);
m.insert("FROM", Keyword::Structural);
m.insert("JOIN", Keyword::Structural);
m.insert("ON", Keyword::Structural);

m.insert("WINDOW", Keyword::Property);
m.insert("GROUP_BY", Keyword::Property);
m.insert("AGGREGATION", Keyword::Property);
m.insert("FILTER", Keyword::Property);

m.insert("SUM", Keyword::Aggregation);
m.insert("COUNT", Keyword::Aggregation);
m.insert("AVG", Keyword::Aggregation);
m.insert("MIN", Keyword::Aggregation);
m.insert("MAX", Keyword::Aggregation);

m.insert("AND", Keyword::Connector);
m.insert("OR", Keyword::Connector);
m.insert("NOT", Keyword::Connector);

m

}
