use std::{sync::{Arc, OnceLock}, env};

use serde_json::Value;
use anyhow::{Result, anyhow};

use super::cache::CacheStorage;


pub static MEMBER_DATABASE_ID: OnceLock<Arc<str>> = OnceLock::new();
pub static GROUP_DATABASE_ID: OnceLock<Arc<str>> = OnceLock::new();
pub static CLUB_DATABASE_ID: OnceLock<Arc<str>> = OnceLock::new();
pub static EVENT_DATABASE_ID: OnceLock<Arc<str>> = OnceLock::new();
pub static ARTICLE_DATABASE_ID: OnceLock<Arc<str>> = OnceLock::new();
pub static SPONSOR_DATABASE_ID: OnceLock<Arc<str>> = OnceLock::new();


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NotionDataType {
  Member, Group, Club, Event, Article, Sponsor
}

impl NotionDataType {
  pub fn iterator() -> std::array::IntoIter<NotionDataType, 6> {
      static DIRECTIONS: [NotionDataType; 6] = [
        NotionDataType::Member,
        NotionDataType::Group,
        NotionDataType::Club,
        NotionDataType::Event,
        NotionDataType::Article,
        NotionDataType::Sponsor
      ];
      DIRECTIONS.clone().into_iter()
  }

  pub fn get_databse_id(self: &Self) -> &str {
    match self {
      NotionDataType::Member => MEMBER_DATABASE_ID.get_or_init(
        || {
          env::var("MEMBER_DATABASE_ID")
            .expect("MEMBER_DATABASE_ID is not set.")
            .into()
        }
      ),
      NotionDataType::Group => GROUP_DATABASE_ID.get_or_init(
        || {
          env::var("GROUP_DATABASE_ID")
            .expect("GROUP_DATABASE_ID is not set.")
            .into()
        }
      ),
      NotionDataType::Club => CLUB_DATABASE_ID.get_or_init(
        || {
          env::var("CLUB_DATABASE_ID")
            .expect("CLUB_DATABASE_ID is not set.")
            .into()
        }
      ),
      NotionDataType::Event => EVENT_DATABASE_ID.get_or_init(
        || {
          env::var("EVENT_DATABASE_ID")
            .expect("EVENT_DATABASE_ID is not set.")
            .into()
        }
      ),
      NotionDataType::Article => ARTICLE_DATABASE_ID.get_or_init(
        || {
          env::var("ARTICLE_DATABASE_ID")
            .expect("ARTICLE_DATABASE_ID is not set.")
            .into()
        }
      ),
      NotionDataType::Sponsor => SPONSOR_DATABASE_ID.get_or_init(
        || {
          env::var("SPONSOR_DATABASE_ID")
            .expect("SPONSOR_DATABASE_ID is not set.")
            .into()
        }
      )
    }
  }
}


#[derive(Debug, Clone)]
pub enum NotionData {
  Member(Member),
  Group(Group),
  Club(Club),
  Event(Event),
  Article(Article),
  Sponsor(Sponsor),
  Null
}

#[derive(Debug, Clone)]
pub struct EventPeriod {
  start: Arc<str>,
  end: Arc<str>
}

impl EventPeriod {
  pub fn default() -> EventPeriod {
    EventPeriod {
      start: "1970-01-01T00:00:00.000+00:00".into(),
      end: "1970-01-01T00:00:00.000+00:00".into(),
    }
  }

  pub fn from_json(json_data: &Value) -> Result<EventPeriod> {
    Ok(
      EventPeriod {
        start: json_data["start"]
          .as_str()
          .ok_or(
            anyhow!("Get `start` failed.")
          )?
          .into(),
        end: json_data["end"]
          .as_str()
          .ok_or(
            anyhow!("Get `end` failed.")
          )?
          .into(),
      }
    )
  }
}

#[derive(Debug, Clone)]
pub struct Member {
  pub id: Arc<str>,
  pub avatar: Arc<str>,
  pub name: Arc<str>,
  pub nickname: Arc<str>,
  pub groups: Option<Vec<Group>>,
  pub description: Arc<str>,
  pub club: Club,
  pub club_positions: Vec<Arc<str>>
}

impl Member {
  pub fn default() -> Member {
    Member {
      id: "N/A".into(),
      avatar: "N/A".into(),
      name: "N/A".into(),
      nickname: "N/A".into(),
      groups: None,
      description: "N/A".into(),
      club: Club::default(),
      club_positions: vec![]
    }
  }

