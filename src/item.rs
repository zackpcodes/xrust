//! # xrust::item
//!
//! Sequence Item module.
//! An Item is a Node, Function or Atomic Value.
//!
//! Nodes are implemented as a trait.

use crate::output::OutputDefinition;
use crate::qname::QualifiedName;
use crate::value::{Operator, Value};
use crate::xdmerror::{Error, ErrorKind};
use std::fmt;
use std::fmt::Formatter;
use std::rc::Rc;
use crate::item;

/// In XPath, the Sequence is the fundamental data structure.
/// It is an ordered collection of [Item]s.
/// The Rust impementation is a Vector of reference counted [Item]s.
///
/// See [SequenceTrait] for methods.
pub type Sequence<N> = Vec<Rc<Item<N>>>;

pub trait SequenceTrait<N: Node> {
    /// Return the string value of the [Sequence].
    fn to_string(&self) -> String;
    /// Return a XML formatted representation of the [Sequence].
    fn to_xml(&self) -> String;
    /// Return a XML formatted representation of the [Sequence], controlled by the supplied output definition.
    fn to_xml_with_options(&self, od: &OutputDefinition) -> String;
    /// Return a JSON formatted representation of the [Sequence].
    fn to_json(&self) -> String;
    /// Return the Effective Boolean Value of the [Sequence].
    fn to_bool(&self) -> bool;
    /// Convert the [Sequence] to an integer. The [Sequence] must be a singleton value.
    fn to_int(&self) -> Result<i64, Error>;
    /// Push an [Node] to the [Sequence]
    fn push_node(&mut self, n: N);
    /// Push a [Value] to the [Sequence]
    fn push_value(&mut self, v: Value);
    /// Push an [Item] to the [Sequence]
    fn push_item(&mut self, i: &Rc<Item<N>>);
}

impl<N: Node> SequenceTrait<N> for Sequence<N> {
    /// Returns the string value of the Sequence.
    fn to_string(&self) -> String {
        let mut r = String::new();
        for i in self {
            r.push_str(i.to_string().as_str())
        }
        r
    }
    /// Renders the Sequence as XML
    fn to_xml(&self) -> String {
        let mut r = String::new();
        for i in self {
            r.push_str(i.to_xml().as_str())
        }
        r
    }
    /// Renders the Sequence as XML
    fn to_xml_with_options(&self, od: &OutputDefinition) -> String {
        let mut r = String::new();
        for i in self {
            r.push_str(i.to_xml_with_options(od).as_str())
        }
        r
    }
    /// Renders the Sequence as JSON
    fn to_json(&self) -> String {
        let mut r = String::new();
        for i in self {
            r.push_str(i.to_json().as_str())
        }
        r
    }
    /// Push a document's [Node] on to the [Sequence]
    fn push_node(&mut self, n: N) {
        self.push(Rc::new(Item::Node(n)));
    }
    /// Push a [Value] on to the [Sequence]
    fn push_value(&mut self, v: Value) {
        self.push(Rc::new(Item::Value(v)));
    }
    //fn new_function(&self, f: Function) -> Sequence {
    //}
    /// Push an [Item] on to the [Sequence]. This clones the Item.
    fn push_item(&mut self, i: &Rc<Item<N>>) {
        self.push(Rc::clone(i));
    }

    /// Calculate the effective boolean value of the Sequence
    fn to_bool(&self) -> bool {
        if self.is_empty() {
            false
        } else {
            match *self[0] {
                Item::Node(..) => true,
                _ => {
                    if self.len() == 1 {
                        (*self[0]).to_bool()
                    } else {
                        false // should be a type error
                    }
                }
            }
        }
    }

    /// Convenience routine for integer value of the [Sequence]. The Sequence must be a singleton; i.e. be a single item.
    fn to_int(&self) -> Result<i64, Error> {
        if self.len() == 1 {
            self[0].to_int()
        } else {
            Result::Err(Error {
                kind: ErrorKind::TypeError,
                message: String::from("type error: sequence is not a singleton"),
            })
        }
    }
}

impl<N: Node> From<Value> for Sequence<N> {
    fn from(v: Value) -> Self {
        vec![Rc::new(Item::Value(v))]
    }
}
impl<N: Node> From<Item<N>> for Sequence<N> {
    fn from(i: Item<N>) -> Self {
        vec![Rc::new(i)]
    }
}

/// All [Node]s have a type. The type of the [Node] determines what components are meaningful, such as name and content.
///
/// Every document must have a single node as it's toplevel node that is of type "Document".
#[derive(Copy, Clone, PartialEq, Debug)]
#[derive(Default)]
pub enum NodeType {
    Document,
    Element,
    Text,
    Attribute,
    Comment,
    ProcessingInstruction,
    Reference,
    #[default]
    Unknown,
}

