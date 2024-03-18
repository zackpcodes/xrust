use std::{env, fs};
use std::rc::Rc;
use url::Url;
use crate::{Document, Error, ErrorKind, Item, Sequence};
use crate::parser::xml;
use crate::transform::context::StaticContext;
use crate::validators::ValidationError;
use crate::xslt::from_document;
use crate::trees::smite::{Node as SmiteNode, Node, RNode};

fn make_from_str(s: &str) -> Result<RNode, Error> {
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), s, None, None);
    Ok(doc)
}

fn get_xml(u: &Url) -> Result<String, Error>{
    //gotta remove that leading "/" character that the URL library adds.
    let loc = if env::consts::OS == "windows" && u.path().starts_with("/") && !u.path().starts_with("//")  {
        let s = u.path()[1..].to_string();
        s
    } else {
        let s =  u.path().to_string();
        s
    };
    let f = fs::read_to_string(loc);
    Ok(f.unwrap())
}

pub fn validate_schematron(doc: &RNode, schema: &RNode) -> Result<(), ValidationError>  {
    /*
    This uses schxslt to validate documents.

    We first run the schema through pipeline-for-svrl.xsl, that generates the validator to
    run against the actual document, so XSLT will run twice.

    TODO add something to also validate the schematron schema file (so run 4 transforms!)

    TODO: There are two versions of schxslt available, depending on whether or not
        the schema binding using xslt or xslt2/xslt3. We will need to support both, ideally
        by checking outselves the root nodes "queryBinding" attribute before selecting the
        appropriate stylesheet.

     */
    let s = fs::read_to_string("src/validators/schematron/schxslt/1.0/compile-for-svrl.xsl").unwrap();
    let schxslt = Rc::new(SmiteNode::new());
    let _ = xml::parse(schxslt.clone(), s.as_str(), None, None);

    let schxsltbase = ["file://", std::env::current_dir().unwrap().to_str().unwrap(),"/src/validators/schematron/schxslt/1.0/"].join("");
    let schxsltbaseurl = Url::parse(schxsltbase.as_str()).expect("unable to parse URL");

    let mut ctxt = from_document(
        schxslt,
        Some(schxsltbaseurl.clone()),
        |s| make_from_str(s),
        |url| get_xml(url),
    ).expect("failed to compile stylesheet");

    ctxt.context(vec![Item::Node(schema.clone())], 0);
    let seq = ctxt.evaluate(&mut StaticContext::<Box<dyn FnMut(&str) -> Result<(), Error>>>::new()).expect("evaluation failed");

    println!("{:?}",seq);

    //Now we have compiled the schematron into a transform, we run that against the document.

    let compiled_ss = match seq.first().unwrap(){
        Item::Node(css) => {css.clone()}
        _ => {panic!("Schematron compilation should output a stylesheet!")}
    };

    let mut ctxt2 = from_document(
        compiled_ss,
        Some(schxsltbaseurl),
        |s| make_from_str(s),
        |url| get_xml(url),
    ).expect("failed to compile 2nd stylesheet");

    ctxt.context(vec![Item::Node(doc.clone())], 0);
    let seq = ctxt.evaluate(&mut StaticContext::<Box<dyn FnMut(&str) -> Result<(), Error>>>::new());

    //TODO check the output results to work out if the document was valid, then report an error type if not.

    Ok(())
}

