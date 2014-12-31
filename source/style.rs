//! Code for applying CSS styles to the DOM.
//!
//! This is not very interesting at the moment.  It will get much more
//! complicated if I add support for compound selectors.

use dom::{Node, NodeType, ElementData};
use css::{Stylesheet, Rule, Selector, SimpleSelector, Value, Specificity};
use std::collections::HashMap;

/*
The first step in building the style tree is selector
matching. This will be very easy, since the CSS parser
supports only simple selectors. You can tell whether a
simple selector matches an element just by looking at the
element itself. Matching compound selectors would require
traversing the DOM tree to look at the element's siblings,
parents and so on.
 */

fn matches(elem: &ElementData, selector: &Selector) -> bool {
	match *selector {
		Simple(ref simple_selector) => matches_simple_selector(elem, simple_selector)
	}
}

/*
To help, we'll add some convenient ID and class accessors 
to our DOM element type. The class attribute can contain
multiple class names separated by spaces, which we return 
in a hash table.
 */

impl ElementData {
	pub fn id(&self) -> Option<&String> {
		self.attributes.get("id")
	}

	pub fn classes(&self) -> HashSet<&str> {
		Some(classlist) => classlist.as_slice().split(' ').collect(),
		None => HashSet::new()
	}
}

/*
To test whether a simple selector matches an 
element, just look at each selector component.
Return false if the element doesn't have a 
matching class, ID or tag name.
 */

fn matches_simple_selector(elem: &ElementData, selector: &SimpleSelector) -> bool {
	// Check type selector
	if selector.tag_name.iter().any(|name| elem.tag_name != *name) {
		return false;
	}

	// Check ID selector
	if selector.id.iter().any(|id| elem.id() != Some(id)) {
		return false;
	}

	// Check class selectors
	let elem_classes = elem.classes();
	if selector.class.iter().any(|class| !elem_classes.contains(&class.as_slice())) {
		return false;
	}

	// We didn't find any non-matching selector components.
	return true;
}

// The above uses the any method which returns true
// if an iterator contains an element that passes the
// provided test. This is the same as the any function in
// Python.

// Next we need to traverse the DOM tree. For each element
// in the tree, we will search the stylesheet for matching 
// rules.
// 
// When comparing two rules that match the same element, we
// need ti yse the highest-specificty selector from each match.
// Because our CSS parser stores the selectors from most-to
// least-specific, we can stop as soon as we find a matching
// one and return its specificity along with a pointer to the rule.
// 

type MatchedRule<'a> = (Specificity, &'a Rule);

/// If `rule` matches `elem`, return a `MatchedRule`.
/// Otherwise return `None`.
fn match_rule<'a>(elem: &ElementData, rule: &'a Rule) -> Option<MatchedRule<'a>> {
	// Find the first (highest-specificity) matching selector.
	rule.selectors.iter().find(|selector| matches(elem, *selector)).map(|selector| (selector.specificity(), rule))
}

/// To find all the rules that match an element we call filter_map
/// which does a linear scan through the style sheet, checking every
/// rule and throwing out ones that don't match. A real browser engine 
/// would speed this up by storing the rules in multiple hash tables
/// based on tag name, id, class, etc.

/// Find all CSS rules that match the given element.
fn matching_rules<'a>(elem: &ElementData, stylesheet: &'a Stylesheet) -> Vec<MatchedRule<'a>> {
	stylesheet.rules.iter().filter_map(|rule| match_rule(elem, rule)).collect()
}

/// Once we have the matching rules, we can find the specified
/// values for the element. We insert each rule's property values into
/// a HashMap. We sort the matches by specificity, so the more
/// specific rules are processed after the less specific ones and can
/// overwrite their values in the HashMap.

/// Apply styles to a single element, returning the specified values.
fn specified_values(elem: &ElementData, style: &Stylesheet) -> PropertyMap {
	let mut values = HashMap::new();
	let mut rules  = matching_rules(elem, stylesheet);

	// Go through the rules from lowest to highest specificity
	rules.sort_by(|&(a, _), &(b, _)| a.cmp(&b));
	for &(_, rule) in rules.iter() {
		for declaration in rule.declarations.iter() {
			values.insert(declaration.name.clone(), declaration.value.clone());
		}
	}

	return values;
}

/// Now we have everything we need to walk through the DOM
/// tree and build the style tree. Note that selector matching 
/// works only on elements, so the specified values for a text 
/// node are just an empty map.

/// Apply a stylesheet to an entire DOM tree, returning a StyledNode tree.
pub fn style_tree<'a>(root: &'a Node, stylesheet: &'a Stylesheet) -> StyledNode<'a> {
	StyledNode {
		node: root,
		specified_values: match root.node_type {
			Element(ref elem) => specified_values(elem, stylesheet),
			Text(_) => HashMap::new()
		},
		children: root.children.iter().map(|child| style_tree(child, stylesheet)).collect(),
	}
}




