  pub async fn from_json(json_data: &Value) -> Result<Member> {
    let properties: &Value = &json_data["properties"];

    let mut groups: Vec<Group> = Vec::new();
    for json_data in properties["groups"]["relation"].as_array().ok_or(
      anyhow!("Get `position` failed.")
    )?.into_iter() {
      groups.push(
        match CacheStorage::get().request(
          json_data["id"].as_str().unwrap_or(""),
          NotionDataType::Group
        ).await {
          NotionData::Group(data) => data,
          _ => Group::default()
        }
      )
    }

    Ok(
      Member {
        id: json_data["id"]
          .as_str()
          .ok_or(
            anyhow!("Get `id` failed.")
          )?
          .into(),
        avatar: properties["avatar"]["files"][0]["external"]["url"]
          .as_str()
          .ok_or(
            anyhow!("Get `avatar` failed.")
          )?
          .into(),
        name: properties["name"]["title"][0]["plain_text"]
          .as_str()
          .ok_or(
            anyhow!("Get `name` failed.")
          )?
          .into(),
        nickname: properties["nickname"]["rich_text"][0]["plain_text"]
          .as_str()
          .ok_or(
            anyhow!("Get `nickname` failed.")
          )?
          .into(),
        groups: Some(groups),
        description: properties["description"]["rich_text"][0]["plain_text"]
          .as_str()
          .ok_or(
            anyhow!("Get `description` failed.")
          )?
          .into(),
        club: match CacheStorage::get().request(
          properties["club"]["relation"][0]["id"].as_str().unwrap_or(""),
          NotionDataType::Club
        ).await {
          NotionData::Club(data) => data,
          _ => Club::default()
        },
        club_positions: properties["club_positions"]["multi_select"]
          .as_array()
          .ok_or(
            anyhow!("Get `club_positions` failed.")
          )?
          .into_iter()
          .map(
            |d| {
              d["name"].as_str().unwrap_or("N/A").into()
            }
          )
          .collect()
      }
    )
  }
}

#[derive(Debug, Clone)]
pub struct Group {
  pub id: Arc<str>,
  pub name: Arc<str>,
  pub description: Arc<str>,
  pub members: Option<Vec<Member>>
}

impl Group {
  pub fn default() -> Group {
    Group {
      id: "N/A".into(),
      name: "N/A".into(),
      description: "N/A".into(),
      members: None
    }
  }

  pub async fn from_json(json_data: &Value) -> Result<Group> {
    let properties: &Value = &json_data["properties"];

    let mut members: Vec<Member> = Vec::new();
    for json_data in properties["members"]["relation"].as_array().ok_or(
      anyhow!("Get `members` failed.")
    )?.into_iter() {
      members.push(
        match CacheStorage::get().request(
          json_data["id"].as_str().unwrap_or(""),
          NotionDataType::Member
        ).await {
          NotionData::Member(data) => data,
          _ => Member::default()
        }
      )
    }

    Ok(
      Group {
        id: json_data["id"]
          .as_str()
          .ok_or(
            anyhow!("Get `id` failed.")
          )?
          .into(),
        name: properties["name"]["title"][0]["plain_text"]
          .as_str()
          .ok_or(
            anyhow!("Get `name` failed.")
          )?
          .into(),
        description: properties["description"]["rich_text"][0]["plain_text"]
          .as_str()
          .ok_or(
            anyhow!("Get `description` failed.")
          )?
          .into(),
        members: Some(members),
      }
    )
  }
}

#[derive(Debug, Clone)]
pub struct Club {
  pub id: Arc<str>,
  pub name: Arc<str>,
  pub description: Arc<str>,
  pub school: Arc<str>,
  pub instagram_id: Arc<str>,
  pub icon: Arc<str>
}

impl Club {
  pub fn default() -> Club {
    Club {
      id: "N/A".into(),
      name: "N/A".into(),
      description: "N/A".into(),
      school: "N/A".into(),
      instagram_id: "N/A".into(),
      icon: "N/A".into()
    }
  }

  pub async fn from_json(json_data: &Value) -> Result<Club> {
    let properties: &Value = &json_data["properties"];

    Ok(
      Club {
        id: json_data["id"]
          .as_str()
          .ok_or(
            anyhow!("Get `id` failed.")
          )?
          .into(),
        name: properties["name"]["title"][0]["plain_text"]
          .as_str()
          .ok_or(
            anyhow!("Get `name` failed.")
          )?
          .into(),
        description: properties["description"]["rich_text"][0]["plain_text"]
          .as_str()
          .ok_or(
            anyhow!("Get `description` failed.")
          )?
          .into(),
        school: properties["school"]["select"]["name"]
          .as_str()
          .ok_or(
            anyhow!("Get `school` failed.")
          )?
          .into(),
        instagram_id: properties["instagram_id"]["rich_text"][0]["plain_text"]
          .as_str()
          .ok_or(
            anyhow!("Get `instagram_id` failed.")
          )?
          .into(),
        icon: properties["icon"]["files"][0]["external"]["url"]
          .as_str()
          .ok_or(
            anyhow!("Get `icon` failed.")
          )?
          .into(),
      }
    )
  }
}

#[derive(Debug, Clone)]
pub struct Event {
  pub id: Arc<str>,
  pub date: EventPeriod,
  pub name: Arc<str>,
  pub description: Arc<str>,
  pub thumbnail: Arc<str>,
  pub principal: Vec<Member>
}

