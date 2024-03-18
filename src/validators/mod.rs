pub mod schematron;


use std::rc::Rc;
use crate::item::{Node, SequenceTrait};
use crate::trees::smite::{RNode, Node as SmiteNode};
use crate::validators::schematron::validate_schematron;
use crate::parser::xml;


pub(crate) enum Schema{
    Schematron(String), //Schema File
    //XMLSchema(schemafile)
    //RelaxNG(schemafile)
    //DTD //How do we pull the DTD? Store on doc while parsing?
}

pub enum ValidationError{
    DocumentError(String),
    SchemaError(String)
}


pub(crate) fn validate(doc: &RNode, s: Schema) -> Result<(), ValidationError>  {
    match s {
        Schema::Schematron(schema) => {
            let schemadoc = Rc::new(SmiteNode::new());
            let e = xml::parse(schemadoc.clone(), schema.as_str(), None, None);
            validate_schematron(doc, &schemadoc)
        }
    }
}


