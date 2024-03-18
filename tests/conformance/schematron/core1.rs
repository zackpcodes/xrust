use std::fs;
use std::rc::Rc;
use xrust::parser::xml;
use xrust::trees::smite::{Node as SmiteNode};
use xrust::validators::schematron::validate_schematron;

#[test]
fn schematron_core_01(){

    /*
        ID: extends-baseuri-fixup
        Extends performs base URI fixup
        XML Inclusions (XInclude) Version 1.1, Section 4.7.5.
    */

    let s = fs::read_to_string("tests/conformance/schematron/core/01/schema.xml").unwrap();
    let schemadoc = Rc::new(SmiteNode::new());
    let _ = xml::parse(schemadoc.clone(), s.as_str(), None, None);

    let d = fs::read_to_string("tests/conformance/schematron/core/01/document.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), d.as_str(), None, None);

    let result = validate_schematron(&doc, &schemadoc);
    assert!(result.is_ok());
}

#[test]
fn schematron_core_02(){

    /*
        ID: extends-recursive
        Extends is recursive
        
    */

    let s = fs::read_to_string("tests/conformance/schematron/core/02/schema.xml").unwrap();
    let schemadoc = Rc::new(SmiteNode::new());
    let _ = xml::parse(schemadoc.clone(), s.as_str(), None, None);

    let d = fs::read_to_string("tests/conformance/schematron/core/02/document.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), d.as_str(), None, None);

    let result = validate_schematron(&doc, &schemadoc);
    assert!(result.is_err());
}

#[test]
fn schematron_core_03(){

    /*
        ID: include-baseuri-fixup
        Include performs base URI fixup
        XML Inclusions (XInclude) Version 1.1, Section 4.7.5.
    */

    let s = fs::read_to_string("tests/conformance/schematron/core/03/schema.xml").unwrap();
    let schemadoc = Rc::new(SmiteNode::new());
    let _ = xml::parse(schemadoc.clone(), s.as_str(), None, None);

    let d = fs::read_to_string("tests/conformance/schematron/core/03/document.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), d.as_str(), None, None);

    let result = validate_schematron(&doc, &schemadoc);
    assert!(result.is_ok());
}

#[test]
fn schematron_core_04(){

    /*
        ID: include-recursive
        Include is recursive
        
    */

    let s = fs::read_to_string("tests/conformance/schematron/core/04/schema.xml").unwrap();
    let schemadoc = Rc::new(SmiteNode::new());
    let _ = xml::parse(schemadoc.clone(), s.as_str(), None, None);

    let d = fs::read_to_string("tests/conformance/schematron/core/04/document.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), d.as_str(), None, None);

    let result = validate_schematron(&doc, &schemadoc);
    assert!(result.is_err());
}

#[test]
fn schematron_core_05(){

    /*
        ID: let-name-collision-error-01
        It is an error for a variable to be multiply defined in the current rule
        ISO Schematron 2016: Section 5.4.5 Clause 3
    */

    let s = fs::read_to_string("tests/conformance/schematron/core/05/schema.xml").unwrap();
    let schemadoc = Rc::new(SmiteNode::new());
    let _ = xml::parse(schemadoc.clone(), s.as_str(), None, None);

    let d = fs::read_to_string("tests/conformance/schematron/core/05/document.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), d.as_str(), None, None);

    let result = validate_schematron(&doc, &schemadoc);
    assert!(result.is_err());
}

#[test]
fn schematron_core_06(){

    /*
        ID: let-name-collision-error-02
        It is an error for a variable to be multiply defined in the current pattern
        ISO Schematron 2016: Section 5.4.5 Clause 3
    */

    let s = fs::read_to_string("tests/conformance/schematron/core/06/schema.xml").unwrap();
    let schemadoc = Rc::new(SmiteNode::new());
    let _ = xml::parse(schemadoc.clone(), s.as_str(), None, None);

    let d = fs::read_to_string("tests/conformance/schematron/core/06/document.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), d.as_str(), None, None);

    let result = validate_schematron(&doc, &schemadoc);
    assert!(result.is_err());
}

#[test]
fn schematron_core_07(){

    /*
        ID: let-name-collision-error-03
        It is an error for a variable to be multiply defined in the current schema
        ISO Schematron 2016: Section 5.4.5 Clause 3
    */

    let s = fs::read_to_string("tests/conformance/schematron/core/07/schema.xml").unwrap();
    let schemadoc = Rc::new(SmiteNode::new());
    let _ = xml::parse(schemadoc.clone(), s.as_str(), None, None);

    let d = fs::read_to_string("tests/conformance/schematron/core/07/document.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), d.as_str(), None, None);

    let result = validate_schematron(&doc, &schemadoc);
    assert!(result.is_err());
}

#[test]
fn schematron_core_08(){

    /*
        ID: let-name-collision-error-04
        It is an error for a variable to be multiply defined in the current phase
        ISO Schematron 2016: Section 5.4.5 Clause 3
    */

    let s = fs::read_to_string("tests/conformance/schematron/core/08/schema.xml").unwrap();
    let schemadoc = Rc::new(SmiteNode::new());
    let _ = xml::parse(schemadoc.clone(), s.as_str(), None, None);

    let d = fs::read_to_string("tests/conformance/schematron/core/08/document.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), d.as_str(), None, None);

    let result = validate_schematron(&doc, &schemadoc);
    assert!(result.is_err());
}

#[test]
fn schematron_core_09(){

    /*
        ID: let-name-collision-error-05
        It is an error for a variable to be multiply defined globally
        ISO Schematron 2016: Section 5.4.5 Clause 3
    */

    let s = fs::read_to_string("tests/conformance/schematron/core/09/schema.xml").unwrap();
    let schemadoc = Rc::new(SmiteNode::new());
    let _ = xml::parse(schemadoc.clone(), s.as_str(), None, None);

    let d = fs::read_to_string("tests/conformance/schematron/core/09/document.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), d.as_str(), None, None);

    let result = validate_schematron(&doc, &schemadoc);
    assert!(result.is_err());
}

#[test]
fn schematron_core_10(){

    /*
        ID: let-name-collision-error-06
        It is an error to define a pattern variable with the same name as a global variable
        ISO Schematron 2016: Section 5.4.5 clause 3
    */

    let s = fs::read_to_string("tests/conformance/schematron/core/10/schema.xml").unwrap();
    let schemadoc = Rc::new(SmiteNode::new());
    let _ = xml::parse(schemadoc.clone(), s.as_str(), None, None);

    let d = fs::read_to_string("tests/conformance/schematron/core/10/document.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), d.as_str(), None, None);

    let result = validate_schematron(&doc, &schemadoc);
    assert!(result.is_err());
}

#[test]
fn schematron_core_11(){

    /*
        ID: let-pattern-global-01
        A pattern variable has global scope
        ISO Schematron 2016: Section 5.4.5 clause 1
    */

    let s = fs::read_to_string("tests/conformance/schematron/core/11/schema.xml").unwrap();
    let schemadoc = Rc::new(SmiteNode::new());
    let _ = xml::parse(schemadoc.clone(), s.as_str(), None, None);

    let d = fs::read_to_string("tests/conformance/schematron/core/11/document.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), d.as_str(), None, None);

    let result = validate_schematron(&doc, &schemadoc);
    assert!(result.is_ok());
}

#[test]
fn schematron_core_12(){

    /*
        ID: let-reference-undefined-01
        It is an error to reference a variable in a rule context expression that has not been defined globally
        ISO Schematron 2016: Section 5.4.5 Clause 3
    */

    let s = fs::read_to_string("tests/conformance/schematron/core/12/schema.xml").unwrap();
    let schemadoc = Rc::new(SmiteNode::new());
    let _ = xml::parse(schemadoc.clone(), s.as_str(), None, None);

    let d = fs::read_to_string("tests/conformance/schematron/core/12/document.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), d.as_str(), None, None);

    let result = validate_schematron(&doc, &schemadoc);
    assert!(result.is_err());
}

#[test]
fn schematron_core_13(){

    /*
        ID: let-reference-undefined-02
        It is an error to reference a variable in an assert test expression that has not been defined globally
        ISO Schematron 2016: Section 5.4.5 Clause 3
    */

    let s = fs::read_to_string("tests/conformance/schematron/core/13/schema.xml").unwrap();
    let schemadoc = Rc::new(SmiteNode::new());
    let _ = xml::parse(schemadoc.clone(), s.as_str(), None, None);

    let d = fs::read_to_string("tests/conformance/schematron/core/13/document.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), d.as_str(), None, None);

    let result = validate_schematron(&doc, &schemadoc);
    assert!(result.is_err());
}

#[test]
fn schematron_core_14(){

    /*
        ID: let-reference-undefined-03
        It is an error to reference a variable in an report test expression that has not been defined globally
        ISO Schematron 2016: Section 5.4.5 Clause 3
    */

    let s = fs::read_to_string("tests/conformance/schematron/core/14/schema.xml").unwrap();
    let schemadoc = Rc::new(SmiteNode::new());
    let _ = xml::parse(schemadoc.clone(), s.as_str(), None, None);

    let d = fs::read_to_string("tests/conformance/schematron/core/14/document.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), d.as_str(), None, None);

    let result = validate_schematron(&doc, &schemadoc);
    assert!(result.is_err());
}

#[test]
fn schematron_core_15(){

    /*
        ID: let-reference-undefined-04
        It is an error to reference a variable in an rule variable that has not been defined globally
        ISO Schematron 2016: Section 5.4.5 Clause 3
    */

    let s = fs::read_to_string("tests/conformance/schematron/core/15/schema.xml").unwrap();
    let schemadoc = Rc::new(SmiteNode::new());
    let _ = xml::parse(schemadoc.clone(), s.as_str(), None, None);

    let d = fs::read_to_string("tests/conformance/schematron/core/15/document.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), d.as_str(), None, None);

    let result = validate_schematron(&doc, &schemadoc);
    assert!(result.is_err());
}

#[test]
fn schematron_core_16(){

    /*
        ID: let-reference-undefined-05
        It is an error to reference a variable in the @select expression of a sch:value-of element that has not been defined globally
        ISO Schematron 2016: Section 5.4.5 Clause 3
    */

    let s = fs::read_to_string("tests/conformance/schematron/core/16/schema.xml").unwrap();
    let schemadoc = Rc::new(SmiteNode::new());
    let _ = xml::parse(schemadoc.clone(), s.as_str(), None, None);

    let d = fs::read_to_string("tests/conformance/schematron/core/16/document.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), d.as_str(), None, None);

    let result = validate_schematron(&doc, &schemadoc);
    assert!(result.is_err());
}

#[test]
fn schematron_core_17(){

    /*
        ID: let-reference-undefined-06
        It is an error to reference a variable in the @path expression of a sch:name element that has not been defined globally
        ISO Schematron 2016: Section 5.4.5 Clause 3
    */

    let s = fs::read_to_string("tests/conformance/schematron/core/17/schema.xml").unwrap();
    let schemadoc = Rc::new(SmiteNode::new());
    let _ = xml::parse(schemadoc.clone(), s.as_str(), None, None);

    let d = fs::read_to_string("tests/conformance/schematron/core/17/document.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), d.as_str(), None, None);

    let result = validate_schematron(&doc, &schemadoc);
    assert!(result.is_err());
}

#[test]
fn schematron_core_18(){

    /*
        ID: let-reference-undefined-07
        It is an error to reference an undefined variable in the @documents expression of a sch:pattern element
        ISO Schematron 2016: Section 5.4.5 Clause 3
    */

    let s = fs::read_to_string("tests/conformance/schematron/core/18/schema.xml").unwrap();
    let schemadoc = Rc::new(SmiteNode::new());
    let _ = xml::parse(schemadoc.clone(), s.as_str(), None, None);

    let d = fs::read_to_string("tests/conformance/schematron/core/18/document.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), d.as_str(), None, None);

    let result = validate_schematron(&doc, &schemadoc);
    assert!(result.is_err());
}

#[test]
fn schematron_core_19(){

    /*
        ID: let-rule-global-01
        A rule variable can use a schema variable
        ISO Schematron 2016: Section 6.5 clause 6
    */

    let s = fs::read_to_string("tests/conformance/schematron/core/19/schema.xml").unwrap();
    let schemadoc = Rc::new(SmiteNode::new());
    let _ = xml::parse(schemadoc.clone(), s.as_str(), None, None);

    let d = fs::read_to_string("tests/conformance/schematron/core/19/document.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), d.as_str(), None, None);

    let result = validate_schematron(&doc, &schemadoc);
    assert!(result.is_ok());
}

#[test]
fn schematron_core_20(){

    /*
        ID: let-rule-global-02
        A rule variable can use a phase variable
        ISO Schematron 2016: Section 6.5 clause 6
    */

    let s = fs::read_to_string("tests/conformance/schematron/core/20/schema.xml").unwrap();
    let schemadoc = Rc::new(SmiteNode::new());
    let _ = xml::parse(schemadoc.clone(), s.as_str(), None, None);

    let d = fs::read_to_string("tests/conformance/schematron/core/20/document.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), d.as_str(), None, None);

    let result = validate_schematron(&doc, &schemadoc);
    assert!(result.is_ok());
}

#[test]
fn schematron_core_21(){

    /*
        ID: let-scope-pattern-01
        Pattern-variable is scoped to the pattern
        ISO Schematron 2016: Section 3.26
    */

    let s = fs::read_to_string("tests/conformance/schematron/core/21/schema.xml").unwrap();
    let schemadoc = Rc::new(SmiteNode::new());
    let _ = xml::parse(schemadoc.clone(), s.as_str(), None, None);

    let d = fs::read_to_string("tests/conformance/schematron/core/21/document.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), d.as_str(), None, None);

    let result = validate_schematron(&doc, &schemadoc);
    assert!(result.is_ok());
}

#[test]
fn schematron_core_22(){

    /*
        ID: let-scope-phase-01
        Phase-variable is scoped to the phase
        ISO Schematron 2016: Section 3.26
    */

    let s = fs::read_to_string("tests/conformance/schematron/core/22/schema.xml").unwrap();
    let schemadoc = Rc::new(SmiteNode::new());
    let _ = xml::parse(schemadoc.clone(), s.as_str(), None, None);

    let d = fs::read_to_string("tests/conformance/schematron/core/22/document.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), d.as_str(), None, None);

    let result = validate_schematron(&doc, &schemadoc);
    assert!(result.is_ok());
}

#[test]
fn schematron_core_23(){

    /*
        ID: let-scope-rule-01
        Rule-variable is scoped to the rule
        ISO Schematron 2016: Section 3.26
    */

    let s = fs::read_to_string("tests/conformance/schematron/core/23/schema.xml").unwrap();
    let schemadoc = Rc::new(SmiteNode::new());
    let _ = xml::parse(schemadoc.clone(), s.as_str(), None, None);

    let d = fs::read_to_string("tests/conformance/schematron/core/23/document.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), d.as_str(), None, None);

    let result = validate_schematron(&doc, &schemadoc);
    assert!(result.is_ok());
}

#[test]
fn schematron_core_24(){

    /*
        ID: let-value-element-content-01
        Let uses the element content as value
        ISO Schematron 2016: Section 5.4.5 clause 2
    */

    let s = fs::read_to_string("tests/conformance/schematron/core/24/schema.xml").unwrap();
    let schemadoc = Rc::new(SmiteNode::new());
    let _ = xml::parse(schemadoc.clone(), s.as_str(), None, None);

    let d = fs::read_to_string("tests/conformance/schematron/core/24/document.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), d.as_str(), None, None);

    let result = validate_schematron(&doc, &schemadoc);
    assert!(result.is_ok());
}

#[test]
fn schematron_core_25(){

    /*
        ID: pattern-abstract-01
        An abstract pattern is instantiated
        ISO Schematron 2016: Section 6.3
    */

    let s = fs::read_to_string("tests/conformance/schematron/core/25/schema.xml").unwrap();
    let schemadoc = Rc::new(SmiteNode::new());
    let _ = xml::parse(schemadoc.clone(), s.as_str(), None, None);

    let d = fs::read_to_string("tests/conformance/schematron/core/25/document.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), d.as_str(), None, None);

    let result = validate_schematron(&doc, &schemadoc);
    assert!(result.is_err());
}

#[test]
fn schematron_core_26(){

    /*
        ID: pattern-subordinate-document-01
        Pattern in a subordinate document
        ISO Schematron 2016: Section 5.4.9 clause 2
    */

    let s = fs::read_to_string("tests/conformance/schematron/core/26/schema.xml").unwrap();
    let schemadoc = Rc::new(SmiteNode::new());
    let _ = xml::parse(schemadoc.clone(), s.as_str(), None, None);

    let d = fs::read_to_string("tests/conformance/schematron/core/26/document.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), d.as_str(), None, None);

    let result = validate_schematron(&doc, &schemadoc);
    assert!(result.is_err());
}

#[test]
fn schematron_core_27(){

    /*
        ID: pattern-subordinate-document-02
        The subordinate document expression contains a variable
        
    */

    let s = fs::read_to_string("tests/conformance/schematron/core/27/schema.xml").unwrap();
    let schemadoc = Rc::new(SmiteNode::new());
    let _ = xml::parse(schemadoc.clone(), s.as_str(), None, None);

    let d = fs::read_to_string("tests/conformance/schematron/core/27/document.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), d.as_str(), None, None);

    let result = validate_schematron(&doc, &schemadoc);
    assert!(result.is_err());
}

#[test]
fn schematron_core_28(){

    /*
        ID: rule-abstract-01
        An abstract rule is instantiated
        ISO Schematron 2016: Section 5.4.12 clause 5
    */

    let s = fs::read_to_string("tests/conformance/schematron/core/28/schema.xml").unwrap();
    let schemadoc = Rc::new(SmiteNode::new());
    let _ = xml::parse(schemadoc.clone(), s.as_str(), None, None);

    let d = fs::read_to_string("tests/conformance/schematron/core/28/document.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), d.as_str(), None, None);

    let result = validate_schematron(&doc, &schemadoc);
    assert!(result.is_err());
}

#[test]
fn schematron_core_29(){

    /*
        ID: rule-abstract-02
        It is an error to extend an abstract rule that is defined in a different pattern
        ISO Schematron 2016: Section 5.4.12 clause 5
    */

    let s = fs::read_to_string("tests/conformance/schematron/core/29/schema.xml").unwrap();
    let schemadoc = Rc::new(SmiteNode::new());
    let _ = xml::parse(schemadoc.clone(), s.as_str(), None, None);

    let d = fs::read_to_string("tests/conformance/schematron/core/29/document.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), d.as_str(), None, None);

    let result = validate_schematron(&doc, &schemadoc);
    assert!(result.is_err());
}

#[test]
fn schematron_core_30(){

    /*
        ID: rule-context-attribute-01
        Context node is an attribute node
        ISO Schematron 2016: Annex C Clause 2 (xslt), Annex H Clause 2 (xslt2), Annex I Clause 2 (xpath2)
    */

    let s = fs::read_to_string("tests/conformance/schematron/core/30/schema.xml").unwrap();
    let schemadoc = Rc::new(SmiteNode::new());
    let _ = xml::parse(schemadoc.clone(), s.as_str(), None, None);

    let d = fs::read_to_string("tests/conformance/schematron/core/30/document.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), d.as_str(), None, None);

    let result = validate_schematron(&doc, &schemadoc);
    assert!(result.is_err());
}

#[test]
fn schematron_core_31(){

    /*
        ID: rule-context-comment-01
        Context node is a comment node
        ISO Schematron 2016: Annex C Clause 2 (xslt), Annex H Clause 2 (xslt2), Annex I Clause 2 (xpath2)
    */

    let s = fs::read_to_string("tests/conformance/schematron/core/31/schema.xml").unwrap();
    let schemadoc = Rc::new(SmiteNode::new());
    let _ = xml::parse(schemadoc.clone(), s.as_str(), None, None);

    let d = fs::read_to_string("tests/conformance/schematron/core/31/document.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), d.as_str(), None, None);

    let result = validate_schematron(&doc, &schemadoc);
    assert!(result.is_err());
}

#[test]
fn schematron_core_32(){

    /*
        ID: rule-context-element-01
        Context node is an element node
        ISO Schematron 2016: Annex C Clause 2 (xslt), Annex H Clause 2 (xslt2), Annex I Clause 2 (xpath2)
    */

    let s = fs::read_to_string("tests/conformance/schematron/core/32/schema.xml").unwrap();
    let schemadoc = Rc::new(SmiteNode::new());
    let _ = xml::parse(schemadoc.clone(), s.as_str(), None, None);

    let d = fs::read_to_string("tests/conformance/schematron/core/32/document.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), d.as_str(), None, None);

    let result = validate_schematron(&doc, &schemadoc);
    assert!(result.is_err());
}

#[test]
fn schematron_core_33(){

    /*
        ID: rule-context-pi-01
        Context node is a processing instruction node
        ISO Schematron 2016: Annex C Clause 2 (xslt), Annex H Clause 2 (xslt2), Annex I Clause 2 (xpath2)
    */

    let s = fs::read_to_string("tests/conformance/schematron/core/33/schema.xml").unwrap();
    let schemadoc = Rc::new(SmiteNode::new());
    let _ = xml::parse(schemadoc.clone(), s.as_str(), None, None);

    let d = fs::read_to_string("tests/conformance/schematron/core/33/document.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), d.as_str(), None, None);

    let result = validate_schematron(&doc, &schemadoc);
    assert!(result.is_err());
}

#[test]
fn schematron_core_34(){

    /*
        ID: rule-context-root-01
        Context node is the root node
        ISO Schematron 2016: Annex C clause 2 (xslt), Annex H clause 2 (xslt2)
    */

    let s = fs::read_to_string("tests/conformance/schematron/core/34/schema.xml").unwrap();
    let schemadoc = Rc::new(SmiteNode::new());
    let _ = xml::parse(schemadoc.clone(), s.as_str(), None, None);

    let d = fs::read_to_string("tests/conformance/schematron/core/34/document.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), d.as_str(), None, None);

    let result = validate_schematron(&doc, &schemadoc);
    assert!(result.is_err());
}

#[test]
fn schematron_core_35(){

    /*
        ID: rule-context-text-01
        Context node is a text node
        ISO Schematron 2016: Annex C Clause 2 (xslt), Annex H Clause 2 (xslt2)
    */

    let s = fs::read_to_string("tests/conformance/schematron/core/35/schema.xml").unwrap();
    let schemadoc = Rc::new(SmiteNode::new());
    let _ = xml::parse(schemadoc.clone(), s.as_str(), None, None);

    let d = fs::read_to_string("tests/conformance/schematron/core/35/document.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), d.as_str(), None, None);

    let result = validate_schematron(&doc, &schemadoc);
    assert!(result.is_err());
}

#[test]
fn schematron_core_36(){

    /*
        ID: rule-context-variable-01
        Rule context expression uses a pattern variable
        
    */

    let s = fs::read_to_string("tests/conformance/schematron/core/36/schema.xml").unwrap();
    let schemadoc = Rc::new(SmiteNode::new());
    let _ = xml::parse(schemadoc.clone(), s.as_str(), None, None);

    let d = fs::read_to_string("tests/conformance/schematron/core/36/document.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), d.as_str(), None, None);

    let result = validate_schematron(&doc, &schemadoc);
    assert!(result.is_ok());
}

#[test]
fn schematron_core_37(){

    /*
        ID: rule-context-variable-02
        Rule context expression uses a phase variable
        
    */

    let s = fs::read_to_string("tests/conformance/schematron/core/37/schema.xml").unwrap();
    let schemadoc = Rc::new(SmiteNode::new());
    let _ = xml::parse(schemadoc.clone(), s.as_str(), None, None);

    let d = fs::read_to_string("tests/conformance/schematron/core/37/document.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), d.as_str(), None, None);

    let result = validate_schematron(&doc, &schemadoc);
    assert!(result.is_ok());
}

#[test]
fn schematron_core_38(){

    /*
        ID: rule-context-variable-03
        Rule context expression uses a schema variable
        
    */

    let s = fs::read_to_string("tests/conformance/schematron/core/38/schema.xml").unwrap();
    let schemadoc = Rc::new(SmiteNode::new());
    let _ = xml::parse(schemadoc.clone(), s.as_str(), None, None);

    let d = fs::read_to_string("tests/conformance/schematron/core/38/document.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), d.as_str(), None, None);

    let result = validate_schematron(&doc, &schemadoc);
    assert!(result.is_ok());
}

#[test]
fn schematron_core_39(){

    /*
        ID: rule-order-01
        Lexical order of rules is significant
        ISO Schematron 2016: Section 6.5 Clause 5
    */

    let s = fs::read_to_string("tests/conformance/schematron/core/39/schema.xml").unwrap();
    let schemadoc = Rc::new(SmiteNode::new());
    let _ = xml::parse(schemadoc.clone(), s.as_str(), None, None);

    let d = fs::read_to_string("tests/conformance/schematron/core/39/document.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), d.as_str(), None, None);

    let result = validate_schematron(&doc, &schemadoc);
    assert!(result.is_ok());
}

#[test]
fn schematron_core_40(){

    /*
        ID: schema-default-phase-01
        When no phase is given, the processor uses the phase given in @defaultPhase
        ISO Schematron 2016: Section 5.4.13 clause 3
    */

    let s = fs::read_to_string("tests/conformance/schematron/core/40/schema.xml").unwrap();
    let schemadoc = Rc::new(SmiteNode::new());
    let _ = xml::parse(schemadoc.clone(), s.as_str(), None, None);

    let d = fs::read_to_string("tests/conformance/schematron/core/40/document.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), d.as_str(), None, None);

    let result = validate_schematron(&doc, &schemadoc);
    assert!(result.is_ok());
}

#[test]
fn schematron_core_41(){

    /*
        ID: schema-default-phase-02
        When a phase named '#DEFAULT' is given, the processor uses the phase given in @defaultPhase
        
    */

    let s = fs::read_to_string("tests/conformance/schematron/core/41/schema.xml").unwrap();
    let schemadoc = Rc::new(SmiteNode::new());
    let _ = xml::parse(schemadoc.clone(), s.as_str(), None, None);

    let d = fs::read_to_string("tests/conformance/schematron/core/41/document.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), d.as_str(), None, None);

    let result = validate_schematron(&doc, &schemadoc);
    assert!(result.is_ok());
}

#[test]
fn schematron_core_42(){

    /*
        ID: xslt-key-01
        The XSLT key element may be used before the pattern elements
        ISO Schematron 2020: Annex C Clause 10 (xslt), Annex H Clause 11 (xslt2), Annex J Clause 11 (xslt3)
    */

    let s = fs::read_to_string("tests/conformance/schematron/core/42/schema.xml").unwrap();
    let schemadoc = Rc::new(SmiteNode::new());
    let _ = xml::parse(schemadoc.clone(), s.as_str(), None, None);

    let d = fs::read_to_string("tests/conformance/schematron/core/42/document.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), d.as_str(), None, None);

    let result = validate_schematron(&doc, &schemadoc);
    assert!(result.is_ok());
}

#[test]
fn schematron_core_43(){

    /*
        ID: xslt-key-element-content-01
        An xsl:key element can have element content
        ISO Schematron 2016: Annex H, XSLT 2.0 Section 16.3.1
    */

    let s = fs::read_to_string("tests/conformance/schematron/core/43/schema.xml").unwrap();
    let schemadoc = Rc::new(SmiteNode::new());
    let _ = xml::parse(schemadoc.clone(), s.as_str(), None, None);

    let d = fs::read_to_string("tests/conformance/schematron/core/43/document.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), d.as_str(), None, None);

    let result = validate_schematron(&doc, &schemadoc);
    assert!(result.is_ok());
}

