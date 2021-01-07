use yew::{html, Component, ComponentLink, Html, ShouldRender, Callback, Properties};

use crate::item::Item;
use crate::item::ItemFormData;
use crate::item::ItemValidationErr;
use crate::input::TextInput;

use yew::services::{
  ConsoleService
};

#[derive(Properties, Clone)]
pub struct ModalProperties {
  pub item: Item,
  pub visible: bool,
  pub on_close: Callback<bool>,
  pub on_save: Callback<Item>
}

pub struct Modal {
  pub item: Item,
  pub name: String,
  pub price: String,
  pub visible: bool,
  pub on_close: Callback<bool>,
  pub on_save: Callback<Item>,
  error: Option<Vec<ItemValidationErr>>,
  link: ComponentLink<Self>
}

pub enum ModalMsg {
  HideModal,
  SetName(String),
  SetPrice(String),
  Save
}

impl Component for Modal {
  type Message = ModalMsg;
  type Properties = ModalProperties;

  fn create(prop: Self::Properties, link: ComponentLink<Self>) -> Self {
    Self {
      item: prop.item,
      name: "".to_string(),
      price: "".to_string(),
      visible: prop.visible,
      on_close: prop.on_close,
      on_save: prop.on_save,
      error: None,
      link
    }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      ModalMsg::HideModal => {
        self.visible = false;
        self.on_close.emit(true);

        true
      }

      ModalMsg::SetName(name) => {
        self.name = name;

        true
      }

      ModalMsg::SetPrice(price) => {
        self.price = price;

        true
      }

      ModalMsg::Save => {
        let form_data: ItemFormData = (self.name.clone(), self.price.clone()).into();
        let valid = ItemFormData::validate(&form_data);

        match valid {
          Ok(_v) => {
            self.visible = false;
            self.on_save.emit(Item {
              id: self.item.id,
              name: form_data.name,
              price: form_data.price.parse().unwrap(),
              ..Default::default()
            });

            //self.error = None;
            ConsoleService::info("Saved");
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
    self.name = props.item.name.clone();
    self.price = props.item.price.to_string();
    self.item = props.item;
    self.visible = props.visible;
    self.error = None;

    true
  }

  fn view(&self) -> Html {
    let visible = if self.visible { "is-active" } else { "" };

    let error = |e: &ItemValidationErr| {
      match e {
        ItemValidationErr::InvalidName => html! {
          <div>
            {"Name is required"}
          </div>
        },
        ItemValidationErr::InvalidPrice => html! {
          <div>
            {"Invalid Price"}
          </div>
        }
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

    let title = if self.item.name.is_empty() {
      "New Item"
    } else {
      "Update Item"
    };

    html! {
      <div class=("modal", visible)>
        <div class="modal-background"></div>
        <div class="modal-card">
          <form onsubmit=self.link.callback(|e: yew::events::FocusEvent| {
            e.prevent_default();

            ModalMsg::Save
          })>
            <header class="modal-card-head">
              <p class="modal-card-title">{title}</p>
              <a onclick=self.link.callback(|_| ModalMsg::HideModal) class="delete" aria-label="close"></a>
            </header>
            <section class="modal-card-body">
              {errors}
              <div class="field">
                <label class="label">{"Name"}</label>
                <div class="control">
                <TextInput value=&self.name oninput=self.link.callback(|val: String| ModalMsg::SetName(val))/>
                </div>
              </div>

              <div class="field">
                <label class="label">{"Price"}</label>
                <p class="control has-icons-left has-icons-right">
                  <TextInput value=&self.price oninput=self.link.callback(|val: String| ModalMsg::SetPrice(val))/>
                  <span class="icon is-small is-left">
                    <i class="icon ion-md-cash"></i>
                  </span>
                </p>
              </div>
            </section>
            <footer class="modal-card-foot">
              <button type="submit" class="button is-info">{"Save"}</button>
              <a onclick=self.link.callback(|_| ModalMsg::HideModal) class="button">{"Cancel"}</a>
            </footer>
          </form>
        </div>
      </div>
    }
  }
}