impl Event {
  pub fn default() -> Event {
    Event {
      id: "N/A".into(),
      date: EventPeriod::default(),
      name: "N/A".into(),
      description: "N/A".into(),
      thumbnail: "N/A".into(),
      principal: vec![]
    }
  }
  
  pub async fn from_json(json_data: &Value) -> Result<Event> {
    let properties: &Value = &json_data["properties"];

    let mut principal: Vec<Member> = Vec::new();
    for json_data in properties["principal"]["relation"].as_array().ok_or(
      anyhow!("Get `principal` failed.")
    )?.into_iter() {
      principal.push(
        match CacheStorage::get().request(
          json_data["id"].as_str().unwrap_or(""),
          NotionDataType::Member
        ).await {
          NotionData::Member(data) => data,
          _ => Member::default()
        }
      )
    }

    Ok(
      Event {
        id: json_data["id"]
          .as_str()
          .ok_or(
            anyhow!("Get `id` failed.")
          )?
          .into(),
        date: EventPeriod::from_json(
          &properties["date"]["date"]
        )?,
        name: properties["name"]["title"][0]["plain_text"]
          .as_str()
          .ok_or(
            anyhow!("Get `name` failed.")
          )?
          .into(),
        description: properties["description"]["rich_text"][0]["plain_text"]
          .as_str()
          .ok_or(
            anyhow!("Get `description` failed.")
          )?
          .into(),
        thumbnail: properties["thumbnail"]["files"][0]["external"]["url"]
          .as_str()
          .ok_or(
            anyhow!("Get `thumbnail` failed.")
          )?
          .into(),
        principal
      }
    )
  }
}

#[derive(Debug, Clone)]
pub struct Article {
  pub id: Arc<str>,
  pub title: Arc<str>,
  pub content: Arc<str>,
  pub description: Arc<str>,
  pub tags: Vec<Arc<str>>,
  pub created_at: Arc<str>,
  pub updated_at: Arc<str>
}

impl Article {
  pub fn default() -> Article {
    Article {
      id: "N/A".into(),
      title: "N/A".into(),
      description: "N/A".into(),
      content: "N/A".into(),
      tags: vec![],
      created_at: "N/A".into(),
      updated_at: "N/A".into(),
    }
  }
  
  pub async fn from_json(json_data: &Value) -> Result<Article> {
    let properties: &Value = &json_data["properties"];

    Ok(
      Article {
        id: json_data["id"]
          .as_str()
          .ok_or(
            anyhow!("Get `id` failed.")
          )?
          .into(),
        title: properties["title"]["title"][0]["plain_text"]
          .as_str()
          .ok_or(
            anyhow!("Get `title` failed.")
          )?
          .into(),
        description: properties["description"]["rich_text"][0]["plain_text"]
          .as_str()
          .ok_or(
            anyhow!("Get `description` failed.")
          )?
          .into(),
        content: "Todo".into(),
        tags: properties["tags"]["multi_select"]
          .as_array()
          .ok_or(
            anyhow!("Get `tags` failed.")
          )?
          .into_iter()
          .map(
            |d| {
              d["name"].as_str().unwrap_or("N/A").into()
            }
          )
          .collect(),
        created_at: properties["created_at"]["created_time"]
          .as_str()
          .ok_or(
            anyhow!("Get `created_at` failed.")
          )?
          .into(),
          updated_at: properties["updated_at"]["last_edited_time"]
          .as_str()
          .ok_or(
            anyhow!("Get `updated_at` failed.")
          )?
          .into(),
      }
    )
  }
}

#[derive(Debug, Clone)]
pub struct Sponsor {
  pub id: Arc<str>,
  pub name: Arc<str>,
  pub icon: Arc<str>,
  pub url: Arc<str>,
  pub description: Arc<str>
}

impl Sponsor {
  pub fn default() -> Sponsor {
    Sponsor {
      id: "N/A".into(),
      name: "N/A".into(),
      description: "N/A".into(),
      icon: "N/A".into(),
      url: "N/A".into(),
    }
  }

  pub async fn from_json(json_data: &Value) -> Result<Sponsor> {
    let properties: &Value = &json_data["properties"];

    Ok(
      Sponsor {
        id: json_data["id"]
          .as_str()
          .ok_or(
            anyhow!("Get `id` failed.")
          )?
          .into(),
        name: properties["name"]["title"][0]["plain_text"]
          .as_str()
          .ok_or(
            anyhow!("Get `name` failed.")
          )?
          .into(),
        description: properties["description"]["rich_text"][0]["plain_text"]
          .as_str()
          .ok_or(
            anyhow!("Get `description` failed.")
          )?
          .into(),
        url: properties["url"]["url"]
          .as_str()
          .ok_or(
            anyhow!("Get `url` failed.")
          )?
          .into(),
        icon: properties["icon"]["files"][0]["external"]["url"]
          .as_str()
          .ok_or(
            anyhow!("Get `icon` failed.")
          )?
          .into(),
      }
    )
  }
}
