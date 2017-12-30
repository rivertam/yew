#[macro_use]
extern crate yew;

mod my_component;

use yew::html::*;
use my_component::MyComponent;

struct Model {
    name: String,
}

enum Msg {
    ChangeName(String),
}

fn update(_: &mut Context<Msg>, model: &mut Model, msg: Msg) {
    match msg {
        Msg::ChangeName(new_name) => {
            model.name = new_name;
        }
    }
}

fn view(model: &Model) -> Html<Msg> {
    html! {
        <div>
            <input oninput=|e: InputData| Msg::ChangeName(e.value), />
            { model.name.clone() }
            <MyComponent initial_value={ 0 }, />
        </div>
    }
}

fn main() {
    let model = Model {
        name: "River".to_owned(),
    };

    program(model, update, view);
}
