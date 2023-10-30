//! These functions are for features defined in XPath Functions 1.0 and 2.0.

use std::rc::Rc;

use crate::xdmerror::{Error, ErrorKind};
use crate::value::Value;
use crate::item::{Item, Node, NodeType, Sequence, SequenceTrait};
use crate::transform::Transform;
use crate::transform::context::{Context, ContextBuilder};

/// XPath position function.
pub fn position<N: Node>(ctxt: &Context<N>) -> Result<Sequence<N>, Error> {
    Ok(vec![Rc::new(Item::Value(Value::from(ctxt.i as i64 + 1)))])
}

/// XPath last function.
pub fn last<N: Node>(ctxt: &Context<N>) -> Result<Sequence<N>, Error> {
        Ok(vec![Rc::new(Item::Value(Value::from(
            ctxt.cur.len() as i64
        )))])
}

/// XPath count function.
pub fn tr_count<N: Node>(
    ctxt: &Context<N>,
    s: Option<Transform<N>>,
) -> Result<Sequence<N>, Error> {
        s.as_ref().map_or_else(
            || {
                Ok(vec![Rc::new(Item::Value(Value::from(
                    ctxt.cur.len() as i64
                )))])
            },
            |i| {
                Ok(vec![Rc::new(Item::Value(Value::from(
                    ctxt.dispatch(i)?.len() as i64,
                )))])
            },
        )
}

/// A user defined function.
/// Each argument is declared as a variable in the [Context].
/// The body of the function is then evaluated and it's result is returned.
pub fn user_defined<N: Node>(
    ctxt: &Context<N>,
    body: &Transform<N>,
    arguments: Vec<(String, &Transform<N>)>,
) -> Result<Sequence<N>, Error> {
    let mut new_ctxt = ctxt.clone();
        arguments.iter().try_for_each(|(n, a)| match ctxt.dispatch(a) {
            Ok(b) => {
                new_ctxt.var_push(n.clone(), b);
                Ok(())
            }
            Err(err) => Err(err),
        })?;
    new_ctxt.dispatch(body)
}

pub(crate) fn not_implemented<N : Node>(
    _ctxt: &Context<N>,
    msg: &String,
) -> Result<Sequence<N>, Error> {
    Err(Error::new(ErrorKind::NotImplemented, msg.clone()))
}
