use yew::{html, prelude::*};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub connect_ws: Callback<()>,
    pub refresh_world: Callback<()>,
    pub reset_world: Callback<()>,
    pub event_count: usize,
    pub status: WsStatus,
}

#[derive(Clone, Debug, PartialEq)]
pub enum WsStatus {
    CONNECTING,
    CONNECTED,
    DISCONNECTED,
}

impl Default for WsStatus {
    fn default() -> Self {
        Self::DISCONNECTED
    }
}

impl WsStatus {
    fn icon_classes(&self) -> String {
        match self {
            Self::CONNECTED => "fas fa-check-circle",
            Self::CONNECTING => "rotate fas fa-circle-notch",
            Self::DISCONNECTED => "fas fa-times-circle",
        }
        .to_string()
    }
    fn text_classes(&self) -> String {
        match self {
            Self::CONNECTED => "icon has-text-success",
            Self::CONNECTING => "icon has-text-info",
            Self::DISCONNECTED => "icon has-text-danger",
        }
        .to_string()
    }
}

pub struct Status {}

pub enum Msg {
    Connect,
    RefreshWorld,
    ResetWorld,
}

impl Component for Status {
    type Message = Msg;
    type Properties = Props;

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link().clone();
        match msg {
            Msg::Connect => {
                ctx.props().connect_ws.emit(());
            }
            Msg::RefreshWorld => {
                ctx.props().refresh_world.emit(());
            }
            Msg::ResetWorld => {
                ctx.props().reset_world.emit(());
            }
        }
        true
    }

    fn create(ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let onclick = link.callback(|_| Msg::Connect);

        let refresh_world_cb = link.callback(|_| Msg::RefreshWorld);
        let reset_world_cb = link.callback(|_| Msg::ResetWorld);

        html! {
            <>
                <button class="button is-outlined is-medium is-static">
                    {ctx.props().event_count}
                </button>
                <button class="button is-outlined is-medium" onclick={ refresh_world_cb }>
                    <span class="icon has-text-info">
                        <i class="fas fa-sync"></i>
                    </span>
                </button>
                <button class="button is-outlined is-medium" {onclick}>
                    <span class={ classes!(ctx.props().status.text_classes()) }>
                        <i class={ classes!(ctx.props().status.icon_classes()) }></i>
                    </span>
                </button>
                <button class="button is-outlined is-medium" onclick={reset_world_cb}>
                    <span class="icon has-text-danger">
                        <i class="fas fa-sign-out-alt"></i>
                    </span>
                </button>
            </>
        }
    }
}
