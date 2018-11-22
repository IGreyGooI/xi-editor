// Copyright 2018 The xi-editor Authors.
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

//! Helper utility for assigning a unique id to a particular [Scope]

use std::collections::HashMap;

use Scope;

#[derive(Default)]
pub struct ScopeTracker {
    elements: HashMap<Scope, u32>,
    next_id: u32,
}

impl ScopeTracker {
    pub fn lookup(&mut self, scope: &Scope) -> LookupResult {
        if let Some(id) = self.elements.get(scope) {
            return LookupResult::Existing(*id);
        }

        let old_id = self.next_id;
        self.next_id += 1;

        self.elements.insert(scope.clone(), old_id);
        LookupResult::New(old_id)
    }
}

pub enum LookupResult {
    Existing(u32),
    New(u32),
}
