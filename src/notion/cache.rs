use std::{collections::HashMap, sync::{Arc, OnceLock}};

use tokio::sync::RwLock;

use super::types::{Member, Group, NotionDataType, NotionData};


pub static CACHE_STORAGE: OnceLock<CacheStorage> = OnceLock::new();


pub struct CacheStorage {
  pub data: RwLock<HashMap<NotionDataType, HashMap<Arc<str>, NotionData>>>
}

impl CacheStorage {
  pub(self) fn new() -> CacheStorage {
    let mut init_hashmap: HashMap<NotionDataType, HashMap<Arc<str>, _>> = HashMap::new();
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
    data_type: NotionDataType,
  ) -> NotionData {
    match self.data.read().await.get(&data_type).unwrap().get(id) {
      Some(data) => data.clone(),
      None => NotionData::Null
    }
  }

  pub async fn request_all(
    self: &Self,
    data_type: NotionDataType
  ) -> Vec<NotionData> {
    let mut result: Vec<NotionData> = Vec::new();
    for data in self.data.read().await.get(&data_type).unwrap().values().into_iter() {
      result.push(data.clone());
    }
    result
  }

  pub async fn update(
    self: &Self,
    data_type: NotionDataType,
    new_data: Vec<NotionData>
  ) {
    let mut storage = self.data.write().await;
    let cache: &mut HashMap<Arc<str>, NotionData> = storage.get_mut(&data_type).unwrap();
    cache.clear();

    new_data
      .into_iter()
      .for_each(
        |raw_data| {
          match raw_data.clone() {
            NotionData::Member(mut data) => {
              let mut cleaned_groups: Vec<Group> = Vec::new();
              data.groups.clone().unwrap_or(Vec::default()).into_iter().for_each(
                |mut g: Group| {
                  g.members = None;
                  cleaned_groups.push(g);
                }
              );
              data.groups = Some(cleaned_groups);
              cache.insert(data.id.clone(), NotionData::Member(data));
            },
            NotionData::Group(mut data) => {
              let mut cleaned_members: Vec<Member> = Vec::new();
              data.members.clone().unwrap_or(Vec::default()).into_iter().for_each(
                |mut m: Member| {
                  m.groups = None;
                  cleaned_members.push(m);
                }
              );
              data.members = Some(cleaned_members);
              cache.insert(data.id.clone(), NotionData::Group(data));
            },
            NotionData::Club(data) => { cache.insert(data.id.clone(), raw_data); },
            NotionData::Event(data) => { cache.insert(data.id.clone(), raw_data); },
            NotionData::Article(data) => { cache.insert(data.id.clone(), raw_data); },
            NotionData::Sponsor(data) => { cache.insert(data.id.clone(), raw_data); },
            NotionData::Null => (),
          }
        }
      );
  }
}
