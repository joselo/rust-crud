use stdweb::web::{FormData, FormDataEntry};
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Default, Clone, PartialEq)]
pub struct Item {
  pub id: usize,
  pub name: String
}

#[derive(Debug, PartialEq)]
pub struct ValidatedItem {
  name: String
}

#[derive(Debug, PartialEq)]
pub enum ItemValidationErr {
  InvalidName
}

impl Item {
  pub fn generate_id() -> usize {
    static COUNTER:AtomicUsize = AtomicUsize::new(1);
    COUNTER.fetch_add(1, Ordering::Relaxed)
  }

  pub fn validate(item: &Item) -> Result<ValidatedItem, Vec<ItemValidationErr>> {
    let mut errors = vec![];

    let name = Item::validate_name(String::from(&item.name))
      .unwrap_or_else(|e| {
        errors.push(e);
        String::from("")
      });

    if !errors.is_empty() {
      return Err(errors);
    }

    Ok( ValidatedItem { name } )
  }

  fn validate_name(name: String) -> Result<String, ItemValidationErr> {
    if name.len() > 1 {
      Ok(name)
    } else {
      Err(ItemValidationErr::InvalidName)
    }
  }
}

impl From<FormData> for Item {
  fn from(fd: FormData) -> Self {
    let name = match fd.get("name").unwrap() {
      FormDataEntry::String(name) => name,
      _ => unreachable!(),
    };

    Self {
      name,
      ..Default::default()
    }
  }
}
