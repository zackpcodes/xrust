use std::rc::Rc;
use crate::value::Value;
use crate::item::{Node, NodeType};
use crate::xdmerror::{Error, ErrorKind};
use crate::parser::combinators::alt::{alt2, alt4};
use crate::parser::combinators::many::many0nsreset;
use crate::parser::combinators::map::map;
use crate::parser::combinators::opt::opt;
use crate::parser::combinators::tag::tag;
use crate::parser::combinators::tuple::{tuple10, tuple2, tuple5};
use crate::parser::combinators::wellformed::wellformed;
use crate::parser::combinators::whitespace::whitespace0;
use crate::parser::xml::attribute::attributes;
use crate::parser::xml::chardata::chardata;
use crate::parser::xml::misc::{comment, processing_instruction};
use crate::parser::xml::qname::qualname;
use crate::parser::xml::reference::reference;
use crate::parser::{ParseError, ParseInput};
use crate::qname::QualifiedName;
use crate::xmldecl::{AttType, DefaultDecl};

// Element ::= EmptyElemTag | STag content ETag
pub(crate) fn element<N: Node>() -> impl Fn(ParseInput<N>) -> Result<(ParseInput<N>, N), ParseError>
{
    move |input| match alt2(
        //Empty element
        map(tuple5(
            tag("<"),
            wellformed(qualname(), |qn| {
                qn.prefix_to_string() != Some("xmlns".to_string())
            }),
            attributes(), //many0(attribute),
            whitespace0(),
            tag("/>"),
        ),|(_, n, (at, ns), _, _)|{
            ((), n.clone(), (at, ns), (), (), vec![], (), n, (), ())
        }),
        //tagged element
        wellformed(
            tuple10(
                tag("<"),
                wellformed(qualname(), |qn| {
                    qn.prefix_to_string() != Some("xmlns".to_string())
                }),
                attributes(), //many0(attribute),
                whitespace0(),
                tag(">"),
                content(),
                tag("</"),
                wellformed(qualname(), |qn| {
                    qn.prefix_to_string() != Some("xmlns".to_string())
                }),
                whitespace0(),
                tag(">"),
            ),
            |(_, n, _a, _, _, _c, _, e, _, _)| n.to_string() == e.to_string(),
        ))(input){
        Err(err) => Err(err),
        Ok(((input1, mut state1), (_, mut n, (av, namespaces), _, _, c, _, _, _, _))) => {
            if n.resolve(|p| state1.namespace.get(&p).map_or(
                Err(Error::new(ErrorKind::DynamicAbsent, "no namespace for prefix")),
                |r| Ok(r.clone())
            )).is_err() {
                return Err(ParseError::MissingNameSpace);
            }
            let elementname = state1.get_qualified_name(n.namespace_uri(), n.prefix(), n.localname());
            if state1.xmlversion=="1.1"
                && elementname.namespace_uri_to_string() == Some("".to_string())
                && elementname.prefix_to_string().is_some(){
                return Err(ParseError::MissingNameSpace);
            }
            let d = state1.doc.clone().unwrap();
            let mut e = d
                .new_element(elementname)
                .expect("unable to create element");

            //Looking up the DTD, seeing if there are any attributes we should populate
            //Remember, DTDs don't have namespaces, you need to lookup based on prefix and local name!
            // We generate the attributes in two sweeps:
            // Once for attributes declared on the element and once for the DTD default attribute values.

            let attlist = state1.dtd.attlists.get(&QualifiedName::new_from_values(None, n.prefix(), n.localname()));

            match attlist {
                None => {
                    //No Attribute DTD, just insert all elements.
                    for (attname, attval) in av.into_iter(){
                        let a = d.new_attribute(
                            Rc::new(attname),
                            attval,
                            false,
                            false
                        ).expect("unable to create attribute");
                        e.add_attribute(a).expect("unable to add attribute")
                    }
                }
                Some(atts) => {
                    if state1.attr_defaults {
                        for (attname, (atttype, defdecl, _)) in atts.iter(){

                            match defdecl {
                                DefaultDecl::Default(s) | DefaultDecl::FIXED(s) => {
                                    let mut at = attname.clone();
                                    match at.prefix() {
                                        None => {}
                                        Some(_) => {
                                            if at.resolve(|p| state1.namespace.get(&p).map_or(
                                                Err(Error::new(ErrorKind::DynamicAbsent, "no namespace for prefix")),
                                                |r| Ok(r.clone())
                                            )).is_err() {
                                                return Err(ParseError::MissingNameSpace);
                                            }
                                        }
                                    }
                                    //Assign IDs only if we are tracking.
                                    let (attr_is_id, attr_is_idref) = match (atttype, state1.id_tracking) {
                                        (_, false) => (false, false),
                                        (AttType::ID, true) => (true, false),
                                        (AttType::IDREF, true) => (false, true),
                                        (AttType::IDREFS, true) => (false, true),
                                        (_, _) => (false, false),
                                    };
                                    let a = d.new_attribute(
                                        Rc::new(at),
                                        Rc::new(Value::String(s.clone())),
                                        attr_is_id,
                                        attr_is_idref
                                    ).expect("unable to create attribute");
                                    e.add_attribute(a).expect("unable to add attribute")
                                }
                                _ => {}
                            }
                        }
                    }

                    for (attname, attval) in av.into_iter() {
                        match atts.get(&QualifiedName::new(None, attname.prefix_to_string(), attname.localname_to_string())) {
                            None => {
                                let a = d.new_attribute(
                                    Rc::new(attname),
                                    attval,
                                    false,
                                    false
                                ).expect("unable to create attribute");
                                e.add_attribute(a).expect("unable to add attribute")
                            }
                            Some((atttype, _, _)) => {

                                //Assign IDs only if we are tracking.
                                let (attr_is_id, attr_is_idref) = match (atttype, state1.id_tracking) {
                                    (_, false) => (false, false),
                                    (AttType::ID, true) => (true, false),
                                    (AttType::IDREF, true) => (false, true),
                                    (AttType::IDREFS, true) => (false, true),
                                    (_, _) => (false, false),
                                };
                                let a = d.new_attribute(
                                    Rc::new(attname),
                                    attval,
                                    attr_is_id,
                                    attr_is_idref
                                ).expect("unable to create attribute");
                                e.add_attribute(a).expect("unable to add attribute")
                            }
                        }
                    }
                }
            }

            //we've added the IDs and IDRefs, but we need to track all that.
            if state1.id_tracking {
                for attribute in e.attribute_iter() {
                    if attribute.is_id(){
                        match state1.ids_read.insert(attribute.value().clone()){
                            true => {}
                            false => {
                                //Value already existed!
                                return Err(ParseError::IDError(String::from("Diplicate ID found")))
                            }
                        }
                    }
                    if attribute.is_idrefs(){
                        /*
                        If the IDRef matches a previously loaded ID, we're all good. If not, that ID
                        may exist further along, we'll make a note of it to check when we
                        have completely parsed the document.
                        */
                        for idref in attribute.value().to_string().split_whitespace(){
                            let idref_new = Rc::new(Value::String(idref.to_string()));
                            match state1.ids_read.get(&idref_new){
                                Some(_) => {}
                                None => {
                                    state1.ids_pending.insert(idref_new);
                                }
                            }
                        }
                    }
                }
            }

            namespaces.iter()
                .for_each(|b| e.add_namespace(b.clone()).expect("unable to add namespace"));
            // Add child nodes
            c.iter().for_each(|d| {
                e.push(d.clone()).expect("unable to add node");
            });
            Ok(((input1, state1.clone()), e))
        }
    }
}

