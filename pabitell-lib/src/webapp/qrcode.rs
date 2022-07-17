use std::{cell::RefCell, rc::Rc};

use qrcode::{render::svg, EcLevel, QrCode};
use web_sys::Element;
use yew::{html, prelude::*};

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub qr_code_scope: Rc<RefCell<Option<html::Scope<QRCode>>>>,
}

impl PartialEq<Self> for Props {
    fn eq(&self, _rhs: &Self) -> bool {
        true
    }
}

pub struct QRCode {
    node_ref: NodeRef,
    data: Option<Rc<Vec<u8>>>,
    show_data: bool,
}

pub enum Msg {
    Show(Option<Rc<Vec<u8>>>),
    ToggleData,
}

impl Component for QRCode {
    type Message = Msg;
    type Properties = Props;

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Show(data) => {
                self.show_data = false;
                self.data = data;
            }
            Msg::ToggleData => {
                self.show_data = !self.show_data;
            }
        }
        true
    }

    fn create(ctx: &Context<Self>) -> Self {
        *ctx.props().qr_code_scope.borrow_mut() = Some(ctx.link().clone());
        Self {
            node_ref: NodeRef::default(),
            data: None,
            show_data: false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let close_cb = ctx.link().callback(|_| Msg::Show(None));
        let toggle_cb = ctx.link().callback(|_| Msg::ToggleData);

        let classes = if self.data.is_some() {
            classes!("modal", "is-active")
        } else {
            classes!("modal")
        };
        let toggle_classes = if self.show_data {
            classes!("dropdown", "is-active")
        } else {
            classes!("dropdown")
        };

        html! {
            <div class={classes}>
                <div class="modal-background"></div>
                  <div class={toggle_classes}>
                    <div class="dropdown-trigger">
                      <div
                        onclick={ toggle_cb }
                        aria-haspopup="true"
                        aria-controls="dropdown-menu"
                        ref={self.node_ref.clone()}
                      >
                      </div>
                    </div>
                    <div class="dropdown-menu" id="dropdown-menu" role="menu">
                      <div class="dropdown-content w-50">
                        <div class="dropdown-item is-size-7">
                          <p class="is-size-7" style="word-wrap:break-word;">{{ self.qr_data().unwrap_or_default() }}</p>
                        </div>
                      </div>
                    </div>
                  </div>
                <button
                  onclick={close_cb}
                  class="modal-close is-large"
                  aria-label="close"
                 ></button>
            </div>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
        if let Some(qr_data) = self.qr_data() {
            let qrcode = QrCode::with_error_correction_level(qr_data, EcLevel::H).unwrap();
            let img = qrcode
                .render()
                .min_dimensions(200, 200)
                .dark_color(svg::Color("#000000"))
                .light_color(svg::Color("#ffffff"))
                .build();
            let el = self.node_ref.cast::<Element>().unwrap();
            el.set_inner_html(&img);
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        // Update when component is reused
        *ctx.props().qr_code_scope.borrow_mut() = Some(ctx.link().clone());
        true
    }
}

impl QRCode {
    fn qr_data(&self) -> Option<String> {
        self.data.as_ref().map(|e| {
            format!(
                "data:application/json;base64,{}",
                base64::encode(e.as_ref())
            )
        })
    }
}
