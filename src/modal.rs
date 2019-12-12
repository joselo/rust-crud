use yew::prelude::*;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use stdweb::unstable::TryInto;
use stdweb::web::{event::IEvent, Element, FormData};

use crate::item::Item;
use crate::item::ItemValidationErr;

use yew::services::{
  ConsoleService
};

#[derive(Properties)]
pub struct ModalProperties {
  #[props(required)]
  pub item: Item,
  #[props(required)]
  pub visible: bool,
  #[props(required)]
  pub on_close: Callback<bool>,
  #[props(required)]
  pub on_save: Callback<Item>
}

pub struct Modal {
  pub item: Item,
  pub visible: bool,
  pub on_close: Callback<bool>,
  pub on_save: Callback<Item>,
  error: Option<Vec<ItemValidationErr>>
}

pub enum ModalMsg {
  HideModal,
  Save(FormData)
}

impl Component for Modal {
  type Message = ModalMsg;
  type Properties = ModalProperties;

  fn create(prop: Self::Properties, _: ComponentLink<Self>) -> Self {
    Self {
      item: prop.item,
      visible: prop.visible,
      on_close: prop.on_close,
      on_save: prop.on_save,
      error: None
    }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    let mut console = ConsoleService::new();

    match msg {
      ModalMsg::HideModal => {
        self.visible = false;
        self.on_close.emit(true);

        true
      }

      ModalMsg::Save(form_data) => {
        let mut item: Item = form_data.into();
        item.id = self.item.id;

        let valid = Item::validate(&item);

        match valid {
          Ok(_v) => {
            self.visible = false;
            self.on_save.emit(item);
            console.log("Saved");
          },
          Err(e) => {
            self.error = Some(e)
          }
        }

        true
      }
    }
  }

  fn change(&mut self, props: Self::Properties) -> ShouldRender {
    self.item = props.item;
    self.visible = props.visible;

    true
  }

  fn view(&self) -> Html<Self> {
    let visible = if self.visible { "is-active" } else { "" };

    let error = |e: &ItemValidationErr| {
      match e {
        ItemValidationErr::InvalidName => html! { {"Enter some text"} }
      }
    };

    let errors = match self.error.as_ref() {
      None => {
        html! {}
      }

      Some(errors) => {
        html! {
          <div class="notification is-danger">
            {for errors.iter().map(error)}
          </div>
        }
      }
    };

    html! {
      <div class=("modal", visible)>
        <div class="modal-background"></div>
        <div class="modal-card">
          <form onsubmit=|e| {
            e.prevent_default();
            let form_element: Element = e.target().unwrap().try_into().unwrap();
            ModalMsg::Save(FormData::from_element(&form_element).unwrap())
          }>
            <header class="modal-card-head">
              <p class="modal-card-title">{"New Item"}</p>
              <a onclick=|_| ModalMsg::HideModal class="delete" aria-label="close"></a>
            </header>
            <section class="modal-card-body">
              {errors}
              <input value=&self.item.name name="name" class="input" autocomplete="off" />
            </section>
            <footer class="modal-card-foot">
              <button type="submit" class="button is-success">{"Save"}</button>
              <a onclick=|_| ModalMsg::HideModal class="button">{"Cancel"}</a>
            </footer>
          </form>
        </div>
      </div>
    }
  }
}
