//! Canonical model of markup supported by Zork

use ungrammar::{Grammar, Error};

pub(crate) struct ZorkKind<'a> {
    // alias -> canonical name
    pub(crate) codelangs_alias: &'a [(&'a str, &'a str)],
    pub(crate) punct_trans: &'a [(&'a str, &'a str)],
}

pub(crate) const ZORK_KIND: ZorkKind = ZorkKind {
    codelangs_alias: &[
        // alias       canon
        ("markdown",   "markdown"),
        ("md",         "markdown"),

        ("python",     "python"),
        ("python3",    "python"),
        ("py",         "python"),
        ("py3",        "python"),

        ("rust",       "rust"),
        ("rs",         "rust"),

        ("shell",      "sh"),
        ("sh",         "sh"),

        ("typescript", "typescript"),
        ("ts",         "typescript"),
    ],

    punct_trans: &[
        // literal   variant
        ("b#",       "BlockIdHashLead"),
        ("b$",       "BlockContentHashlead"),

        ("#",        "HeaderBlockLead"),

        ("|",        "QuoteBlockLead"),
        (">",        "QuoteBlocklead"),

        ("/",        "AbsoluteLinuxPathLead"),
        ("./",       "RelativeLinuxPathLead"),
    ],
};

pub(crate) fn markup_grammar() -> Result<Grammar, Error> {
    let grammar = include_str!("../zork_keg.ungram");
    
    grammar.parse::<Grammar>()
}

#[cfg(test)]
mod tests {
    use super::markup_grammar;
    #[test]
    fn markup_grammar_content() {
        insta::assert_debug_snapshot!(markup_grammar().unwrap());
    }
}
