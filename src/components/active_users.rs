// src/components/active_users.rs

use yew::prelude::*;

pub struct ActiveUsers {
    props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub count: usize,
}

impl Component for ActiveUsers {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                { format!("Active users: {}", self.props.count) }
            </div>
        }
    }
}