impl NodeType {
    /// Return a string representation of the node type.
    pub fn to_string(&self) -> &'static str {
        match self {
            NodeType::Document => "Document",
            NodeType::Element => "Element",
            NodeType::Attribute => "Attribute",
            NodeType::Text => "Text",
            NodeType::ProcessingInstruction => "Processing-Instruction",
            NodeType::Comment => "Comment",
            NodeType::Reference => "Reference",
            NodeType::Unknown => "--None--",
        }
    }
}



/// An Item in a [Sequence]. Can be a node, function or [Value].
///
/// Functions are not yet implemented.
#[derive(Clone)]
pub enum Item<N: Node> {
    /// A [Node] in the source document.
    Node(N),

    /// Functions are not yet supported
    Function,

    /// A scalar value
    Value(Value),
}

impl<N: item::Node> fmt::Display for Item<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // Gives the string value of an item. All items have a string value.
        let result = match self {
            Item::Node(n) => n.to_string(),
            Item::Function => "".to_string(),
            Item::Value(v) => v.to_string(),
        };
        f.write_str(result.as_str())
    }
}

impl<N: Node> Item<N> {
    /// Serialize as XML
    pub fn to_xml(&self) -> String {
        match self {
            Item::Node(n) => n.to_xml(),
            Item::Function => "".to_string(),
            Item::Value(v) => v.to_string(),
        }
    }
    /// Serialize as XML, with options
    pub fn to_xml_with_options(&self, od: &OutputDefinition) -> String {
        match self {
            Item::Node(n) => n.to_xml_with_options(od),
            Item::Function => "".to_string(),
            Item::Value(v) => v.to_string(),
        }
    }
    /// Serialize as JSON
    pub fn to_json(&self) -> String {
        match self {
            Item::Node(n) => n.to_json(),
            Item::Function => "".to_string(),
            Item::Value(v) => v.to_string(),
        }
    }

    /// Determine the effective boolean value of the item.
    /// See XPath 2.4.3.
    pub fn to_bool(&self) -> bool {
        match self {
            Item::Node(..) => true,
            Item::Function => false,
            Item::Value(v) => v.to_bool(),
        }
    }

    /// Gives the integer value of the item, if possible.
    pub fn to_int(&self) -> Result<i64, Error> {
        match self {
            Item::Node(..) => Result::Err(Error {
                kind: ErrorKind::TypeError,
                message: String::from("type error: item is a node"),
            }),
            Item::Function => Result::Err(Error {
                kind: ErrorKind::TypeError,
                message: String::from("type error: item is a function"),
            }),
            Item::Value(v) => match v.to_int() {
                Ok(i) => Ok(i),
                Err(e) => Result::Err(e),
            },
        }
    }

    /// Gives the double value of the item. Returns NaN if the value cannot be converted to a double.
    pub fn to_double(&self) -> f64 {
        match self {
            Item::Node(..) => f64::NAN,
            Item::Function => f64::NAN,
            Item::Value(v) => v.to_double(),
        }
    }

    /// Gives the name of the item. Certain types of Nodes have names, such as element-type nodes. If the item does not have a name returns an empty string.
    pub fn name(&self) -> QualifiedName {
        match self {
            Item::Node(n) => n.name(),
            _ => QualifiedName::new(None, None, "".to_string()),
        }
    }

    // TODO: atomization
    // fn atomize(&self);

    /// Compare two items.
    pub fn compare(&self, other: &Item<N>, op: Operator) -> Result<bool, Error> {
        match self {
            Item::Value(v) => match other {
                Item::Value(w) => v.compare(w, op),
                Item::Node(..) => v.compare(&Value::String(other.to_string()), op),
                _ => Result::Err(Error {
                    kind: ErrorKind::TypeError,
                    message: String::from("type error"),
                }),
            },
            Item::Node(..) => other.compare(&Item::Value(Value::String(self.to_string())), op),
            _ => Result::Err(Error {
                kind: ErrorKind::TypeError,
                message: String::from("type error"),
            }),
        }
    }

    /// Is this item an element-type node?
    pub fn is_element_node(&self) -> bool {
        match self {
            Item::Node(n) => matches!(n.node_type(), NodeType::Element),
                /*
                match n.node_type() {
                NodeType::Element => true,
                _ => false,
            },
                 */
            _ => false,
        }
    }

    /// Convenience method to set an attribute for a Node-type item.
    /// If the item is not an element-type node, then this method has no effect.
    pub fn add_attribute(&self, a: N) -> Result<(), Error> {
        match self {
            Item::Node(n) => match n.node_type() {
                NodeType::Element => n.add_attribute(a),
                _ => Ok(()),
            },
            _ => Ok(()),
        }
    }

