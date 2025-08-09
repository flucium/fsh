pub mod expression;
pub mod statement;

/// A trait for converting an AST into JSON format.
///
/// Implementors of this trait define how their AST representation
/// should be serialized into a JSON string.
pub trait FshAst {
    /// Converts the AST into a JSON string.
    ///
    /// # Arguments
    /// - `is_pretty`: If `true`, formats the JSON output with indentation
    ///   and line breaks for readability; if `false`, produces a compact single-line JSON string.
    ///
    /// # Returns
    /// A `String` containing the JSON representation of the AST.
    fn to_json(&self, is_pretty: bool) -> String;
}
