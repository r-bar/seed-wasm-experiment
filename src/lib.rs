#[macro_use]
extern crate seed;
use wasm_bindgen::prelude::*;
use web_sys::{console, Node};
#[macro_use]
use seed::prelude::*;


// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


// This is like the `main` function, except for JavaScript.
//#[wasm_bindgen(start)]
//pub fn main_js() -> Result<(), JsValue> {
//    // This provides better error messages in debug mode.
//    // It's disabled in release mode so it doesn't bloat up the file size.
//    #[cfg(debug_assertions)]
//    console_error_panic_hook::set_once();


//    // Your code goes here!
//    console::log_1(&JsValue::from_str("Hello world!"));

//    Ok(())
//}

// Model

struct Model {
    count: i32,
    what_we_count: String
}

// Setup a default here, for initialization later.
impl Default for Model {
    fn default() -> Self {
        Self {
            count: 0,
            what_we_count: "click".into()
        }
    }
}


// Update

#[derive(Clone)]
enum Msg {
    Increment,
    Decrement,
    ChangeWWC(String),
}

/// How we update the model
fn update(msg: Msg, model: &mut Model, _orders: &mut Orders<Msg>) {
    match msg {
        Msg::Increment => model.count += 1,
        Msg::Decrement => model.count -= 1,
        Msg::ChangeWWC(what_we_count) => model.what_we_count = what_we_count,
    }
}


// View

/// A simple component.
fn success_level(clicks: i32) -> El<Msg> {
    let descrip = match clicks {
        0 ... 5 => "Not very many 🙁",
        6 ... 9 => "I got my first real six-string 😐",
        10 ... 11 => "Spinal Tap 🙂",
        _ => "Double pendulum 🙃"
    };
    p![ descrip ]
}

/// The top-level component we pass to the virtual dom.
fn view(model: &Model) -> El<Msg> {
    let plural = if model.count == 1 {""} else {"s"};

    // Attrs, Style, Events, and children may be defined separately.
    let outer_style = style!{
            "display" => "flex";
            "flex-direction" => "column";
            "text-align" => "center"
    };

    div![ outer_style,
        h1![ "The Grand Total" ],
        div![
            style!{
                // Example of conditional logic in a style.
                "color" => if model.count > 4 {"purple"} else {"gray"};
                "border" => "2px solid #004422";
                "padding" => unit!(20, px);
            },
            // We can use normal Rust code and comments in the view.
            h3![ format!("{} {}{} so far", model.count, model.what_we_count, plural) ],
            button![ simple_ev(Ev::Click, Msg::Increment), "+" ],
            button![ simple_ev(Ev::Click, Msg::Decrement), "-" ],

            // Optionally-displaying an element
            if model.count >= 10 { h2![ style!{"padding" => px(50)}, "Nice!" ] } else { empty![] }
        ],
        success_level(model.count),  // Incorporating a separate component

        h3![ "What are we counting?" ],
        input![ attrs!{At::Value => model.what_we_count}, input_ev(Ev::Input, Msg::ChangeWWC) ]
    ]
}


#[wasm_bindgen(start)]
pub fn main_js() {
    seed::App::build(Model::default(), update, view)
        .finish()
        .run();
}
