use std::{sync::{Arc, OnceLock}, env};

use serde::Serialize;
use serde_json::Value;
use anyhow::{Result, anyhow};

use super::cache::CacheStorage;


pub static MEMBER_DATABASE_ID: OnceLock<Arc<str>> = OnceLock::new();
pub static GROUP_DATABASE_ID: OnceLock<Arc<str>> = OnceLock::new();
pub static CLUB_DATABASE_ID: OnceLock<Arc<str>> = OnceLock::new();
pub static EVENT_DATABASE_ID: OnceLock<Arc<str>> = OnceLock::new();
pub static ARTICLE_DATABASE_ID: OnceLock<Arc<str>> = OnceLock::new();
pub static SPONSOR_DATABASE_ID: OnceLock<Arc<str>> = OnceLock::new();


#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all(serialize = "snake_case"))]
pub enum NotionDataType {
  Member, Group, Club, Event, Article, Sponsor
}

impl NotionDataType {
  pub fn iterator() -> std::array::IntoIter<NotionDataType, 6> {
      static DIRECTIONS: [NotionDataType; 6] = [
        NotionDataType::Club,
        NotionDataType::Group,
        NotionDataType::Member,
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


#[derive(Debug, Clone, Serialize)]
#[serde(untagged, rename_all(serialize = "snake_case"))]
pub enum NotionData {
  Member(Member),
  Group(Group),
  Club(Club),
  Event(Event),
  Article(Article),
  Sponsor(Sponsor)
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all(serialize = "snake_case"))]
pub struct EventPeriod {
  start: String,
  end: String
}

impl EventPeriod {
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

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all(serialize = "snake_case"))]
pub struct Member {
  pub id: String,
  pub avatar: String,
  pub name: String,
  pub nickname: String,
  pub groups: Option<Vec<Group>>,
  pub description: String,
  pub club: Option<Club>,
  pub club_positions: Vec<String>
}

impl Member {
  pub async fn from_json(json_data: &Value) -> Result<Member> {
    let properties: &Value = &json_data["properties"];

    let mut groups: Vec<Group> = Vec::new();
    for groups_data in properties["groups"]["relation"].as_array().ok_or(
      anyhow!("Get `groups` failed.")
    )?.into_iter() {
      groups.push(
        match CacheStorage::get().request(
          groups_data["id"].as_str().unwrap_or(""),
          &NotionDataType::Group
        ).await {
          Some(data) => match data {
            NotionData::Group(mut data) => {
              data.members = None;
              data
            },
            _ => Group::default()
          },
          None => Group::default()
        }
      );
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
            &NotionDataType::Club
          ).await {
            Some(data) => match data {
              NotionData::Club(data) => Some(data),
              _ => None
            },
            None => None
          },
        club_positions: properties["club_positions"]["multi_select"]
          .as_array()
          .ok_or(
            anyhow!("Get `club_positions` failed.")
          )?
          .into_iter()
          .map(
            |d: &Value| {
              d["name"].as_str().unwrap_or("N/A").into()
            }
          )
          .collect()
      }
    )
  }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all(serialize = "snake_case"))]
pub struct Group {
  pub id: String,
  name: String,
  description: String,
  members: Option<Vec<Member>>
}

impl Group {
  pub async fn from_json(json_data: &Value) -> Result<Group> {
    let properties: &Value = &json_data["properties"];

    let mut members: Vec<Member> = Vec::new();
    for members_data in properties["members"]["relation"].as_array().ok_or(
      anyhow!("Get `members` failed.")
    )?.into_iter() {
      members.push(
        match CacheStorage::get().request(
          members_data["id"].as_str().unwrap_or(""),
          &NotionDataType::Member
        ).await {
          Some(data) => match data {
            NotionData::Member(mut data) => {
              data.groups = None;
              data
            },
            _ => Member::default()
          },
          None => Member::default()
        }
      );
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
        members: Some(members)
      }
    )
  }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all(serialize = "snake_case"))]
pub struct Club {
  pub id: String,
  name: String,
  description: String,
  school: String,
  instagram_id: String,
  icon: String
}

impl Club {
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

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all(serialize = "snake_case"))]
pub struct Event {
  pub id: String,
  date: EventPeriod,
  name: String,
  description: String,
  thumbnail: String,
  principal: Vec<Member>
}

impl Event {
  pub async fn from_json(json_data: &Value) -> Result<Event> {
    let properties: &Value = &json_data["properties"];

    let mut principal: Vec<Member> = Vec::new();
    for principal_data in properties["principal"]["relation"].as_array().ok_or(
      anyhow!("Get `principal` failed.")
    )?.into_iter() {
      principal.push(
        match CacheStorage::get().request(
          principal_data["id"].as_str().unwrap_or(""),
          &NotionDataType::Member
        ).await {
          Some(data) => match data {
            NotionData::Member(data) => data,
            _ => Member::default()
          },
          None => Member::default()
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

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all(serialize = "snake_case"))]
pub struct Article {
  pub id: String,
  title: String,
  content: String,
  description: String,
  tags: Vec<String>,
  created_at: String,
  updated_at: String
}

impl Article {
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

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all(serialize = "snake_case"))]
pub struct Sponsor {
  pub id: String,
  name: String,
  icon: String,
  url: String,
  description: String
}

impl Sponsor {
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
