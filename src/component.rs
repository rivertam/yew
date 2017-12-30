use html::{Context, Html};

pub trait Component<Model, Props, Msg> {
    fn get_initial_model(&self, props: Props) -> Model;
    fn update(&self, context: &mut Context<Msg>, model: &mut Model, msg: Option<Msg>);
    fn view(&self, props: &Props, model: &Model) -> Html<Msg>;
}
