use std::fs;
use std::rc::Rc;
use xrust::Node;
use xrust::parser::xml;
use xrust::trees::smite::Node as SmiteNode;

#[test]
fn serializer_issue_98() {
    /*
        Github issue number 98
        We wish to have XML documents output attributes in some stable order for test purposes.
        IMPORTANT NOTE: We will be stable for a particular version, but XML itself does not care
        about attribute order. We may switch the ordering between versions if we find a technical
        reason to do so.
    */

    let data = fs::read_to_string("tests/xml/issue-98.xml").unwrap();
    let source = Rc::new(SmiteNode::new());
    let parseresult = xml::parse(source.clone(), &data, None);
    let out1 = parseresult.unwrap().to_xml();

    let source2 = Rc::new(SmiteNode::new());
    let parseresult2 = xml::parse(source2.clone(), &data, None);
    let out2 = parseresult2.unwrap().to_xml();

    //println!("{:?}", out1);
    //println!("{:?}", out2);
    assert_eq!(out1, out2)

}
