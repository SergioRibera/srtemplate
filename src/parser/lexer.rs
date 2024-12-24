use super::TemplateNode;

/// Enum representing the types of template nodes.
#[derive(Debug, PartialEq, Clone)]
pub enum SintaxNode {
    RawText {
        start: usize,
        end: usize,
    },
    Variable {
        start: usize,
        end: usize,
    },
    Function {
        name_start: usize,
        name_end: usize,
        args: Vec<SintaxNode>,
    },
    Number {
        start: usize,
        end: usize,
    },
    Float {
        start: usize,
        end: usize,
    },
    Str {
        start: usize,
        end: usize,
    },
    Array(Vec<SintaxNode>),
    Dict(Vec<((usize, usize), SintaxNode)>),
}

impl<'a> From<(&'a str, SintaxNode)> for TemplateNode<'a> {
    fn from((input, value): (&'a str, SintaxNode)) -> Self {
        match value {
            SintaxNode::RawText { start, end } => Self::RawText(&input[start..end]),
            SintaxNode::Variable { start, end } => Self::Variable(&input[start..end]),
            SintaxNode::Function {
                name_start,
                name_end,
                args,
            } => Self::Function(
                &input[name_start..name_end],
                args.into_iter().map(|node| (input, node).into()).collect(),
            ),
            SintaxNode::Str { start, end } => Self::String(&input[start..end]),
            SintaxNode::Number { start, end } => unreachable!(),
            SintaxNode::Float { start, end } => unreachable!(),
            SintaxNode::Array(_) => unreachable!(),
            SintaxNode::Dict(_) => unreachable!(),
        }
    }
}
