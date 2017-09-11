mod post;

pub enum Template {
    Post,
}

pub fn get_template(template: Template) -> String {
    match template {
        _ => post::TEMPLATE.to_string(),
    }
}
