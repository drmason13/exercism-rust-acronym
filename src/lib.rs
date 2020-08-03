const DELIMITERS: &str = "- ,:_";

/// Takes a phrase and returns an acronym of letters that are at the start of words or are "Individually capitalised".
/// Examples
///
/// ```
/// assert_eq!(acronym::abbreviate("camelCase"), "CC");
/// assert_eq!(acronym::abbreviate("Looks good to Me"), "LGTM");
/// assert_eq!(acronym::abbreviate("Just flippin' do it!"), "JFDI");
/// assert_eq!(acronym::abbreviate("YAML ain't Markup Language"), "YAML");
/// ```
pub fn abbreviate(phrase: &str) -> String {
    std::iter::once('-') // add a leading delimiter (_bit_ of a cludge)
        .chain(phrase.chars())
        .collect::<Vec<_>>() // Collecting into a Vec needed to use windows()
        .windows(2) // we will compare each char with its previous
        .map(extract_acronym)
        .filter_map(|x| x) // unwrap Options, discarding Nones
        .collect::<String>()
}

/// Tests if a character is a delimiter and should be used to split a phrase into words
fn is_delimiter(test: char) -> bool {
    DELIMITERS.contains(test)
}

/// Takes a pair of chars and returns a Capital letter if it should form part of an acronym otherwise returns None.
fn extract_acronym(pair: &[char]) -> Option<char> {
    if is_delimiter(pair[0]) && pair[1].is_alphanumeric() {
        // the first letter after a delimiter should be part of the acronym
        // because it is the start of a word
        return Some(pair[1].to_uppercase().next().unwrap());
    }
    if pair[0].is_lowercase() && pair[1].is_uppercase() {
        // an uppercase letter preceeded by a lowercase letter should be part of the acronym
        // because it is "Individually Capitalised"
        return Some(pair[1]);
    }
    None
}
