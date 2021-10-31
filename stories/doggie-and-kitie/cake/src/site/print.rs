use ::qrcode::{render::svg, EcLevel, QrCode, Version};
use std::{cell::RefCell, rc::Rc};
use uuid::Uuid;
use yew::{html, prelude::*, utils, web_sys::Element, Html};

use super::{characters, qrcode};

#[derive(Debug, Clone, PartialEq)]
pub struct PrintItem {
    title: Option<String>,
    description: Option<String>, // should be msgid
    img_url: Option<String>,
    data: Rc<Vec<u8>>,
}

impl PrintItem {
    pub fn new(data: Rc<Vec<u8>>) -> Self {
        Self {
            title: None,
            description: None,
            img_url: None,
            data,
        }
    }

    pub fn title(mut self, title: Option<String>) -> Self {
        self.title = title;
        self
    }

    pub fn description(mut self, description: Option<String>) -> Self {
        self.description = description;
        self
    }

    pub fn img_url(mut self, img_url: Option<String>) -> Self {
        self.img_url = img_url;
        self
    }
}

pub struct Print {}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub items: Vec<PrintItem>,
    pub close_cb: Callback<()>,
}

pub enum Msg {
    Close,
    Print,
}

impl Component for Print {
    type Message = Msg;
    type Properties = Props;

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Close => {
                log::debug!("Closing print");
                ctx.props().close_cb.emit(());
            }
            Msg::Print => {}
        }
        false
    }

    fn create(ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let close_cb = ctx.link().callback(|_| Msg::Close);

        let render_printable = |e: &PrintItem| {
            // QR code
            let qr_data = format!(
                "data:application/json;base64,{}",
                base64::encode(e.data.as_ref())
            );
            let qrcode = QrCode::with_error_correction_level(qr_data, EcLevel::H).unwrap();
            let svg = qrcode
                .render()
                .min_dimensions(200, 200)
                .dark_color(svg::Color("#000000"))
                .light_color(svg::Color("#ffffff"))
                .build();
            let qr_div = utils::document().create_element("div").unwrap();
            qr_div.set_inner_html(&svg);
            let qr_part = Html::VRef(qr_div.into());

            // Image
            let render_image = |url: Option<String>| {
                if let Some(url) = url.as_ref() {
                    html! {
                        <figure class="image is-48x48 is-align-items-center is-flex">
                            <img src={url.clone()}/>
                        </figure>
                    }
                } else {
                    html! {}
                }
            };
            html! {
                <>
                    <td>
                        <div class="card simple-border is-radiusless is-shadowless m-1 card-print">
                            <div class="card-content">
                                <div class="media is-align-items-center">
                                    <div class="media-left">
                                    { render_image(e.img_url.clone()) }
                                    </div>
                                    <div class="media-content">
                                        <p class="title">{ e.title.clone().unwrap_or_default() }</p>
                                    </div>
                                </div>
                            </div>
                            <div class="card-image has-text-centered">
                                <figure class="image">
                                    {qr_part}
                                </figure>
                                <div class="content">{e.description.clone().unwrap_or_default()}</div>
                            </div>
                        </div>
                    </td>
                </>
            }
        };

        let render_printable_line = |line: &[PrintItem]| {
            html! {
                <tr class="card-print">
                    { for line.into_iter().map(|e| render_printable(e)) }
                </tr>
            }
        };

        html! {
            <>
                <nav class="navbar is-fixed-top is-justify-content-right no-print">
                    <div class="navbar-brand">
                        <div
                          class="navbar-burger is-flex is-justify-content-center is-align-items-center"
                          onclick={close_cb.clone()}
                        >
                            <a class="delete"></a>
                        </div>
                    </div>
                </nav>
                <table class="print-table">
                    { for ctx.props().items.clone().chunks(4).map(|e| render_printable_line(e)) }
                </table>
            </>
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        // Update when component is reused
        true
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        let mut body = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .body()
            .unwrap();

        body.set_class_name("has-navbar-fixed-top");
    }
}
