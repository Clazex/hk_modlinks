/// Find a suitable counterpart for mod name that behaves well when
/// using as filesystem path, URL path and part of XML. Does not promise
/// absolute uniqueness, but should be sufficiently unique.
///
/// Steps in detail:
/// 1. Split the string into parts by characters: `*`, `?`, `:`, `<`, `>`, `"`,
/// `/`, `|`, `\`, `\t` (tab), ` ` (space), `[` and `]`, empty parts are ignored
/// throughout the process;
/// 2. Remove all characters that are neither ascii-alphanumeric nor `.`, `-`, `_`;
/// 4. Make first character in each part uppercase;
/// 5. Join the parts together.
pub fn get_safe_mod_name(name: &str) -> String {
    name.split([
        ' ', // Not good for URI
        '[', ']', // Not good for XML CDATA
    ])
    .filter(|s| !s.is_empty())
    .map(|d| {
        d.replace(
            |c: char| !matches!(c, '.' | '-' | '_') && !c.is_ascii_alphanumeric(),
            "",
        )
    })
    .filter(|s| !s.is_empty())
    .map(|mut s| {
        s[0..1].make_ascii_uppercase();
        s
    })
    .collect::<String>()
}
