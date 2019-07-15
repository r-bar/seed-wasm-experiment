use std::collections::{HashMap, HashSet};

//#[macro_use] extern crate log;
#[macro_use] extern crate seed;
use futures::Future;
use wasm_bindgen::prelude::*;
use web_sys::console;
use seed::prelude::*;
use seed::fetch;
use seed::dom_types::Tag;
use serde::{Serialize, Deserialize};


// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


// Model

#[derive(Clone, Debug)]
struct Model {
    count: usize,
    what_we_count: String,
    httpbin_data: Option<HttpbinAnythingData>,
}

// Setup a default here, for initialization later.
impl Default for Model {
    fn default() -> Self {
        Self {
            count: 500,
            what_we_count: "clicks".into(),
            httpbin_data: None,
        }
    }
}


// Update

fn fetch_data(model: &mut Model) -> impl Future<Item = Msg, Error = Msg> {
    //console::log_1(&JsValue::from_str("Fetch!"));
    let url = "https://httpbin.dev.fanai.io/anything/somepath?foo=bar";
    fetch::Request::new(url.into()).fetch_json(Msg::HttpbinAnythingDataFetched)
}


#[derive(Clone, Debug)]
enum Msg {
    Increment,
    Decrement,
    Fetch,
    HttpbinAnythingDataFetched(fetch::FetchObject<HttpbinAnythingData>),
    ChangeWWC(String),
    Log(String),
    NoOp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct HttpbinAnythingData {
    args: HashMap<String, String>,
    data: String,
    files: HashMap<String, String>,
    form: HashMap<String, String>,
    headers: HashMap<String, String>,
    json: Option<String>,
    method: String,
    origin: String,
    url: String,
}

/// How we update the model
fn update(msg: Msg, model: &mut Model, orders: &mut Orders<Msg>) {
    let delta = 100;
    // help witht the debug message, the message is moved during the update
    let msg_copy = msg.clone();
    match msg {
        Msg::Increment => model.count += delta,
        Msg::Decrement => {
            if delta > model.count {
                model.count = 0;
            } else {
                model.count -= delta
            }
        },
        Msg::ChangeWWC(what_we_count) => model.what_we_count = what_we_count,
        Msg::Fetch => {
            orders.skip().perform_cmd(fetch_data(model));
        },
        Msg::HttpbinAnythingDataFetched(fetch_object) => {
            match fetch_object.response() {
                Ok(response) => {
                    log!("fetch success");
                    //log!("fetch success!");
                    model.httpbin_data = Some(response.data);
                },
                Err(fail_reason) => error!("httpbin request failed: {:?}", fail_reason),
            }
        },
        Msg::Log(message) => log!("{}", message),
        Msg::NoOp => return,
    }
    log!(&msg_copy, &model);
}


// View

/// A simple component.
fn view_success_level(clicks: usize) -> El<Msg> {
    let descrip = match clicks / 100 {
        0 ... 5 => "Not very many 🙁",
        6 ... 9 => "I got my first real six-string 😐",
        10 ... 11 => "Spinal Tap 🙂",
        _ => "Double pendulum 🙃"
    };
    p![ descrip ]
}

fn view_things(count: usize, label: &str) -> Vec<El<Msg>> {
    //let mut output = Vec::with_capacity(count);
    (1 ..= count)
        .map(|i| div![
             class!["thing"],
             format!("{} {}", label, i),
        ])
        .collect()
}

fn view_httpbin_data(data: &Option<HttpbinAnythingData>) -> El<Msg> {
    match data {
        Some(httpbin_data) => {
            let pretty_json =  serde_json::to_string_pretty(&httpbin_data).unwrap();
            pre![ code![ pretty_json ] ]
        },
        None => empty![],
    }
}

fn view_what_we_count(what_we_count: &str) -> Vec<El<Msg>> {
    vec![
        h3![ "What are we counting?" ],
        input![ attrs!{At::Value => what_we_count}, input_ev(Ev::Input, Msg::ChangeWWC) ]
    ]
}

fn view_main(count: usize, what_we_count: &str) -> El<Msg> {
    div![
        class![ if count > 1000 { "success" } else { "failure" }, "main"],
        // We can use normal Rust code and comments in the view.
        h3![ format!("{} {} so far", count, what_we_count) ],
        view_success_level(count),  // Incorporating a separate component

        button![ simple_ev(Ev::Click, Msg::Increment), "+" ],
        button![ simple_ev(Ev::Click, Msg::Decrement), "-" ],

        // Optionally-displaying an element
        if count >= 10 {
            h2![ style!{"padding" => px(50)}, "Nice!" ]
        } else {
            empty![]
        },
        view_what_we_count(&what_we_count),
    ]

}

/// The top-level component we pass to the virtual dom.
fn view(model: &Model) -> El<Msg> {
    div![
        h1![ style!{ "text-align" => "center" }, "The Grand Total" ],
        view_main(model.count, &model.what_we_count),

        button![ simple_ev(Ev::Click, Msg::Fetch), "Fetch data!" ],
        view_httpbin_data(&model.httpbin_data),
        pre![ code![ el_to_string(view_main(model.count, &model.what_we_count)) ] ],
        div![ class!["thing-container"], view_things(model.count, &model.what_we_count), ],
    ]
}

fn el_to_string<T>(el: El<T>) -> String {
    // handle text nodes
    match (&el.tag, &el.text) {
        (Tag::Text, Some(string)) => return String::from(string),
        (Tag::Text, None) => return String::new(),
        (_, _) => (),
    }
    let mut output = String::new();
    let tag = String::from(el.tag.as_str());
    let opening = format!("<{}", &tag);
    output.push_str(&opening);
    if !el.attrs.vals.is_empty() {
        output.push(' ');
        output.push_str(&el.attrs.to_string());
    }
    output.push('>');

    // Do not return children or a closing tag for void elements
    // https://html.spec.whatwg.org/multipage/syntax.html#void-elements
    let void_tags: HashSet<_> = [
        String::from("area"),
        String::from("base"),
        String::from("br"),
        String::from("col"),
        String::from("embed"),
        String::from("hr"),
        String::from("img"),
        String::from("input"),
        String::from("link"),
        String::from("meta"),
        String::from("param"),
        String::from("source"),
        String::from("track"),
        String::from("wrb"),
    ].iter().cloned().collect();
    if void_tags.contains(&tag) {
        return output;
    }

    for child in el.children {
        output.push_str(&el_to_string(child));
    }
    output.push_str(&format!("</{}>", &tag));
    output
}

#[wasm_bindgen(start)]
pub fn main_js() {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    console::log_1(&JsValue::from_str("Starting app"));

    let app = seed::App::build(Model::default(), update, view)
        .finish()
        .run();
    app.update(Msg::Fetch);
}

#[wasm_bindgen]
pub fn debug_element(element: JsValue) -> JsValue {
    log!(element);
    element
}
