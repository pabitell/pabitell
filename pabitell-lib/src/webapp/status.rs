use yew::{html, prelude::*};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub connect_ws: Callback<()>,
    pub refresh_world: Callback<()>,
    pub reset_world: Callback<()>,
    pub event_count: usize,
    pub status: WsStatus,
    pub ws_request_failed: bool,
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
            Self::CONNECTED => "fas fa-book",
            Self::CONNECTING => "fas fa-book-alt",
            Self::DISCONNECTED => "fas fa-book-dead",
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

        let status_part = if !ctx.props().ws_request_failed {
            html! {
                <button class="button is-outlined is-medium" {onclick}>
                    <span class={ classes!(ctx.props().status.text_classes()) }>
                        <i class={ classes!(ctx.props().status.icon_classes()) }></i>
                    </span>
                </button>
            }
        } else {
            html! {
                <button class="button is-outlined is-medium" {onclick}>
                    <span class="icon has-text-danger">
                        <i class="fas fa-book-dead"></i>
                    </span>
                </button>
            }
        };

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
                { status_part }
                <button class="button is-outlined is-medium" onclick={reset_world_cb}>
                    <span class="icon has-text-danger">
                        <i class="fas fa-sign-out-alt"></i>
                    </span>
                </button>
            </>
        }
    }
}
