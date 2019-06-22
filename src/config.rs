use super::osgood_v8::wrapper::*;
use super::osgood_v8::V8;
use glob::Pattern;
use std::net::SocketAddr;

use path_clean::clean;
use std::path::PathBuf;

static CONFIG_BOOTSTRAP: &str = include_str!("../js/config_bootstrap.js");

pub struct Policy {
    method: std::string::String,
    pattern: Pattern,
}

impl Policy {
    fn new(mut v8_policy: Local<V8::Object>, context: Local<V8::Context>) -> Policy {
        let method = v8_policy.get(context, "method").as_rust_string();
        let pattern = v8_policy.get(context, "pattern").as_rust_string();
        Policy {
            method,
            pattern: Pattern::new(&pattern).unwrap(),
        }
    }

    pub fn matches(&self, method: &str, url: &str) -> bool {
        self.method == method && self.pattern.matches(url)
    }
}

pub struct ConfigRoute {
    pub method: std::string::String,
    pub pattern: Pattern,
    pub worker_file: std::string::String,
    pub policies: Vec<Policy>,
    pub raw: std::string::String,
}

impl ConfigRoute {
    fn new(
        mut v8_route: Local<V8::Object>,
        context: Local<V8::Context>,
        worker_base: &PathBuf,
    ) -> ConfigRoute {
        let method = v8_route.get(context, "method").as_rust_string();
        let pattern = v8_route.get(context, "pattern").as_rust_string();
        let raw_pattern = v8_route.get(context, "rawPattern").as_rust_string();
        let worker_file = v8_route.get(context, "file").as_rust_string();
        let worker_file = get_absolute(worker_base, &PathBuf::from(worker_file))
            .to_string_lossy()
            .into();

        let mut policies = Vec::new();
        for (_, v8_policy) in v8_route.get(context, "policies").to_object().iter(context) {
            policies.push(Policy::new(v8_policy.to_object(), context));
        }

        ConfigRoute {
            method,
            pattern: Pattern::new(&pattern).unwrap(),
            worker_file,
            policies,
            raw: raw_pattern,
        }
    }
}

#[derive(Clone)]
pub struct ConfigStaticRoute {
    pub route_prefix: std::string::String,
    pub directory: std::string::String,
    pub index: Option<std::string::String>,
    pub clean_html_ext: bool,
}

impl ConfigStaticRoute {
    pub fn new(
        mut obj: Local<V8::Object>,
        context: Local<Context>,
        worker_base: &PathBuf,
    ) -> ConfigStaticRoute {
        let route_prefix = obj.get(context, "routePrefix").as_rust_string();

        let directory = obj.get(context, "directory").as_rust_string();
        let directory = get_absolute(&worker_base, &PathBuf::from(directory));
        // PathBuf is needed for getting absolute, but hyper_staticfile expects a
        // String, hence this slightly complex conversion
        let directory = std::string::String::from(directory.to_str().unwrap());

        let mut options = obj.get(context, "options").to_object();
        let index_type = options.get(context, "index").type_of();

        let mut index;

        match index_type {
            TypeofTypes::Boolean => {
                let index_value = options.get(context, "index").as_rust_bool(context);
                if index_value {
                    index = Some(std::string::String::from("index.html"));
                } else {
                    index = None;
                }
            }
            TypeofTypes::String => {
                let index_value = options.get(context, "index").as_rust_string();
                index = Some(index_value);
            }
            _ => index = Some(std::string::String::from("index.html")),
        }

        let clean_html_ext = options.get(context, "cleanUrls").as_rust_bool(context);

        ConfigStaticRoute {
            route_prefix,
            directory,
            index,
            clean_html_ext,
        }
    }
}

// This struct may seem out of place or unnecessary, but it's part of a future plan to support
// multiple hostnames (think vhosts) in a single app.
pub struct ConfigOrigin {
    pub origin: std::string::String,
    pub routes: Vec<ConfigRoute>,
    pub static_routes: Vec<ConfigStaticRoute>,
}

