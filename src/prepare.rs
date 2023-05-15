use ahash::AHashMap;
use std::borrow::Cow;

use crate::types::{Builtins, Expr, Node};
use crate::object::Object;

pub(crate) type PrepareResult<T> = Result<T, Cow<'static, str>>;

pub(crate) type RunNode = Node<usize, Builtins>;
pub(crate) type RunExpr = Expr<usize, Builtins>;

/// TODO:
/// * pre-calculate const expressions
/// * const assignment add directly to namespace
pub(crate) fn prepare(nodes: Vec<Node<String, String>>, input_names: &[&str]) -> PrepareResult<(Vec<Object>, Vec<RunNode>)> {
    let mut p = Prepare::new(nodes.len(), input_names);
    let new_nodes = p.prepare_nodes(nodes)?;
    let initial_namespace = vec![Object::Undefined; p.names_count];
    Ok((initial_namespace, new_nodes))
}

struct Prepare {
    name_map: AHashMap<String, usize>,
    names_count: usize,
}

impl Prepare {
    fn new(capacity: usize, input_names: &[&str]) -> Self {
        let mut name_map = AHashMap::with_capacity(capacity);
        for (index, name) in input_names.iter().enumerate() {
            name_map.insert(name.to_string(), index);
        }
        let names_count = input_names.len();
        Self { name_map, names_count }
    }

    fn prepare_nodes(&mut self, nodes: Vec<Node<String, String>>) -> PrepareResult<Vec<RunNode>> {
        let mut new_nodes = Vec::with_capacity(nodes.len());
        for node in nodes {
            match node {
                Node::Pass => (),
                Node::Expr(expr) => {
                    let expr = self.prepare_expression(expr)?;
                    new_nodes.push(Node::Expr(expr));
                }
                Node::Assign { target, object } => {
                    let target = self.get_id(target);
                    let object = Box::new(self.prepare_expression(*object)?);
                    new_nodes.push(Node::Assign { target, object });
                }
                Node::OpAssign { target, op, object } => {
                    let target = self.get_id(target);
                    let object = Box::new(self.prepare_expression(*object)?);
                    new_nodes.push(Node::OpAssign { target, op, object });
                }
                Node::For {
                    target,
                    iter,
                    body,
                    or_else,
                } => new_nodes.push(Node::For {
                    target: self.prepare_expression(target)?,
                    iter: self.prepare_expression(iter)?,
                    body: self.prepare_nodes(body)?,
                    or_else: self.prepare_nodes(or_else)?,
                }),
                Node::If { test, body, or_else } => new_nodes.push(Node::If {
                    test: self.prepare_expression(test)?,
                    body: self.prepare_nodes(body)?,
                    or_else: self.prepare_nodes(or_else)?,
                }),
            }
        }
        Ok(new_nodes)
    }

    fn prepare_expression(&mut self, expr: Expr<String, String>) -> PrepareResult<RunExpr> {
        match expr {
            Expr::Constant(object) => Ok(Expr::Constant(object)),
            Expr::Name(name) => Ok(Expr::Name(self.get_id(name))),
            Expr::Op { left, op, right } => Ok(Expr::Op {
                left: Box::new(self.prepare_expression(*left)?),
                op,
                right: Box::new(self.prepare_expression(*right)?),
            }),
            Expr::CmpOp { left, op, right } => Ok(Expr::CmpOp {
                left: Box::new(self.prepare_expression(*left)?),
                op,
                right: Box::new(self.prepare_expression(*right)?),
            }),
            Expr::Call { func, args, kwargs } => {
                let func = Builtins::find(&func)?;
                Ok(Expr::Call {
                    func,
                    args: args
                        .into_iter()
                        .map(|e| self.prepare_expression(e))
                        .collect::<PrepareResult<Vec<_>>>()?,
                    kwargs: kwargs
                        .into_iter()
                        .map(|(_, e)| self.prepare_expression(e).map(|e| (0, e)))
                        .collect::<PrepareResult<Vec<_>>>()?,
                })
            }
            Expr::List(elements) => {
                let expressions = elements
                    .into_iter()
                    .map(|e| self.prepare_expression(e))
                    .collect::<PrepareResult<Vec<_>>>()?;
                Ok(Expr::List(expressions))
            }
        }
    }

    fn get_id(&mut self, name: String) -> usize {
        *self.name_map.entry(name).or_insert_with(|| {
            let name = self.names_count;
            self.names_count += 1;
            name
        })
    }
}
