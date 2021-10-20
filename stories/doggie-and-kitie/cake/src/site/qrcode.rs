use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
};

use qrcode::{render::svg, EcLevel, QrCode, Version};
use yew::{function_component, html, prelude::*, web_sys::Element};

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub qr_code_scope: Rc<RefCell<Option<html::Scope<QRCode>>>>,
}

impl PartialEq<Self> for Props {
    fn eq(&self, rhs: &Self) -> bool {
        true
    }
}

pub struct QRCode {
    node_ref: NodeRef,
    data: Option<Rc<Vec<u8>>>,
}

pub enum Msg {
    Show(Option<Rc<Vec<u8>>>),
}

impl Component for QRCode {
    type Message = Msg;
    type Properties = Props;

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Show(data) => {
                self.data = data;
            }
        }
        true
    }

    fn create(ctx: &Context<Self>) -> Self {
        *ctx.props().qr_code_scope.borrow_mut() = Some(ctx.link().clone());
        Self {
            node_ref: NodeRef::default(),
            data: None,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let close_cb = ctx.link().callback(|_| Msg::Show(None));
        let classes = if self.data.is_some() {
            classes!("modal", "is-active")
        } else {
            classes!("modal")
        };
        html! {
            <div class={classes}>
                <div class="modal-background"></div>
                <div class="modal-content has-text-centered" ref={self.node_ref.clone()}>
                </div>
                <button
                  onclick={close_cb}
                  class="modal-close is-large"
                  aria-label="close"
                 ></button>
            </div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        if let Some(data) = self.data.as_ref() {
            let qr_data = format!(
                "data:application/json;base64,{}",
                base64::encode(data.as_ref())
            );
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
