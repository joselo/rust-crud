#![recursion_limit = "512"]

use yew::{html, Component, ComponentLink, Html, ShouldRender};

mod item;
mod modal;

use crate::item::Item;
use crate::modal::Modal;

// use yew::services::{
//   ConsoleService
// };

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
  Remove(usize)
}

impl Component for List {
  type Message = Msg;
  type Properties = ();

  fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
    let mut list = List {
      items: Vec::new(),
      modal_visible: false,
      current_item: None
    };

    list.items.push(Item { id: Item::generate_id(), name: "Test".to_string(), ..Default::default() });
    list
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    //let mut console = ConsoleService::new();

    match msg {
      Msg::New => {
        self.modal_visible = true;
        self.current_item = None;

        true
      }

      Msg::HiddedModal => {
        self.modal_visible = false;
        true
      }

      Msg::Saved(item) => {
        if item.id == 0 {
          let mut item = item;
          item.id = Item::generate_id();
          self.items.push(item);
        } else {
          let index = self.items.iter().position(|i| i.id == item.id).unwrap();
          self.items[index] = item;
        }

        self.update(Msg::HiddedModal);

        true
      }

      Msg::Edit(idx) => {
        let item = self.items[idx].clone();
        self.current_item = Some(item);
        self.modal_visible = true;
        //console.log("aa");

        true
      }

      Msg::Remove(idx) => {
        self.items.remove(idx);
        true
      }
    }
  }

  fn view(&self) -> Html<Self> {
    let modal = match self.current_item.as_ref() {
      None => {
        html! {
          <Modal: item=Item { ..Default::default() } visible=self.modal_visible on_close=|_| { Msg::HiddedModal } on_save=Msg::Saved />
        }
      }

      Some(item) => {
        html! {
          <Modal: item=item visible=self.modal_visible on_close=|_| { Msg::HiddedModal } on_save=Msg::Saved />
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
                <th colspan="2"></th>
              </tr>
            </thead>
            <tbody>
              {for self.items.iter().enumerate().map(view_item)}
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

fn view_item((idx, item): (usize, &Item)) -> Html<List> {
  html! {
    <tr>
      <td>{&item.id}</td>
      <td>{&item.name}</td>
      <td><button onclick=|_| { Msg::Edit(idx) } type="button" class="button is-black">{"Edit"}</button></td>
      <td><button onclick=|_| { Msg::Remove(idx) } type="button" class="button is-danger">{"Remove"}</button></td>
    </tr>
  }
}
