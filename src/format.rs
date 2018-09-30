use askama::Template;

use protocol::{Arg, Request};

#[derive(Template)]
#[template(path = "request.html")]
struct RequestTemplate {
    name: String,
    ret: String,
    args: String,
}

pub fn format_request(request: &Request, protocol_name: &str) -> String {
    match request.request_type {
        None => {
            let template = RequestTemplate {
                name: format!("{}_{}", protocol_name, request.name),
                ret: format_return(&request),
                args: format_args(&request, protocol_name),
            };
            template.render().unwrap()
        }
        Some(ref request_type) => {
            if (request_type) == "destructor" {
                let template = RequestTemplate {
                    name: format!("{}_{}", protocol_name, request.name),
                    ret: "void ".to_string(),
                    args: this_arg(protocol_name),
                };
                template.render().unwrap()
            } else {
                panic!("Unknown request type in request: {}", request.name)
            }
        }
    }
}

fn format_return(request: &Request) -> String {
    let id = request.args.iter().find(|&x| x.arg_type == "new_id");
    match id {
        Some(arg) => format!("struct {} *", arg.interface.clone().unwrap()),
        None => "void ".to_string(),
    }
}

fn format_args(request: &Request, protocol_name: &str) -> String {
    let arg_strings = request
        .args
        .iter()
        .filter(|&a| a.arg_type != "new_id")
        .map(format_arg)
        .collect::<Vec<String>>();

    let mut default = vec![this_arg(protocol_name)];
    default.extend(arg_strings);
    default.join(", ")
}

fn format_arg(arg: &Arg) -> String {
    let arg_type = match arg.arg_type.as_ref() {
        "int" => "int32_t ".to_string(),
        "uint" => "uint32_t ".to_string(),
        "fixed" => "wl_fixed_t".to_string(),
        "string" => "const char *".to_string(),
        "object" => format!("struct {} *", arg.interface.clone().unwrap()),
        "array" => "struct wl_array *".to_string(),
        "fd" => "int32_t".to_string(),
        _ => {
            panic!("Unknown argument type");
        }
    };

    format!("{}{}", arg_type, arg.name)
}

fn this_arg(protocol_name: &str) -> String {
    format!("struct {} *{}", protocol_name, protocol_name)
}
