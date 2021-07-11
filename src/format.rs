use askama::Template;

use crate::protocol::{Arg, Request};
use crate::filters;

#[derive(Template)]
#[template(path = "request.html")]
struct RequestTemplate {
    name: String,
    ret: String,
    args: Vec<ArgTuple>,
}

type ArgTuple = (String, String, Option<String>);

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
                    args: [this_arg(protocol_name)].to_vec(),
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
        Some(Arg {
            interface: Some(iface),
            ..
        }) => format!("struct {}* ", iface),
        Some(_) => "void* ".to_string(),
        None => "void ".to_string(),
    }
}

fn format_args(request: &Request, protocol_name: &str) -> Vec<ArgTuple> {
    let arg_tuples = request
        .args
        .iter()
        .filter(|&a| a.arg_type != "new_id")
        .map(format_arg)
        .collect::<Vec<ArgTuple>>();

    let mut default = vec![this_arg(protocol_name)];
    default.extend(arg_tuples);
    default
}

pub fn format_arg(arg: &Arg) -> ArgTuple {
    let arg_type = match arg.arg_type.as_ref() {
        "int" => "int32_t".to_string(),
        "uint" => "uint32_t".to_string(),
        "fixed" => "wl_fixed_t".to_string(),
        "string" => "const char*".to_string(),
        "object" => format!("struct {}*", arg.interface.clone().unwrap_or("void".to_string())),
        "array" => "struct wl_array*".to_string(),
        "fd" => "int32_t".to_string(),
        "new_id" => format!("struct {}*", arg.interface.clone().unwrap_or("void".to_string())),
        t => {
            panic!("Unknown argument type: {}", t);
        }
    };
    (arg_type, arg.name.to_string(), arg.summary.clone())
}

fn this_arg(protocol_name: &str) -> ArgTuple {
    let arg_type = format!("struct {}*", protocol_name);
    (arg_type, protocol_name.to_string(), None)
}
