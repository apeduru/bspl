use std::collections::HashMap;
use lexer::Tokens;
use error::ParsingError;

pub type Operators = HashMap<&'static str, Operator>;
pub type LookupTable = HashMap<String, String>;

// Refs: stackoverflow.com/questions/930486/what-is-associativity-of-operators-and-why-is-it-important
pub enum Associativity {
    LeftToRight,
    RightToLeft,
}

pub struct Operator(u8, Associativity);

/// Operator Precedence
/// | Precedence | Operator | Symbol | Associativity |
/// |:----------:|:--------:|:------:|---------------|
/// |1           |Brackets  |()      |Left-to-Right  |
/// |2           |NOT       |~       |Right-to-Left  |
/// |3           |Shift     |<< >>   |Left-to-Right  |
/// |4           |Equality  |==      |Left-to-Right  |
/// |5           |AND       |&       |Left-to-Right  |
/// |6           |XOR       |^       |Left-to-Right  |
/// |7           |OR        ||       |Left-to-Right  |
/// |8           |Assignment|=       |Right-to-Left  |
impl Operator {
    pub fn new(precedence: u8, associativity: Associativity) -> Operator {
        Operator(precedence, associativity)
    }
}

pub struct Parser {
    operators: Operators,
    lookup_table: LookupTable,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            operators: Operators::new(),
            lookup_table: LookupTable::new(),
        }
    }

    pub fn init(&mut self) {
        self.operators.insert("(", Operator::new(1, Associativity::LeftToRight));
        self.operators.insert(")", Operator::new(1, Associativity::LeftToRight));
        self.operators.insert("~", Operator::new(2, Associativity::RightToLeft));
        self.operators.insert(">>", Operator::new(3, Associativity::LeftToRight));
        self.operators.insert("<<", Operator::new(3, Associativity::LeftToRight));
        self.operators.insert("==", Operator::new(4, Associativity::LeftToRight));
        self.operators.insert("&", Operator::new(5, Associativity::LeftToRight));
        self.operators.insert("^", Operator::new(6, Associativity::LeftToRight));
        self.operators.insert("|", Operator::new(7, Associativity::LeftToRight));
        self.operators.insert("=", Operator::new(8, Associativity::RightToLeft));
    }

    // fn is_keyword(&mut self, err_position: usize, radix: &String) -> Result<&Tokens> {
    //     for keyword in KEYWORDS {
    //         if keyword.to_string() == radix.to_lowercase() {
    //             return Err(LexerError::KeywordError(err_position));
    //         }
    //     }
    //     Ok((&self.tokens))
    // }

    pub fn parse(&mut self, tokens: Tokens) -> Result<Tokens, ParsingError> {
        // let mut operator_stack = Tokens::with_capacity(10);
        Ok((tokens))
    }
}
