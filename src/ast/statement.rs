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

/// Represents an assignment operation.
///
/// An `Assignment` consists of an identifier and a value.
#[derive(Debug, PartialEq, Serialize)]
pub struct Assignment {
    identifier: Expression,
    value: Expression,
}

impl Assignment {
    /// Creates a new `Assignment`.
    ///
    /// # Arguments
    /// - `identifier` - The identifier being assigned to.
    /// - `value` - The value being assigned.
    ///
    /// # Returns
    /// A new `Assignment` instance.
    pub fn new(identifier: Expression, value: Expression) -> Self {
        Self { identifier, value }
    }

    /// Returns a reference to the identifier.
    pub fn identifier(&self) -> &Expression {
        &self.identifier
    }

    /// Returns a reference to the value.
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

/// Represents redirection operators (`>` and `<`).
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum RedirectOperator {
    /// <
    LessThan,

    /// >
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

/// Represents a redirection operation.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Redirect {
    operator: RedirectOperator,
    left: Expression,
    right: Expression,
}

impl Redirect {
    /// Creates a new `Redirect`.
    ///
    /// # Arguments
    /// - `operator` - The redirection operator (`>` or `<`).
    /// - `left` - The left-hand expression (typically a file descriptor).
    /// - `right` - The right-hand expression (typically a filename or another descriptor).
    ///
    /// # Returns
    /// A new `Redirect` instance.
    pub fn new(operator: RedirectOperator, left: Expression, right: Expression) -> Self {
        Self {
            operator,
            left,
            right,
        }
    }

    /// Returns a reference to the redirection operator.
    pub fn operator(&self) -> &RedirectOperator {
        &self.operator
    }

    /// Returns a reference to the left-hand expression.
    pub fn left(&self) -> &Expression {
        &self.left
    }

    /// Returns a reference to the right-hand expression.
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

/// Represents a command.
#[derive(Debug, PartialEq, Serialize)]
pub struct Command {
    name: Expression,
    arguments: Vec<Expression>,
    redirects: Vec<Redirect>,
    is_background: Expression,
}

impl Command {
    /// Creates a new `Command`.
    ///
    /// # Arguments
    /// - `name` - The command name.
    /// - `arguments` - A list of arguments.
    /// - `redirects` - A list of redirections.
    /// - `is_background` - A flag indicating if the command runs in the background.
    ///
    /// # Returns
    /// A new `Command` instance.
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

    /// Returns a reference to the command name.
    pub fn name(&self) -> &Expression {
        &self.name
    }

    /// Returns a reference to the list of arguments.
    pub fn arguments(&self) -> &Vec<Expression> {
        &self.arguments
    }

    /// Returns a reference to the list of redirections.
    pub fn redirects(&self) -> &Vec<Redirect> {
        &self.redirects
    }

    /// Returns a reference to the background execution flag.
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

/// Represents a commands connected by pipes (`|`).
///
/// Internally, this is implemented as a `VecDeque<Command>` to allow
/// efficient push and pop operations at both ends.
#[derive(Debug, PartialEq, Serialize)]
pub struct Pipe(VecDeque<Command>);

impl Pipe {
    /// Creates a new `Pipe`.
    ///
    /// # Returns
    /// A new `Pipe` instance.
    pub fn new() -> Self {
        Self(VecDeque::new())
    }

    /// Appends a command to the end of the pipe.
    ///
    /// # Arguments
    /// - `command`: The `Command` to append.
    pub fn push_back(&mut self, command: Command) {
        self.0.push_back(command);
    }

    /// Removes and returns the command at the front of the pipe.
    ///
    /// # Returns
    /// - `Some(Command)` if a command was present.
    /// - `None` if the pipe is empty.
    pub fn pop_front(&mut self) -> Option<Command> {
        self.0.pop_front()
    }

    /// Checks whether the pipe contains no commands.
    ///
    /// # Returns
    /// `true` if the pipe is empty, `false` otherwise.
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
