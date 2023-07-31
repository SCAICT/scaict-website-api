use std::{
  collections::HashMap,
  sync::OnceLock
};

use tokio::sync::{
  RwLock,
  RwLockReadGuard,
  RwLockWriteGuard
};

use super::types::{NotionDataType, NotionData};


pub static CACHE_STORAGE: OnceLock<CacheStorage> = OnceLock::new();


pub struct CacheStorage {
  data: RwLock<HashMap<NotionDataType, HashMap<String, NotionData>>>
}

impl CacheStorage {
  fn new() -> CacheStorage {
    let mut init_hashmap: HashMap<_, _> = HashMap::new();

    NotionDataType::iterator().for_each(
      |data_type| {
        init_hashmap.insert(data_type, HashMap::new());
      }
    );

    CacheStorage {
      data: RwLock::new(init_hashmap)
    }
  }

  pub fn get() -> &'static CacheStorage {
    CACHE_STORAGE.get_or_init(
      || CacheStorage::new()
    )
  }

  pub async fn request(
    self: &Self,
    id: &str,
    data_type: &NotionDataType,
  ) -> Option<NotionData> {
    let storage: RwLockReadGuard<_> = self.data.read().await;

    match storage.get(&data_type).unwrap().get(id) {
      Some(data) => Some(data.clone()),
      None => None
    }
  }

  pub async fn request_all(
    self: &Self,
    data_type: &NotionDataType
  ) -> Vec<NotionData> {
    let storage: RwLockReadGuard<_> = self.data.read().await;

    let mut result: Vec<NotionData> = Vec::new();

    for data in storage.get(&data_type).unwrap().values().into_iter() {
      result.push(data.clone());
    }

    result
  }

  pub async fn update(
    self: &Self,
    data_type: &NotionDataType,
    new_data: Vec<NotionData>
  ) {
    let mut storage: RwLockWriteGuard<_> = self.data.write().await;

    let cache: &mut HashMap<String, NotionData> = storage.get_mut(&data_type).unwrap();
    cache.clear();

    new_data
      .into_iter()
      .for_each(
        |raw_data: NotionData| {
          match &raw_data {
            NotionData::Member(data) => { cache.insert(data.id.clone(), raw_data); },
            NotionData::Group(data) => { cache.insert(data.id.clone(), raw_data); },
            NotionData::Club(data) => { cache.insert(data.id.clone(), raw_data); },
            NotionData::Event(data) => { cache.insert(data.id.clone(), raw_data); },
            NotionData::Article(data) => { cache.insert(data.id.clone(), raw_data); },
            NotionData::Sponsor(data) => { cache.insert(data.id.clone(), raw_data); }
          }
        }
      );
  }
}