    /// Gives the type of the item.
    pub fn item_type(&self) -> &'static str {
        match self {
            Item::Node(..) => "Node",
            Item::Function => "Function",
            Item::Value(v) => v.value_type(),
        }
    }
}

impl<N: Node> fmt::Debug for Item<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Item::Node(n) => {
                write!(
                    f,
                    "node type item (node type {}, name \"{}\")",
                    n.node_type().to_string(),
                    n.name()
                )
            }
            Item::Function => {
                write!(f, "function type item")
            }
            Item::Value(v) => {
                write!(f, "value type item ({})", v)
            }
        }
    }
}

/// Nodes make up a document tree. Nodes must be fully navigable. The tree must be mutable but also stable (i.e. removing a node from the tree does not invalidate the remaining nodes).
///
/// Some nodes have names, such as elements. Some nodes have values, such as text or comments. Some have both a name and a value, such as attributes and processing instructions.
///
/// Element nodes have children and attributes.
pub trait Node: Clone {
    type NodeIterator: Iterator<Item = Self>;

    /// Get the type of the node
    fn node_type(&self) -> NodeType;
    /// Get the name of the node. If the node doesn't have a name, then returns a [QualifiedName] with an empty string for it's localname.
    fn name(&self) -> QualifiedName;
    /// Get the value of the node. If the node doesn't have a value, then returns a [Value] that is an empty string.
    fn value(&self) -> Value;

    /// Get the string value of the node. See XPath ???
    fn to_string(&self) -> String;
    /// Serialise the node as XML
    fn to_xml(&self) -> String;
    /// Serialise the node as XML, with options such as indentation.
    fn to_xml_with_options(&self, od: &OutputDefinition) -> String;
    /// Serialise the node as JSON
    fn to_json(&self) -> String {
        String::new()
    }

    /// Check if two Nodes are the same Node
    fn is_same(&self, other: &Self) -> bool;
    /// Check if a node is an element-type
    fn is_element(&self) -> bool {
        self.node_type() == NodeType::Element
    }

    /// An iterator over the children of the node
    fn child_iter(&self) -> Self::NodeIterator;
    /// Get the first child of the node, if there is one
    fn first_child(&self) -> Option<Self>
    where
        Self: Sized,
    {
        self.child_iter().next()
    }
    /// An iterator over the ancestors of the node
    fn ancestor_iter(&self) -> Self::NodeIterator;
    /// Get the parent of the node. Top-level nodes do not have parents, also nodes that have been detached from the tree.
    fn parent(&self) -> Option<Self>
    where
        Self: Sized,
    {
        self.ancestor_iter().next()
    }
    /// Get the document node
    fn owner_document(&self) -> Self;
    /// An iterator over the descendants of the node
    fn descend_iter(&self) -> Self::NodeIterator;
    /// An iterator over the following siblings of the node
    fn next_iter(&self) -> Self::NodeIterator;
    /// An iterator over the preceding siblings of the node
    fn prev_iter(&self) -> Self::NodeIterator;
    /// An iterator over the attributes of an element
    fn attribute_iter(&self) -> Self::NodeIterator;
    /// Get an attribute of the node. Returns a copy of the attribute's value. If the node does not have an attribute of the given name, a value containing an empty string is returned.
    fn get_attribute(&self, a: &QualifiedName) -> Value;

    /// Create a new element-type node in the same document tree. The new node is not attached to the tree.
    fn new_element(&self, qn: QualifiedName) -> Result<Self, Error>;
    /// Create a new text-type node in the same document tree. The new node is not attached to the tree.
    fn new_text(&self, v: Value) -> Result<Self, Error>;
    fn new_attribute(&self, qn: QualifiedName, v: Value) -> Result<Self, Error>;

    /// Append a node to the child list
    fn push(&mut self, n: Self) -> Result<(), Error>;
    /// Remove a node from the tree
    fn pop(&mut self) -> Result<(), Error>;
    /// Insert a node in the child list before the given node. The node will be detached from it's current position prior to insertion.
    fn insert_before(&mut self, n: Self) -> Result<(), Error>;
    /// Set an attribute. self must be an element-type node. att must be an attribute-type node.
    fn add_attribute(&self, att: Self) -> Result<(), Error>;

    /// Deep copy the node, i.e. the node itself and it's attributes and descendants. The resulting top-level node is unattached.
    fn deep_copy(&self) -> Result<Self, Error>;
    /// Canonical XML representation of the node
    fn get_canonical(&self) -> Result<Self, Error>;
}
