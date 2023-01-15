use core::{pin::Pin, future::Future, marker::Send};
use std::{collections::HashMap, sync::{Arc}, ops::DerefMut};
use salvo::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::sync::Mutex;

#[derive(Debug, Serialize, Deserialize)]
pub struct CardList {
  identifiers : Vec<CardID>
}

impl CardList {
  pub fn length(&self) -> usize {
    self.identifiers.len()
  }
}

impl From<Vec<CardID>> for CardList {
    fn from(value: Vec<CardID>) -> Self {
        CardList { identifiers: value }
    }
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Hash, Debug, Clone)]
pub struct CardID {
  name : String,
  #[serde(default)]
  #[serde(skip_serializing_if = "Option::is_none")]
  set : Option<String>
}

impl From<String> for CardID {
    fn from(value: String) -> Self {
        CardID { name: value, set: None }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CardInfo {
  name : String,
  front : String,
  #[serde(default)]
  #[serde(skip_serializing_if = "Option::is_none")]
  back : Option<String>
}

#[derive()]
pub struct MTGCardManager {
  card_info : HashMap<CardID, CardInfo>
}

impl MTGCardManager {
  pub fn new() -> MTGCardManager {
    MTGCardManager {
      card_info: HashMap::new()
    }
  }

  pub async fn query_group(&mut self, cards : Vec<CardID>) -> HashMap<String, Option<CardInfo>> {
    let mut return_hash = HashMap::new();
    let mut cards_to_fetch = Vec::new();

    for card in cards.into_iter() {
      if self.card_info.contains_key(&card) {
        return_hash.insert(card.name.clone(), Some(self.card_info.get(&card).unwrap().clone()));
      } else {
        return_hash.insert(card.name.clone(), None);
        cards_to_fetch.push(card.clone());
      }
    }

    if !cards_to_fetch.is_empty() {
      println!("Starting Fetch");
      self.fetch_cards(cards_to_fetch, &mut return_hash).await;
    }

    for (key, value) in return_hash.iter() {
      if value.is_some() {
        let card_info = value.clone().unwrap();
        self.card_info.insert(key.clone().into(), card_info);
      }
    }

    return_hash
  }

  pub async fn fetch_cards(&self, cards : impl Into<CardList>, card_hash : &mut HashMap<String, Option<CardInfo>>) {
    let cards : CardList = cards.into();

    println!("\nScryfall Fetch Starting!");
    println!("  Attempting to pull {} Cards.", cards.length());

    let client = reqwest::Client::new();
    if let Ok(res) = client.post("https://api.scryfall.com/cards/collection").json(&cards).send().await {
      let obj : Value = res.json().await.unwrap();
      if let Some(data) = obj["data"].as_array() {
        println!("  Got {} cards from Scryfall.", data.len());
        for card in data.iter() {
          let name = card["name"].as_str().unwrap().to_string();
          println!("  Pulled card with name '{}'!", name);
          let layout = card["layout"].as_str().unwrap();
          if card["card_faces"].is_null() {
            card_hash.insert(name.clone(), Some(CardInfo{
              name : name.clone(),
              front :  card["image_uris"]["normal"].as_str().unwrap().to_string(),
              back : None
            }));
          } else if layout == "split" || layout == "flip" {
            let name = card["card_faces"][0]["name"].as_str().unwrap().to_string();
            card_hash.insert(name.clone(), Some(CardInfo{
              name : name.clone(),
              front :  card["image_uris"]["normal"].as_str().unwrap().to_string(),
              back : None
            }));
          } else {
            let name = card["card_faces"][0]["name"].as_str().unwrap().to_string();
            card_hash.insert(name.clone(), Some(CardInfo { 
              name: name, 
              front: card["card_faces"][0]["image_uris"]["normal"].as_str().unwrap().to_string(), 
              back: Some(card["card_faces"][1]["image_uris"]["normal"].as_str().unwrap().to_string())
            }));
          }
        }
      }
    };
  }
}

pub struct MTGCardHandler {
  manager : Arc<Mutex<MTGCardManager>>
}

#[handler]
impl MTGCardHandler {
  pub fn new(manager : Arc<Mutex<MTGCardManager>>) -> MTGCardHandler {
    MTGCardHandler {
      manager
    }
  }

  async fn handle(&self, req: &mut Request, res: &mut Response) {
    if let Ok(cards) = req.parse_json::<Vec<CardID>>().await {
      let mut card_memory = self.manager.lock().await;
      let c = card_memory.deref_mut().query_group(cards).await;
      res.render(Json(c));
      res.set_status_code(StatusCode::OK);
    } else {
      res.set_status_code(StatusCode::BAD_REQUEST)
    }
  }
}