impl ConfigOrigin {
    fn new(
        mut v8_origin: Local<V8::Object>,
        context: Local<V8::Context>,
        port: &str,
        worker_base: &PathBuf,
    ) -> ConfigOrigin {
        // TODO: Need to implement the HTTPS case
        let host = v8_origin.get(context, "host").as_rust_string();

        let mut origin = format!("http://{}", host);

        if port != "80" {
            origin = format!("{}:{}", origin, port);
        }

        let mut routes = Vec::new();
        for (_, v8_route) in v8_origin.get(context, "routes").to_object().iter(context) {
            routes.push(ConfigRoute::new(v8_route.to_object(), context, worker_base));
        }

        let mut static_routes = Vec::new();
        for (_, v8_static_route) in v8_origin
            .get(context, "staticRoutes")
            .to_object()
            .iter(context)
        {
            static_routes.push(ConfigStaticRoute::new(
                v8_static_route.to_object(),
                context,
                worker_base,
            ));
        }

        ConfigOrigin {
            origin,
            routes,
            static_routes,
        }
    }
}

pub struct Config {
    pub origin: ConfigOrigin, // We'll need a Vec of these eventually.
    pub addr: SocketAddr,
}

impl Config {
    pub fn new(filename: &str) -> Result<Config, std::string::String> {
        let file_path = get_absolute(&std::env::current_dir().unwrap(), &PathBuf::from(filename));
        let config_js = std::fs::read_to_string(&file_path).expect("Bad config file");
        let worker_base = PathBuf::from(&file_path.parent().unwrap());
        let mut origin = None;
        let mut addr = Err(std::string::String::from("addr not yet retrieved"));
        v8_simple_init!(|mut context: Local<Context>| {
            let src = v8_str!(CONFIG_BOOTSTRAP);

            let mut global = context.global();
            global.set_extern_method(context, "_log", log);
            global.set_extern_method(context, "_error", error);

            Script::compile(context, *src)
                .unwrap()
                .run(context)
                .unwrap();
            let src = v8_str!(&config_js);
            addr = match Script::compile(context, *src) {
                Err(e) => Err(e),
                Ok(mut script) => match script.run(context) {
                    Err(e) => Err(e),
                    Ok(_) => {
                        let mut v8_config = global.get(context, "app").to_object();
                        let port = match std::env::var("PORT") {
                            Ok(port) => port,
                            Err(_) => v8_config.get(context, "port").as_rust_string(),
                        };
                        let interface = v8_config.get(context, "interface").as_rust_string();

                        origin = Some(ConfigOrigin::new(v8_config, context, &port, &worker_base));

                        match port.parse::<u16>() {
                            Ok(port) => format!("{}:{}", interface, port)
                                .parse()
                                .map_err(|e| format!("{}", e)),
                            Err(_err) => Err(format!(
                                "Expected port to be a number between 1 and 65535, received {}",
                                port
                            )),
                        }
                    }
                },
            }
        });

        match addr {
            Ok(addr) => match origin {
                Some(origin) => Ok(Config { origin, addr }),
                None => Err(std::string::String::from("origin was not populated!")),
            },
            Err(err) => Err(err),
        }
    }
}

fn get_absolute(base: &PathBuf, path: &PathBuf) -> PathBuf {
    if path.is_absolute() {
        path.clone()
    } else {
        PathBuf::from(clean(base.join(path).to_str().unwrap()))
    }
}

#[v8_fn]
fn log(args: FunctionCallbackInfo) {
    println!(
        "[{}] {}",
        ansi_term::Colour::Blue.paint("APP"),
        args.get(0).unwrap().as_rust_string()
    );
}

#[v8_fn]
fn error(args: FunctionCallbackInfo) {
    eprintln!(
        "[{}] {}",
        ansi_term::Colour::Blue.paint("APP"),
        args.get(0).unwrap().as_rust_string()
    );
}
