//! Basic DOM data structures.

use std::collections::{HashMap,HashSet};

/*
The DOM is a tree of nodes. Nodes have zero
or more children. They also have many other
attributes and methods, but we'll be ignoring
them. Below we represent children as a node
vector. Each Node has a Node type:
http://dom.spec.whatwg.org/#dom-node-nodetype
 */

#[deriving(Show)]
struct Node {
    // data common to all nodes:
    pub children: Vec<Node>,

    // data specific to each node type:
    pub node_type: NodeType,
}

/*
A Node is either an Element or a Text node. In 
a language with inheritance these would be subtypes
of Node. In Rust, they're a enum (tagged union).
 */
#[deriving(Show)]
enum NodeType {
    Element(ElementData),
    Text(String),
}

/*
An element includes a tag name and any number of
attributes which can be stored as a map from names
to values. As we don't support namespaces, just store
the tag and attribtue name as simple strings.
 */
struct ElementData {
    tag_name: String,
    attributes: AttrMap,
}

type AttrMap = HashMap<String, String>;

/*
Finally add some constructor functions to make it easy to create
new nodes.
 */

fn text(data: String) -> Node {
    Node { children: Vec::new(), node_type: NodeType::Text(data) }
}

fn elem(name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
    Node {
        children: children,
        node_type: NodeType::Element(ElementData {
            tag_name: name,
            attributes: attrs,
        })
    }
}