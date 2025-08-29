pub mod expression;
pub mod statement;

pub trait FshAst {
    fn to_json(&self, is_pretty: bool) -> String;
}
