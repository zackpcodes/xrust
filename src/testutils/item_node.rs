#[macro_export]
macro_rules! item_node_tests (
    ( $x:expr, $y:expr ) => {

	#[test]
	fn node_push_content() {
	    let mut d = $x();
	    let n = d.new_element(QualifiedName::new(None, None, String::from("Test")))
		.expect("unable to create element node");
	    d.push(n)
		.expect("unable to add node");
	    assert_eq!(d.to_xml(), "<Test></Test>")
	}

	// This test expects the document to have a single toplevel element
	// TODO: filter nodes to get elements and check there is only one
	#[test]
	fn item_node_type() {
	    assert_eq!(
		$y(QualifiedName::new(None, None, String::from("Test")), Value::from("foobar")).node_type(),
		NodeType::Document
	    )
	}

	#[test]
	fn item_node_name() {
	    let d = $y(QualifiedName::new(None, None, String::from("Test")), Value::from("foobar"));
	    match d.child_iter().nth(0) {
		Some(c) => {
		    assert_eq!(c.node_type(), NodeType::Element);
		    assert_eq!(c.name().to_string(), "Test")
		}
		None => panic!("no toplevel element")
	    }
	}

	#[test]
	fn item_node_value() {
	    let d = $y(QualifiedName::new(None, None, String::from("Test")), Value::from("foobar"));
	    match d.child_iter().nth(0) {
		Some(c) => {
		    assert_eq!(c.node_type(), NodeType::Element);
		    assert_eq!(c.name().to_string(), "Test");
		    let mut it = c.child_iter();
		    match it.next() {
			Some(t) => {
			    assert_eq!(t.node_type(), NodeType::Text);
			    assert_eq!(t.value().to_string(), "foobar");
			    match it.next() {
				Some(_) => panic!("unexpected child node"),
				None => assert!(true)
			    }
			}
			None => panic!("root element does not have child node")
		    }
		}
		None => panic!("no toplevel element")
	    }
	}

	#[test]
	fn item_node_to_string_doc() {
	    let d = $y(QualifiedName::new(None, None, String::from("Test")), Value::from("foobar"));
	    assert_eq!(d.to_string(), "foobar")
	}

	#[test]
	fn item_node_to_xml_doc() {
	    let d = $y(QualifiedName::new(None, None, String::from("Test")), Value::from("foobar"));
	    assert_eq!(d.to_xml(), "<Test>foobar</Test>")
	}

	#[test]
	fn item_node_parent() {
	    let mut sd = $x();
	    let mut t = sd.new_element(QualifiedName::new(None, None, String::from("Test")))
		.expect("unable to create element");
	    sd.push(t.clone())
		.expect("unable to append child");
	    let mut l1 = sd.new_element(QualifiedName::new(None, None, String::from("Level-1")))
		.expect("unable to create element");
	    t.push(l1.clone())
		.expect("unable to append child");
	    let l2 = sd.new_element(QualifiedName::new(None, None, String::from("Level-2")))
		.expect("unable to create element");
	    l1.push(l2.clone())
		.expect("unable to append child");
	    assert_eq!(l2.parent().unwrap().name().to_string(), "Level-1")
	}

	#[test]
	fn item_node_ancestor() {
	    let mut sd = $x();
	    let mut t = sd.new_element(QualifiedName::new(None, None, String::from("Test")))
		.expect("unable to create element");
	    sd.push(t.clone())
		.expect("unable to append child");
	    let mut l1 = sd.new_element(QualifiedName::new(None, None, String::from("Level-1")))
		.expect("unable to create element");
	    t.push(l1.clone())
		.expect("unable to append child");
	    let mut l2 = sd.new_element(QualifiedName::new(None, None, String::from("Level-2")))
		.expect("unable to create element");
	    l1.push(l2.clone())
		.expect("unable to append child");
	    let leaf = sd.new_text(Value::from("leaf node"))
		.expect("unable to create text node");
	    l2.push(leaf.clone())
		.expect("unable to append child");
	    let mut aiter = leaf.ancestor_iter();
	    assert_eq!(aiter.next().unwrap().name().to_string(), "Level-2");
	    assert_eq!(aiter.next().unwrap().name().to_string(), "Level-1");
	    assert_eq!(aiter.next().unwrap().name().to_string(), "Test");
	    assert_eq!(aiter.next().unwrap().node_type(), NodeType::Document);
	    match aiter.next() {
		None => {},
		_ => panic!("iterator should have no more items")
	    }
	}

	#[test]
	fn item_node_children() {
	    let mut sd = $x();
	    let mut t = sd.new_element(QualifiedName::new(None, None, String::from("Test")))
		.expect("unable to create element");
	    sd.push(t.clone())
		.expect("unable to append child");
	    let mut l1 = sd.new_element(QualifiedName::new(None, None, String::from("Level-1")))
		.expect("unable to create element");
	    t.push(l1.clone())
		.expect("unable to append child");
	    let mut l2 = sd.new_element(QualifiedName::new(None, None, String::from("Level-2")))
		.expect("unable to create element");
	    t.push(l2.clone())
		.expect("unable to append child");
	    let leaf = sd.new_text(Value::from("leaf node"))
		.expect("unable to create text node");
	    t.push(leaf.clone())
		.expect("unable to append child");
	    let mut citer = t.child_iter();
	    assert_eq!(citer.next().unwrap().name().to_string(), "Level-1");
	    assert_eq!(citer.next().unwrap().name().to_string(), "Level-2");
	    assert_eq!(citer.next().unwrap().value().to_string(), "leaf node");
	    match citer.next() {
		None => {},
		_ => panic!("iterator should have no more items")
	    }
	}

	#[test]
	fn item_node_first_child() {
	    let mut sd = $x();
	    let mut t = sd.new_element(QualifiedName::new(None, None, String::from("Test")))
		.expect("unable to create element");
	    sd.push(t.clone())
		.expect("unable to append child");
	    let mut l1 = sd.new_element(QualifiedName::new(None, None, String::from("Level-1")))
		.expect("unable to create element");
	    t.push(l1.clone())
		.expect("unable to append child");
	    let mut l2 = sd.new_element(QualifiedName::new(None, None, String::from("Level-2")))
		.expect("unable to create element");
	    t.push(l2.clone())
		.expect("unable to append child");
	    let leaf = sd.new_text(Value::from("leaf node"))
		.expect("unable to create text node");
	    t.push(leaf.clone())
		.expect("unable to append child");
	    match t.first_child() {
		Some(f) => assert_eq!(f.name().to_string(), "Level-1"),
		None => panic!("no first child")
	    }
	}

	#[test]
	fn item_node_descend() {
	    let mut sd = $x();
	    let mut t = sd.new_element(QualifiedName::new(None, None, String::from("Test")))
		.expect("unable to create element");
	    sd.push(t.clone())
		.expect("unable to append child");
	    let mut l1a = sd.new_element(QualifiedName::new(None, None, String::from("A")))
		.expect("unable to create element");
	    t.push(l1a.clone())
		.expect("unable to append child");
	    let mut l1b = sd.new_element(QualifiedName::new(None, None, String::from("B")))
		.expect("unable to create element");
	    t.push(l1b.clone())
		.expect("unable to append child");

	    let mut l2aa = sd.new_element(QualifiedName::new(None, None, String::from("A")))
		.expect("unable to create element");
	    l1a.push(l2aa.clone())
		.expect("unable to append child");
	    l2aa.push(
		sd.new_text(Value::from("AA"))
		    .expect("unable to create text")
	    ).expect("unable to append text");
	    let mut l2ab = sd.new_element(QualifiedName::new(None, None, String::from("B")))
		.expect("unable to create element");
	    l1a.push(l2ab.clone())
		.expect("unable to append child");
	    l2ab.push(
		sd.new_text(Value::from("AB"))
		    .expect("unable to create text")
	    ).expect("unable to append text");

	    let mut l2ba = sd.new_element(QualifiedName::new(None, None, String::from("A")))
		.expect("unable to create element");
	    l1b.push(l2ba.clone())
		.expect("unable to append child");
	    l2ba.push(
		sd.new_text(Value::from("BA"))
		    .expect("unable to create text")
	    ).expect("unable to append text");
	    let mut l2bb = sd.new_element(QualifiedName::new(None, None, String::from("B")))
		.expect("unable to create element");
	    l1b.push(l2bb.clone())
		.expect("unable to append child");
	    l2bb.push(
		sd.new_text(Value::from("BB"))
		    .expect("unable to create text")
	    ).expect("unable to append text");

	    let mut diter = t.descend_iter();
	    assert_eq!(diter.next().unwrap().name().to_string(), "A");
	    assert_eq!(diter.next().unwrap().name().to_string(), "A");
	    assert_eq!(diter.next().unwrap().value().to_string(), "AA");
	    assert_eq!(diter.next().unwrap().name().to_string(), "B");
	    assert_eq!(diter.next().unwrap().value().to_string(), "AB");
	    assert_eq!(diter.next().unwrap().name().to_string(), "B");
	    assert_eq!(diter.next().unwrap().name().to_string(), "A");
	    assert_eq!(diter.next().unwrap().value().to_string(), "BA");
	    assert_eq!(diter.next().unwrap().name().to_string(), "B");
	    assert_eq!(diter.next().unwrap().value().to_string(), "BB");
	    match diter.next() {
		None => {},
		_ => panic!("iterator should have no more items")
	    }
	}

	#[test]
	fn item_node_next() {
	    let mut sd = $x();
	    let mut t = sd.new_element(QualifiedName::new(None, None, String::from("Test")))
		.expect("unable to create element");
	    sd.push(t.clone())
		.expect("unable to append child");
	    let mut l1 = sd.new_element(QualifiedName::new(None, None, String::from("Level-1")))
		.expect("unable to create element");
	    t.push(l1.clone())
		.expect("unable to append child");
	    let mut l2 = sd.new_element(QualifiedName::new(None, None, String::from("Level-2")))
		.expect("unable to create element");
	    t.push(l2.clone())
		.expect("unable to append child");
	    let leaf = sd.new_text(Value::from("leaf node"))
		.expect("unable to create text node");
	    t.push(leaf.clone())
		.expect("unable to append child");
	    let mut niter = l1.next_iter();
	    assert_eq!(niter.next().unwrap().name().to_string(), "Level-2");
	    assert_eq!(niter.next().unwrap().value().to_string(), "leaf node");
	    match niter.next() {
		None => {},
		_ => panic!("iterator should have no more items")
	    }
	}

	#[test]
	fn item_node_prev() {
	    let mut sd = $x();
	    let mut t = sd.new_element(QualifiedName::new(None, None, String::from("Test")))
		.expect("unable to create element");
	    sd.push(t.clone())
		.expect("unable to append child");
	    let mut l1 = sd.new_element(QualifiedName::new(None, None, String::from("Level-1")))
		.expect("unable to create element");
	    t.push(l1.clone())
		.expect("unable to append child");
	    let mut l2 = sd.new_element(QualifiedName::new(None, None, String::from("Level-2")))
		.expect("unable to create element");
	    t.push(l2.clone())
		.expect("unable to append child");
	    let leaf = sd.new_text(Value::from("leaf node"))
		.expect("unable to create text node");
	    t.push(leaf.clone())
		.expect("unable to append child");
	    let mut piter = leaf.prev_iter();
	    assert_eq!(piter.next().unwrap().name().to_string(), "Level-2");
	    assert_eq!(piter.next().unwrap().name().to_string(), "Level-1");
	    match piter.next() {
		None => {},
		_ => panic!("iterator should have no more items")
	    }
	}

	#[test]
	fn item_node_attr() {
	    let mut sd = $x();
	    let mut t = sd.new_element(QualifiedName::new(None, None, String::from("Test")))
		.expect("unable to create element");
	    sd.push(t.clone())
		.expect("unable to append child");
	    let a1 = sd.new_attribute(
		QualifiedName::new(None, None, String::from("role")),
		Value::from("testing")
	    ).expect("unable to create attribute");
	    t.add_attribute(a1)
		.expect("unable to add attribute");
	    let a2 = sd.new_attribute(
		QualifiedName::new(None, None, String::from("phase")),
		Value::from("one")
	    ).expect("unable to create element");
	    t.add_attribute(a2)
		.expect("unable to add attribute");

	    // NB. attributes could be returned in a different order
	    assert_eq!(sd.to_xml(), "<Test role='testing' phase='one'></Test>");
	    let mut aiter = t.attribute_iter();
	    assert_eq!(aiter.next().unwrap().name().to_string(), "role");
	    assert_eq!(aiter.next().unwrap().name().to_string(), "phase");
	    match aiter.next() {
		None => {},
		_ => panic!("iterator should have no more items")
	    }
	}
    }
);
