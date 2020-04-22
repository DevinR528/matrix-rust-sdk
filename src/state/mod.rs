// Copyright 2020 Damir Jelić
// Copyright 2020 The Matrix.org Foundation C.I.C.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::collections::HashMap;
use std::path::Path;

pub mod state_store;
pub use state_store::JsonStore;

use serde::{Deserialize, Serialize};

use crate::base_client::{Client as BaseClient, Token};
use crate::events::push_rules::Ruleset;
use crate::identifiers::{RoomId, UserId};
use crate::models::Room;
use crate::session::Session;
use crate::Result;
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ClientState {
    /// The current client session containing our user id, device id and access
    /// token.
    pub session: Option<Session>,
    /// The current sync token that should be used for the next sync call.
    pub sync_token: Option<Token>,
    /// A list of ignored users.
    pub ignored_users: Vec<UserId>,
    /// The push ruleset for the logged in user.
    pub push_ruleset: Option<Ruleset>,
}

impl ClientState {
    pub fn from_base_client(client: &BaseClient) -> ClientState {
        let BaseClient {
            session,
            sync_token,
            ignored_users,
            push_ruleset,
            ..
        } = client;
        Self {
            session: session.clone(),
            sync_token: sync_token.clone(),
            ignored_users: ignored_users.clone(),
            push_ruleset: push_ruleset.clone(),
        }
    }
}

/// Abstraction around the data store to avoid unnecessary request on client initialization.
#[async_trait::async_trait]
pub trait StateStore: Send + Sync {
    /// Set up connections or check files exist to load/save state.
    fn open(&self, path: &Path) -> Result<()>;
    /// Loads the state of `BaseClient` through `StateStore::Store` type.
    async fn load_client_state(&self, path: &Path) -> Result<ClientState>;
    /// Load the state of a single `Room` by `RoomId`.
    async fn load_room_state(&self, path: &Path, room_id: &RoomId) -> Result<Room>;
    /// Load the state of all `Room`s.
    ///
    /// This will be mapped over in the client in order to store `Room`s in an async safe way.
    async fn load_all_rooms(&self, path: &Path) -> Result<HashMap<RoomId, Room>>;
    /// Save the current state of the `BaseClient` using the `StateStore::Store` type.
    async fn store_client_state(&self, path: &Path, _: ClientState) -> Result<()>;
    /// Save the state a single `Room`.
    async fn store_room_state(&self, path: &Path, _: &Room) -> Result<()>;
}

#[cfg(test)]
mod test {
    use super::*;

    use std::collections::HashMap;
    use std::convert::TryFrom;

    use crate::identifiers::{RoomId, UserId};

    #[test]
    fn serialize() {
        let id = RoomId::try_from("!roomid:example.com").unwrap();
        let user = UserId::try_from("@example:example.com").unwrap();

        let room = Room::new(&id, &user);

        let state = ClientState {
            session: None,
            sync_token: Some("hello".into()),
            ignored_users: vec![user],
            push_ruleset: None,
        };
        assert_eq!(
            r#"{"session":null,"sync_token":"hello","ignored_users":["@example:example.com"],"push_ruleset":null}"#,
            serde_json::to_string(&state).unwrap()
        );

        let mut joined_rooms = HashMap::new();
        joined_rooms.insert(id, room);
        assert_eq!(
            r#"{
  "!roomid:example.com": {
    "room_id": "!roomid:example.com",
    "room_name": {
      "name": null,
      "canonical_alias": null,
      "aliases": [],
      "heroes": [],
      "joined_member_count": null,
      "invited_member_count": null
    },
    "own_user_id": "@example:example.com",
    "creator": null,
    "members": {},
    "typing_users": [],
    "power_levels": null,
    "encrypted": false,
    "unread_highlight": null,
    "unread_notifications": null,
    "tombstone": null
  }
}"#,
            serde_json::to_string_pretty(&joined_rooms).unwrap()
        );
    }

    #[test]
    fn deserialize() {
        let id = RoomId::try_from("!roomid:example.com").unwrap();
        let user = UserId::try_from("@example:example.com").unwrap();

        let room = Room::new(&id, &user);

        let state = ClientState {
            session: None,
            sync_token: Some("hello".into()),
            ignored_users: vec![user],
            push_ruleset: None,
        };
        let json = serde_json::to_string(&state).unwrap();

        assert_eq!(state, serde_json::from_str(&json).unwrap());

        let mut joined_rooms = HashMap::new();
        joined_rooms.insert(id, room);
        let json = serde_json::to_string(&joined_rooms).unwrap();

        assert_eq!(joined_rooms, serde_json::from_str(&json).unwrap());
    }
}
