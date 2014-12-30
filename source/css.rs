
//! A simple parser for a tiny subset of CSS.
//!
//! To support more CSS syntax, it would probably be easiest to replace this
//! hand-rolled parser with one based on a library or parser generator.

use std::ascii::OwnedAsciiExt; // for `into_ascii_lowercase`
use std::str::FromStr;
use std::num::FromStrRadix;

/*
A simple selector can include a tag name, an ID prefixed by '#', any number of class names prefixed by '.', or some combination of the above. If the tag name is empty or '*' then it is a “universal selector” that can match any tag.
*/

// A CSS stylesheet is a series of rules
struct Stylesheet {
	rules: Vec<Rule>,
}

/*
A rule includes one or more selectors separated by commas, 
followed by a series of declarations enclosed in braces
 */
struct Rule {
	selectors: Vec<Selector>,
	declarations: Vec<Declaration>,
}

/*
A selector can be a simple selector or it can be a chain of 
selectors joined by combinators. Robinson supports only simple
selectors for now.
 */
enum Selector {
	Simple(SimpleSelector),
}

struct SimpleSelector {
	tag_name: Option<String>,
	id: Option<String>,
	class: Vec<String>,
}

/*
A declaration is just a name/value pair separated by a colon
and ending with a semicolon. For example, "margin: auto;" is a
declaration.
 */
struct Declaration {
	name: String,
	value: Value,
}

/*
This engine only supports a small handful of CSS value types
 */
enum Value {
	Keyword(String),
	Length(f32, Unit),
	ColorValue(Color),
	// insert more values here
}

enum Unit {
	Px,
	// insert more units here
}

struct Color {
	r: u8,
	g: u8,
	b: u8,
	a: u8
}

// Rust note: u8 is an 8-bit unsigned integer and
// f32 is a 32-bit float

/*
This project uses a very simplistic parser built the
same way as the HTML parser was. 
 */

/// Parse one simple selector, e.g: `type#id.class1.class2`
fn parse_simple_selector(&mut self) -> SimpleSelector {
    let mut selector = SimpleSelector { tag_name: None, id: None, class: Vec::new() };
    while !self.eof() {
    	match self.next_char() {
    		'#' => {
    			self.consume_char();
    			selector.id = Some(self.parse_identifier());
    		}
    		'.' => {
    			self.consume_char();
    			selector.class.push(self.parse_identifier());
    		}
    		'*' => {
    			// universal selector
    			self.consume_char();
    		}
    		c if valid_identifier_char(c) => {
    			selector.tag_name = Some(self.parse_identifier());
    		}
    		_ => break
    	}
    }
    return selector;
}

/*
There's a lack of error checking. Some malformed input like ### or *foo* successfully and produce weird results. A real CSS parser would discard selectors.

Specificity is one of the ways a rendering engine decides which style overrides the other in a conflict. If a stylesheet contains two rules that match an element, the rule with the matching selector of higher specificity can override values from the one with lower specificity.

The specificity of a selector is based on its components. An ID selector is more specific than a class selector, which is more specific than a tag selector. Within each of these "levels" more selectors beats fewer.
 */

pub type Specificity = (uint, uint, uint);

impl Selector {
	pub fn specificity(&self) -> Specificity {
		 // http://www.w3.org/TR/selectors/#specificity
		 let Selector::Simple(ref simple) = *self;
		 let a = simple.id.iter().len();
		 let b = simple.class.len();
		 let c = simple.tag_name.iter().len();
		 (a, b, c)
	}
}

/*
The selectors for each rule are stored in a sorted vector, most-specific first. This will be important in matching
 */

/// Parse a rule set: `<selectors> { <declarations> }`
fn parse_rule(&mut self) -> Rule {
	Rule {
		selectors: self.parse_selectors(),
		declarations: self.parse_declarations()
	}
}

// Parse a comma-separated list of selectors.
fn parse_selectors(&mut self) -> Vec<Selector> {
	let mut selectors = Vec::new();
	loop {
	  selectors.push(Selector::Simple(self.parse_simple_selector()));
	  self.consume_whitespace();
	  match self.next_char() {
	  	',' => { self.consume_char(); self.consume_whitespace(); }
	  	'{' => break, // start of declarations
	  	c => panic!("Unexpected character {} in selector list", c)
	  }
	}

	// Return selectors with highest specificity first, for use in matching
	selectors.sort_by(|a,b| b.specificity().cmp(&a.specificity()));
	return selectors;
}
