// content ::= CharData? ((element | Reference | CDSect | PI | Comment) CharData?)*
pub(crate) fn content<N: Node>(
) -> impl Fn(ParseInput<N>) -> Result<(ParseInput<N>, Vec<N>), ParseError> {
    move |(input, state)| match tuple2(
        opt(chardata()),
        many0nsreset(tuple2(
            alt4(
                map(processing_instruction(), |e| vec![e]),
                map(comment(), |e| vec![e]),
                map(element(), |e| vec![e]),
                reference(),
            ),
            opt(chardata()),
        )),
    )((input, state.clone()))
    {
        Ok((state1, (c, v))) => {
            let mut new: Vec<N> = Vec::new();
            let mut notex: Vec<String> = Vec::new();
            if c.is_some() {
                notex.push(c.unwrap());
            }
            if !v.is_empty() {
                for (w, d) in v {
                    for x in w {
                        match x.node_type() {
                            NodeType::Text => notex.push(x.to_string()),
                            _ => {
                                if !notex.is_empty() {
                                    new.push(
                                        state
                                            .doc
                                            .clone()
                                            .unwrap()
                                            .new_text(Rc::new(Value::String(notex.concat())))
                                            .expect("unable to create text node"),
                                    );
                                    notex.clear();
                                }
                                new.push(x);
                            }
                        }
                    }
                    if d.is_some() {
                        notex.push(d.unwrap())
                    }
                }
            }
            if !notex.is_empty() {
                new.push(
                    state
                        .doc
                        .clone()
                        .unwrap()
                        .new_text(Rc::new(Value::String(notex.concat())))
                        .expect("unable to create text node"),
                );
            }
            Ok((state1, new))
        }

        Err(e) => Err(e),
    }
}
