use super::{expression::*, *};

#[derive(Debug, PartialEq, Serialize)]
pub enum Statement {
    Assignment(Assignment),
    Command(Command),
}

impl FshAst for Statement {
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    fn to_json_pretty(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Assignment {
    identifier: Expression,
    value: Expression,
}

impl Assignment {
    pub fn new(identifier: Expression, value: Expression) -> Self {
        Self {
            identifier,
            value,
        }
    }

    pub fn identifier(&self) -> &Expression {
        &self.identifier
    }

    pub fn value(&self) -> &Expression {
        &self.value
    }
}

impl FshAst for Assignment {
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    fn to_json_pretty(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }
}

#[derive(Debug, Clone,PartialEq, Serialize)]
pub struct Redirect {
    operator: RedirectOperator,
    left: Expression,
    right: Expression,
}

impl Redirect {
    pub fn new(operator: RedirectOperator, left: Expression, right: Expression) -> Self {
        Self {
            operator,
            left,
            right,
        }
    }

    pub fn operator(&self) -> &RedirectOperator {
        &self.operator
    }

    pub fn left(&self) -> &Expression {
        &self.left
    }

    pub fn right(&self) -> &Expression {
        &self.right
    }
}

impl FshAst for Redirect {
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    fn to_json_pretty(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }
}

#[derive(Debug,Clone, PartialEq, Serialize)]
pub enum RedirectOperator {
    GreaterThan,
    // GreaterThanGreaterThan,
    LessThan,
    // LessThanLessThan,
}

impl FshAst for RedirectOperator {
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    fn to_json_pretty(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Command {
    name: Expression,
    arguments: Vec<Expression>,
    redirects: Vec<Redirect>,
    is_background: Expression,
}

impl Command {
    pub fn new(
        name: Expression,
        arguments: Vec<Expression>,
        redirects: Vec<Redirect>,
        is_background: Expression,
    ) -> Self {
        Self {
            name,
            arguments,
            redirects,
            is_background,
        }
    }

    pub fn name(&self) -> &Expression {
        &self.name
    }

    pub fn arguments(&self) -> &[Expression] {
        &self.arguments
    }

    pub fn redirects(&self) -> &[Redirect] {
        &self.redirects
    }

    pub fn is_background(&self) -> &Expression {
        &self.is_background
    }
}

impl FshAst for Command {
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    fn to_json_pretty(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }
}
