use super::{expression::Expression, FshAst};
use serde::Serialize;
use std::collections::VecDeque;

#[derive(Debug, PartialEq, Serialize)]
pub enum Statement {
    Sequence(Sequence),

    Assignment(Assignment),

    Redirect(Redirect),

    Command(Command),

    Pipe(Pipe),
}

impl FshAst for Statement {
    fn to_json(&self, is_pretty: bool) -> String {
        if is_pretty {
            serde_json::to_string_pretty(self).unwrap()
        } else {
            serde_json::to_string(self).unwrap()
        }
    }
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Sequence(VecDeque<Statement>);

impl Sequence {
    pub fn new() -> Self {
        Self(VecDeque::new())
    }

    pub fn push_back(&mut self, statement: Statement) {
        self.0.push_back(statement);
    }

    pub fn pop_front(&mut self) -> Option<Statement> {
        self.0.pop_front()
    }
}

impl FshAst for Sequence {
    fn to_json(&self, is_pretty: bool) -> String {
        if is_pretty {
            serde_json::to_string_pretty(self).unwrap()
        } else {
            serde_json::to_string(self).unwrap()
        }
    }
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Assignment {
    identifier: Expression,
    value: Expression,
}

impl Assignment {
    pub fn new(identifier: Expression, value: Expression) -> Self {
        Self { identifier, value }
    }

    pub fn identifier(&self) -> &Expression {
        &self.identifier
    }

    pub fn value(&self) -> &Expression {
        &self.value
    }
}

impl FshAst for Assignment {
    fn to_json(&self, is_pretty: bool) -> String {
        if is_pretty {
            serde_json::to_string_pretty(self).unwrap()
        } else {
            serde_json::to_string(self).unwrap()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum RedirectOperator {
    LessThan,
    GreaterThan,
}

impl FshAst for RedirectOperator {
    fn to_json(&self, is_pretty: bool) -> String {
        if is_pretty {
            serde_json::to_string_pretty(self).unwrap()
        } else {
            serde_json::to_string(self).unwrap()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
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
    fn to_json(&self, is_pretty: bool) -> String {
        if is_pretty {
            serde_json::to_string_pretty(self).unwrap()
        } else {
            serde_json::to_string(self).unwrap()
        }
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

    pub fn arguments(&self) -> &Vec<Expression> {
        &self.arguments
    }

    pub fn redirects(&self) -> &Vec<Redirect> {
        &self.redirects
    }

    pub fn is_background(&self) -> &Expression {
        &self.is_background
    }
}

impl FshAst for Command {
    fn to_json(&self, is_pretty: bool) -> String {
        if is_pretty {
            serde_json::to_string_pretty(self).unwrap()
        } else {
            serde_json::to_string(self).unwrap()
        }
    }
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Pipe(VecDeque<Command>);

impl Pipe {
    pub fn new() -> Self {
        Self(VecDeque::new())
    }

    pub fn push_back(&mut self, command: Command) {
        self.0.push_back(command);
    }

    pub fn pop_front(&mut self) -> Option<Command> {
        self.0.pop_front()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl FshAst for Pipe {
    fn to_json(&self, is_pretty: bool) -> String {
        if is_pretty {
            serde_json::to_string_pretty(self).unwrap()
        } else {
            serde_json::to_string(self).unwrap()
        }
    }
}
