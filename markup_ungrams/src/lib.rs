//! Canonical model of markup supported by Zork

use ungrammar::{Error, Grammar};

pub struct ZorkKind<'a> {
    // alias -> canonical name
    pub codelangs_alias: &'a [(&'a str, &'a str)],
    pub punct_trans: &'a [(&'a str, &'a str)],
}

pub const ZORK_KIND: ZorkKind = ZorkKind {
    codelangs_alias: &[
        // alias       canon
        ("markdown", "markdown"),
        ("md", "markdown"),
        ("python", "python"),
        ("python3", "python"),
        ("py", "python"),
        ("py3", "python"),
        ("rust", "rust"),
        ("rs", "rust"),
        ("shell", "sh"),
        ("sh", "sh"),
        ("typescript", "typescript"),
        ("ts", "typescript"),
    ],

    punct_trans: &[
        // literal   variant
        ("b#", "BlockIdHashLead"),
        ("b$", "BlockContentHashlead"),
        ("#", "HeaderBlockLead"),
        ("|", "QuoteBlockLead"),
        (">", "QuoteBlocklead"),
        ("/", "AbsoluteLinuxPathLead"),
        ("./", "RelativeLinuxPathLead"),
    ],
};

pub fn zork_grammar() -> Result<Grammar, Error> {
    let grammar = include_str!("../zork_keg.ungram");

    grammar.parse::<Grammar>()
}

#[cfg(test)]
mod tests {
    use super::zork_grammar;
    #[test]
    fn markup_grammar_content() {
        insta::assert_debug_snapshot!(zork_grammar().unwrap());
    }
}
