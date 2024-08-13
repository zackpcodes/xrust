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
    let mut prev_xml_output = None;

    for iteration in 0..100 {
        let doc = xml::parse(Rc::new(SmiteNode::new()), data.clone().as_str(), None).unwrap();
        let xml_output = doc.to_xml();

        if let Some(prev_xml_output) = &prev_xml_output {
            assert_eq!(
                &xml_output,
                prev_xml_output,
                "Failed on run {}", iteration
            );
        }
        prev_xml_output = Some(xml_output);
    }

}
