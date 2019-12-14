#![recursion_limit = "512"]

use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew::format::Json;
use yew::services::storage::{Area, StorageService};

mod item;
mod modal;

use crate::item::Item;
use crate::modal::Modal;

const KEY: &'static str = "yew.rust.crud.database";

pub struct Model {
  storage: StorageService,
  state: List
}

pub struct List {
  items: Vec<Item>,
  modal_visible: bool,
  current_item: Option<Item>
}

pub enum Msg {
  New,
  HiddedModal,
  Saved(Item),
  Edit(usize),
  Remove(usize),
  Store
}

impl Component for Model {
  type Message = Msg;
  type Properties = ();

  fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
    let storage = StorageService::new(Area::Local);

    let items = {
      if let Json(Ok(items)) = storage.restore(KEY) {
        items
      } else {
        Vec::new()
      }
    };

    let state = List {
      items,
      modal_visible: false,
      current_item: None
    };

    Model { storage, state }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {

    match msg {
      Msg::New => {
        self.state.modal_visible = true;
        self.state.current_item = None;

        true
      }

      Msg::HiddedModal => {
        self.state.modal_visible = false;
        true
      }

      Msg::Saved(item) => {
        if item.id == 0 {
          let mut item = item;
          item.id = Item::generate_id();
          self.state.items.push(item);
        } else {
          let index = self.state.items.iter().position(|i| i.id == item.id).unwrap();
          self.state.items[index] = item;
        }

        self.update(Msg::HiddedModal);
        self.update(Msg::Store);

        true
      }

      Msg::Edit(idx) => {
        let item = self.state.items[idx].clone();
        self.state.current_item = Some(item);
        self.state.modal_visible = true;

        true
      }

      Msg::Remove(idx) => {
        self.state.items.remove(idx);
        self.update(Msg::Store);

        true
      }

      Msg::Store => {
        self.storage.store(KEY, Json(&self.state.items));
        false
      }
    }
  }

  fn view(&self) -> Html<Self> {
    let modal = match self.state.current_item.as_ref() {
      None => {
        html! {
          <Modal: item=Item { ..Default::default() } visible=self.state.modal_visible on_close=|_| { Msg::HiddedModal } on_save=Msg::Saved />
        }
      }

      Some(item) => {
        html! {
          <Modal: item=item visible=self.state.modal_visible on_close=|_| { Msg::HiddedModal } on_save=Msg::Saved />
        }
      }
    };

    html! {
      <>
        {modal}
        <div class="container">
          <table class="table is-hoverable is-fullwidth">
            <thead>
              <tr>
                <th>{"Id"}</th>
                <th>{"Name"}</th>
                <th>{"Price"}</th>
                <th colspan="2"></th>
              </tr>
            </thead>
            <tbody>
              {for self.state.items.iter().enumerate().map(view_item)}
            </tbody>
          </table>

          <div>
            <button onclick=|_| { Msg::New } type="button" class="button is-primary">{"Add"}</button>
          </div>
        </div>
      </>
    }
  }
}

fn view_item((idx, item): (usize, &Item)) -> Html<Model> {
  html! {
    <tr>
      <td>{&item.id}</td>
      <td>{&item.name}</td>
      <td>{&item.price}</td>
      <td><button onclick=|_| { Msg::Edit(idx) } type="button" class="button is-black">{"Edit"}</button></td>
      <td><button onclick=|_| { Msg::Remove(idx) } type="button" class="button is-danger">{"Remove"}</button></td>
    </tr>
  }
}
