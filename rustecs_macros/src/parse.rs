use syntax::ast;
use syntax::ext::base::ExtCtxt;
use syntax::parse;
use syntax::parse::common::seq_sep_trailing_allowed;
use syntax::parse::parser::Parser;
use syntax::parse::token;
use syntax::ptr::P;


pub fn parse(context: &ExtCtxt, token_tree: &[ast::TokenTree]) -> World {
	let mut parser = parse::new_parser_from_tts(
		context.parse_sess(),
		context.cfg(),
		token_tree.to_vec()
	);

	World::parse(&mut parser)
}


pub struct World {
	pub components         : Vec<ast::Ident>,
	pub entity_constructors: Vec<EntityConstructor>,
}

impl World {
	fn parse(parser: &mut Parser) -> World {
		let mut components = Vec::new();
		let mut entities   = Vec::new();

		loop {
			let declaration = parser.parse_ident();
			match declaration.as_str() {
				"components" => {
					loop {
						components.push(parser.parse_ident());

						parser.eat(&token::COMMA);
						if parser.eat(&token::SEMI) {
							break;
						}
					}
				},

				"entity_constructor" =>
					entities.push(EntityConstructor::parse(parser)),

				_ =>
					parser.fatal(
						format!(
							"Unexpected declaration: {}",
							declaration.as_str(),
						)
						.as_slice()
					)
			}

			if parser.eat(&token::EOF) {
				break;
			}
		}

		World {
			components         : components,
			entity_constructors: entities,
		}
	}
}


pub struct EntityConstructor {
	pub name       : ast::Ident,
	pub components : Vec<ast::Ident>,
	pub args       : Vec<ast::Arg>,
	pub constr_impl: ConstructorImpl,
}

impl EntityConstructor {
	fn parse(parser: &mut Parser) -> EntityConstructor {
		let name = parser.parse_ident();

		let args = parser.parse_unspanned_seq(
			&token::LPAREN,
			&token::RPAREN,
			seq_sep_trailing_allowed(token::COMMA),
			|p| p.parse_arg());

		parser.expect(&token::RARROW);

		let components = parser.parse_unspanned_seq(
			&token::LPAREN,
			&token::RPAREN,
			seq_sep_trailing_allowed(token::COMMA),
			|p| p.parse_ident());


		let constructor_impl = if parser.eat(&token::EQ) {
			let constructor_fn_name = parser.parse_ident();
			parser.expect(&token::SEMI);

			External(constructor_fn_name)
		}
		else {
			Inline(parser.parse_block())
		};

		EntityConstructor {
			name       : name,
			components : components,
			args       : args,
			constr_impl: constructor_impl,
		}
	}
}

pub enum ConstructorImpl {
	Inline(P<ast::Block>),
	External(ast::Ident),
}
