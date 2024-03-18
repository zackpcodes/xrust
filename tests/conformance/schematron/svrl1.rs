use std::fs;
use std::rc::Rc;
use xrust::parser::xml;
use xrust::trees::smite::{Node as SmiteNode};
use xrust::validators::schematron::validate_schematron;

#[test]
fn schematron_svrl_01(){

    /*
        ID: svrl-diagnostic-01
        Diagnostic references are copied to SVRL output
        
    */

    let s = fs::read_to_string("tests/conformance/schematron/svrl/01/schema.xml").unwrap();
    let schemadoc = Rc::new(SmiteNode::new());
    let _ = xml::parse(schemadoc.clone(), s.as_str(), None, None);

    let d = fs::read_to_string("tests/conformance/schematron/svrl/01/document.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), d.as_str(), None, None);

    let result = validate_schematron(&doc, &schemadoc);
    assert!(result.is_err());
}

#[test]
fn schematron_svrl_02(){

    /*
        ID: svrl-diagnostic-02
        Language tag of diagnostic is preserved in SVRL output
        
    */

    let s = fs::read_to_string("tests/conformance/schematron/svrl/02/schema.xml").unwrap();
    let schemadoc = Rc::new(SmiteNode::new());
    let _ = xml::parse(schemadoc.clone(), s.as_str(), None, None);

    let d = fs::read_to_string("tests/conformance/schematron/svrl/02/document.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), d.as_str(), None, None);

    let result = validate_schematron(&doc, &schemadoc);
    assert!(result.is_err());
}

#[test]
fn schematron_svrl_03(){

    /*
        ID: svrl-name-nopath-01
        The sch:name element expands into the name of the context node if no @path is present
        ISO Schematron 2016: Section 5.4.6, Annex C clause 4 (xslt), Annex H clause 4 (xslt2), Annex I clause 4 (xpath2)
    */

    let s = fs::read_to_string("tests/conformance/schematron/svrl/03/schema.xml").unwrap();
    let schemadoc = Rc::new(SmiteNode::new());
    let _ = xml::parse(schemadoc.clone(), s.as_str(), None, None);

    let d = fs::read_to_string("tests/conformance/schematron/svrl/03/document.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), d.as_str(), None, None);

    let result = validate_schematron(&doc, &schemadoc);
    assert!(result.is_err());
}

#[test]
fn schematron_svrl_04(){

    /*
        ID: svrl-name-path-01
        The sch:name element expands into the value of evaluating the expression in @path
        ISO Schematron 2016: Section 5.4.6, Annex C clause 4 (xslt), Annex H clause 4 (xslt2), Annex I clause 4 (xpath2)
    */

    let s = fs::read_to_string("tests/conformance/schematron/svrl/04/schema.xml").unwrap();
    let schemadoc = Rc::new(SmiteNode::new());
    let _ = xml::parse(schemadoc.clone(), s.as_str(), None, None);

    let d = fs::read_to_string("tests/conformance/schematron/svrl/04/document.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), d.as_str(), None, None);

    let result = validate_schematron(&doc, &schemadoc);
    assert!(result.is_err());
}

#[test]
fn schematron_svrl_05(){

    /*
        ID: svrl-property-01
        Property references are copied to SVRL output
        
    */

    let s = fs::read_to_string("tests/conformance/schematron/svrl/05/schema.xml").unwrap();
    let schemadoc = Rc::new(SmiteNode::new());
    let _ = xml::parse(schemadoc.clone(), s.as_str(), None, None);

    let d = fs::read_to_string("tests/conformance/schematron/svrl/05/document.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), d.as_str(), None, None);

    let result = validate_schematron(&doc, &schemadoc);
    assert!(result.is_err());
}

#[test]
fn schematron_svrl_06(){

    /*
        ID: svrl-property-copy-of
        A xsl:copy-of inside a sch:property is executed
        ISO Schematron 2016: Annex C Clause 11 (xslt), Annex H Clause 11 (xslt2)
    */

    let s = fs::read_to_string("tests/conformance/schematron/svrl/06/schema.xml").unwrap();
    let schemadoc = Rc::new(SmiteNode::new());
    let _ = xml::parse(schemadoc.clone(), s.as_str(), None, None);

    let d = fs::read_to_string("tests/conformance/schematron/svrl/06/document.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), d.as_str(), None, None);

    let result = validate_schematron(&doc, &schemadoc);
    assert!(result.is_err());
}

#[test]
fn schematron_svrl_07(){

    /*
        ID: svrl-value-of-01
        The sch:value-of element expands into the value of evaluating the expression in @select
        ISO Schematron 2016: Section 5.4.14, Annex C clause 5 (xslt), Annex H clause 5 (xslt2), Annex I clause 5 (xpath2)
    */

    let s = fs::read_to_string("tests/conformance/schematron/svrl/07/schema.xml").unwrap();
    let schemadoc = Rc::new(SmiteNode::new());
    let _ = xml::parse(schemadoc.clone(), s.as_str(), None, None);

    let d = fs::read_to_string("tests/conformance/schematron/svrl/07/document.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), d.as_str(), None, None);

    let result = validate_schematron(&doc, &schemadoc);
    assert!(result.is_err());
}

