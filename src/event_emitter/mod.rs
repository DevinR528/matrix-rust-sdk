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

use crate::events::collections::all::RoomEvent;
use crate::models::Room;

pub trait EventEmitter {
    fn on_room_name(&mut self, _: &Room, _: &RoomEvent) {}
    /// Any event that alters the state of the room's members
    fn on_room_member(&mut self, _: &Room, _: &RoomEvent) {}
}
