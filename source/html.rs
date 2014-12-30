//! A simple parser for a tiny subset of HTML.
//!
//! Can parse basic opening and closing tags, and text nodes.
//!
//! Not yet supported:
//!
//! * Comments
//! * Doctypes and processing instructions
//! * Self-closing tags
//! * Non-well-formed markup
//! * Character entities

use dom;
use std::collections::HashMap;

/*
The parser stores its input string and a 
current position within the string. The
position is the index of the next character
we haven't processed yet.
 */
struct Parser {
	pos: uint,
	input: String,
}

/*
We can use this to implement simple methods 
for peeking at the next characters in the input
*/
impl Parser {
	/// Read the next character without consuming it
	fn next_char(&self) -> char {
		self.input.as_slice().char_at(self.pos)
	}

	/// Do the next characters start with the given string?
	fn starts_with(&self, s: &str) -> bool {
		self.input.as_slice().slice_from(self.pos).starts_with(s)
	}

	/// Return true if all input as consumed
	fn eof(&self) -> bool {
		self.pos >= self.input.len()
	}

	// ...
}

/*
Rust strings are stored as UTF-8 byte arrays. To go to
the next character we can't just advance byte by byte.
Instead, we use char_range_at which correctly handles
multi-byte characters. If our string used fixed-width
characters we could just increment pos by 1.
 */

/// Return the current character and advance to the next character
fn consume_char(&mut self) -> char {
	let range = self.input.as_slice().char_range_at(self.pos);
	self.pos = range.next;
	return range.ch;
}

/*
Often we want to consume a string of consecutive characters.
The consume_while method consumes characters that meet a given
condition and return them as a string.
 */

/// Consume characters until `test` returns false
fn consume_while(&mut self, test: |char| -> bool) -> String {
	let mut result = String::new();
	while !self.eof() && test(self.next_char()) {
		result.push(self.consume_char())
	}
	return result;
}

/*
We can use this to igore a sequence of space characters or to
consume a string of alphanumeric characters
 */

/// Consume and discard zero or more whitespace characters.
fn consume_whitespace(&mut self) {
	self.consume_while(|c| c.is_whitespace());
}

/// Parse a tag or attribute name
fn parse_tag_name(&mut self) -> String {
	self.consume_while(|c| match c {
		'a'...'z' | 'A'...'Z' | '0'...'9' => true,
		_ => false
	})
}

/// Now we're ready to start parsing HTML. To parse a single node
/// we look at its first character t see if it is an element or a
/// text node. In our simplified version of HTML, a text node can
/// contain any character except <.

/// Parse a single node.
fn parse_node(&mut self) -> dom::Node {
	match self.next_char() {
		'<' => self.parse_element(),
		_   => self.parse_text()
	}
}

/// Parse a text node.
fn parse_text(&mut self) -> dom::Node {
	dom::text(self.consume_while(|c| c != '<'))
}

/*
An element is more complicated. It includes opening and closing tags and between them any
number of child nodes
 */

/// Parse a single element, incluidng its open
/// tag, contents and closing tag
fn parse_element(&mut self) -> dom::Node {
	// Opening tag
	assert!(self.consume_char() == '<');
	let tag_name = self.parse_tag_name();
	let attrs = self.parse_attributes();
	assert!(self.consume_char() == '>');

	// Contents
	let children = self.parse_nodes();

	// Closing tag
	assert!(self.consume_char() == '<');
	assert!(self.consume_char() == '/');
	assert!(self.parse_tag_name() == tag_name);
	assert!(self.consume_char() == '>');

	return dom::elem(tag_name, attrs, children);
}

/*
Parsing attributes is quite easy with our simplified syntax. Until we reach the end of the opening tag (>) we repeatedly look for a name, followed by = and then a string enclosed in quotes.
 */

/// Parse a single name="value" pair
fn parse_attr(&mut self) -> (String, String) {
	let name = self.parse_tag_name();
	assert!(self.consume_char() == '=');
	let value = self.parse_attr_value();
	return (name, value);
}

/// Parse a quoted value
fn parse_attr_value(&mut self) -> String {
	let open_quote = self.consume_char();
	assert!(open_quote == '"' || open_quote == '\');
	let value = self.consume_while(|c| c != open_quote);
	return value;
}

/// Parse a list of name="value" pairs, separated by whitespace.
fn parse_attributes(&mut self) -> dom::AttrMap {
	let mut attributes = HashMap::new();
	loop {
		self.consume_whitespace();
		if self.next_char() == '>' {
			break;
		}
		let (name, value) = self.parse_attr();
		attributes.insert(name, value);
	}
	return attributes;
}

/*
To parse the child nodes, we recursively call parse_node in a loop until we reach the closing tag
 */

/// Parse a sequence of sibling nodes
fn parse_nodes(&mut self) -> Vec<dom::Node> {
	let mut nodes = Vec::new();
	loop {
		self.consume_whitespace();
		if self.eof() || self.starts_with("</") {
			break;
		}
		nodes.push(self.parse_node());
	}
	return nodes;
}

/*
Finally, we can put this all together to parse an entire HTML document into a DOM tree. This function will create a root node for the document if it doesn't include one explicitly. This is similar to what a real HTML parser does.
 */

/// Parse an HTML document and return the root element
pub fn parse(source: String) -> dom::Node {
	let mut nodes = Parser { pos:0, input: source}.parse_nodes();

	// If the document contains a root element, return it. Otherwise create one.
	if nodes.len() == 1 {
		nodes.swap_remove(0).unwrap();
	} else {
		dom::elem("html".to_string(), HashMap::new(), nodes);
	}
}






