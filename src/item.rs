use stdweb::web::{FormData, FormDataEntry};
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Default, Clone, PartialEq)]
pub struct Item {
  pub id: usize,
  pub name: String,
  pub price: f32
}

#[derive(Default, PartialEq)]
pub struct ItemFormData {
  pub name: String,
  pub price: String
}

#[derive(Debug, PartialEq)]
pub struct ValidatedItem {
  name: String,
  price: String
}

#[derive(Debug, PartialEq)]
pub enum ItemValidationErr {
  InvalidName,
  InvalidPrice
}

impl ItemFormData {
  pub fn validate(form_data: &ItemFormData) -> Result<ValidatedItem, Vec<ItemValidationErr>> {
    let mut errors = vec![];

    let name = ItemFormData::validate_name(String::from(&form_data.name))
      .unwrap_or_else(|e| {
        errors.push(e);
        String::from("")
      });

    let price = ItemFormData::validate_price(String::from(&form_data.price))
      .unwrap_or_else(|e| {
        errors.push(e);
        String::from("")
      });

    if !errors.is_empty() {
      return Err(errors);
    }

    Ok( ValidatedItem { name, price } )
  }

  fn validate_name(name: String) -> Result<String, ItemValidationErr> {
    if name.len() > 1 {
      Ok(name)
    } else {
      Err(ItemValidationErr::InvalidName)
    }
  }

  fn validate_price(price: String) -> Result<String, ItemValidationErr> {
    if price.parse::<f64>().is_ok() {
      Ok(price)
    } else {
      Err(ItemValidationErr::InvalidPrice)
    }
  }
}

impl From<FormData> for ItemFormData {
  fn from(fd: FormData) -> Self {
    let name = match fd.get("name").unwrap() {
      FormDataEntry::String(name) => name,
      _ => unreachable!()
    };

    let price = match fd.get("price").unwrap() {
      FormDataEntry::String(price) => price,
      _ => unreachable!()
    };

    Self {
      name,
      price,
      ..Default::default()
    }
  }
}

impl Item {
  pub fn generate_id() -> usize {
    static COUNTER:AtomicUsize = AtomicUsize::new(1);
    COUNTER.fetch_add(1, Ordering::Relaxed)
  }
}
