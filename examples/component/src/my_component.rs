use yew::html::*;
use yew::component::*;
use yew::services::console::ConsoleService;

pub struct MyComponent;

pub struct MyModel {
    pub counter: i32,
}

// I'm thinking this could be included and used by the macro, but not by the user
// so it could have to always be called "Props" so the macro knows what to use
#[derive(PartialEq, Eq)]
pub struct Props {
    // bad example, probably, because it would imply this is controlled while it's not
    pub initial_value: i32,
}

enum MyMsg {
    Increment,
    Decrement,
    Noop,
}

impl Component<MyModel, Props, MyMsg> for MyComponent {
    fn get_initial_model(&self, props: Props) -> MyModel {
        MyModel { counter: props.initial_value }
    }

    // other lifecycle hooks could definitely exist

    // I guess view would be called after every update, regardless of if it
    // updated the model or the view
    fn update(&self, context: &mut Context<MyMsg>, model: &mut MyModel, msg: Option<MyMsg>) {
        match msg.unwrap_or(MyMsg::Noop) {
            MyMsg::Increment => {
                model.counter = model.counter + 1;
            }
            MyMsg::Decrement => {
                 model.counter = model.counter - 1;
            }
            MyMsg::Noop => {
                context.get_console().info("Maybe a lifecycle hook got called or something");
            }
        }
    }

    fn view(&self, _: &Props, model: &MyModel) -> Html<MyMsg> {
        html! {
            <div>
               <p>{ "Counter:" } { model.counter }</p>
               <button onclick=|_| MyMsg::Increment,> { "Up" } </button>
               <button onclick=|_| MyMsg::Decrement,>{ "Down" }</button>
            </div>
        }
    }
}
