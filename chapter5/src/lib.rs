use std::collections::HashMap;


#[derive(PartialEq,Debug)]
pub enum ContentType {
    Literal(String),
    TemplateVariable(ExpressionData),
    Tag(TagType),
    Unrecognized,
}

#[derive(PartialEq,Debug,Clone)]
pub struct ExpressionData{
    pub expression: String,
    pub var_map: Vec<String>,
    pub gen_html: String,
}

#[derive(PartialEq,Debug)]
pub enum TagType {
    ForTag,
    IfTag,
}

pub fn check_symbol_string(input: &str,symbol:&str) -> bool {
    let symbols = input.chars();
    symbols.as_str().contains(symbol)
}

pub fn check_matching_pair(input: &str,symbol1:&str,symbol2:&str) -> bool {
    let input_iter = input.chars();
    input_iter.as_str().contains(symbol1) && input_iter.as_str().contains(symbol2)
}

pub fn get_index_for_symbol(input: &str, symbol:char) -> (bool,usize) {
    let mut characters = input.char_indices();
    let mut dose_exist = false;
    let mut index = 0;

    while let Some((c,d)) = characters.next() {
        if d == symbol {
            dose_exist = true;
            index = c;
            break;
        }
    }
    (dose_exist,index)
}

pub fn get_content_type(input_line: &str) -> ContentType {
    let is_tag_expression = check_matching_pair(&input_line, "{%", "%}");
    let is_for_tag = (
        check_symbol_string(&input_line, "for")
        && check_symbol_string(&input_line, "in")
    ) || check_symbol_string(&input_line, "endfor");

    let is_if_tag = 
    check_symbol_string(&input_line, "if") ||check_symbol_string(&input_line, "endif");

    let is_template_variable = check_matching_pair(&input_line, "{{", "}}");
    
    let return_val;

    if is_tag_expression && is_for_tag {
        return_val = ContentType::Tag(TagType::ForTag);
    } else if is_tag_expression && is_if_tag {
        return_val = ContentType::Tag(TagType::IfTag);
    } else if is_template_variable{
        let content = get_expression_data(&input_line);
        return_val = ContentType::TemplateVariable(content);
    } else if !is_tag_expression & !is_template_variable {
        return_val = ContentType::Literal(input_line.to_string());
    } else {
        return_val = ContentType::Unrecognized;
    }
    return_val
}

pub fn generate_html_template_var(
    content: &mut ExpressionData,
    context: HashMap<String,String>,
) -> &mut ExpressionData {
    
}

pub fn get_expression_data(input_line: &str) -> ExpressionData {
    let expression_iter = input_line.split_whitespace();
    let mut template_var_map: Vec<String> = vec![];

    for word in expression_iter {
        if check_symbol_string(word, "{{") && check_symbol_string(word, "}}"){
            template_var_map.push(word.to_string());
        }
    }

    ExpressionData {
        expression: input_line.into(),
        var_map: template_var_map,
        gen_html: "".into(),
    }
}








