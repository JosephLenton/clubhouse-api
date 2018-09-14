
use urls;

pub fn parse_text_for_ticket_story_urls( text : &str, org : &str ) -> Vec<String> {
  parse_text_for_items( text ).map(|code| urls::generate_url_story( org, code )).collect()
}

///
/// This is intended for parsing commit messages, or commit headers,
/// for a clubhouse ticket number.
///
/// Clubhouse tickets come in three forms ...
///
///  - #123 at the start of the message. Only at the start.
///  - []
pub fn parse_text_for_ticket_codes<'a>( text : &'a str ) -> Iterator<&'a str> {
}
