pub mod expression;
pub mod statement;

/// A trait for converting an Fsh AST (Abstract Syntax Tree) to JSON format.
///
/// This trait provides a method for serializing an AST structure into
/// JSON, with an option to format the output in a compact or pretty-printed style.
pub trait FshAst {
    /// Converts the AST into a JSON string representation.
    ///
    /// # Arguments
    ///
    /// - `is_pretty` - A boolean flag that determines the JSON format.
    ///   - If `true`, the output is pretty-printed.
    ///   - If `false`, the output is compact.
    ///
    /// # Returns
    ///
    /// A `String` containing the JSON representation of the AST.
    fn to_json(&self, is_pretty: bool) -> String;
}