mod core1;
mod svrl1;

use std::fs;
use std::rc::Rc;
use xrust::parser::xml;
use xrust::trees::smite::{Node as SmiteNode};
use xrust::validators::schematron::validate_schematron;

/*
#[test]
fn schetestone(){

    let s = "<?xml version=\"1.0\" encoding=\"utf-8\"?><sch:schema xmlns:sch=\"http://purl.oclc.org/dsdl/schematron\" xmlns=\"tag:dmaus@dmaus.name,2019:Schematron:Testsuite\"><sch:pattern><sch:rule context=\"/\"><sch:report test=\"true()\" diagnostics=\"d1 d2\"/></sch:rule></sch:pattern><sch:diagnostics><sch:diagnostic id=\"d1\">
          Context: <sch:value-of select=\"name()\"/></sch:diagnostic><sch:diagnostic id=\"d2\">
          Context: <sch:value-of select=\"name()\"/></sch:diagnostic></sch:diagnostics></sch:schema>";

    let d = "<?xml version=\"1.0\" encoding=\"utf-8\"?><element/>";

    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), d, None, None);

    let schemadoc = Rc::new(SmiteNode::new());
    let _ = xml::parse(schemadoc.clone(), s, None, None);

    let result = validate_schematron(&doc, &schemadoc);
    assert!(result.is_ok());
}
*/