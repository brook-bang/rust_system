#[derive(PartialEq,Debug)]
pub enum ContentType {
    Literal(String),
    TemplateVariable(ExpressionData),
    Tag(TagType),
    Unrecognized,
}

#[derive(PartialEq,Debug)]
pub enum TagType {
    ForTag,
    IfTag,
}

#[derive(PartialEq,Debug)]
pub struct ExpressionData {
    pub head: Option<String>,
    pub variable: String,
    pub tail: Option<String>
}

pub fn check_symbol_string(input: &str,symbol: &str) -> bool {
    input.contains(symbol)
}

pub fn check_matching_pair(input: &str,symbol1: &str,symbol2: &str) -> bool {
    input.contains(symbol1) && input.contains(symbol2)
}

pub fn get_index_for_symbol(input: &str,symbol:char) -> (bool,usize) {
    let mut characters = input.char_indices();
    let mut does_exist = false;
    let mut index = 0;

    while let Some((c,d)) = characters.next() {
        if d == symbol {
            does_exist = true;
            index = c;
            break;
        }
    }
    (does_exist,index)
}

pub fn get_content_type(input_line: &str) -> ContentType {
    let is_tag_expression = check_matching_pair(&input_line, "{%", "%}");
    let is_for_tag = (
        check_symbol_string(&input_line, "for")
        && check_symbol_string(&input_line, "in")
    ) || check_symbol_string(&input_line, "endfor");

    let is_if_tag = check_symbol_string(&input_line, "if") || check_symbol_string(&input_line, "endif");
    let is_template_variable = check_matching_pair(&input_line, "{{", "}}");

    




}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_literal_test() {
        let s = "<h1>Hello world<h2>";
        assert_eq!(ContentType::Literal(s.to_string()),get_content_type(s));
    }
    




}