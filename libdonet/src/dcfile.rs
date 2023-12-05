// DONET SOFTWARE
// Copyright (c) 2023, Donet Authors.
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License version 3.
// You should have received a copy of this license along
// with this source code in a file named "LICENSE."
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program; if not, write to the Free Software Foundation,
// Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.

use crate::globals;
use multimap::MultiMap;
use std::sync::{Arc, Mutex}; // thread safe

// --------- Field ---------- //

pub struct DCField {
    class: Option<Arc<Mutex<DClass>>>,
    _struct: Option<Arc<Mutex<DCStruct>>>,
    field_name: String,
    field_id: globals::FieldId,
    parent_is_dclass: bool,
    default_value_stale: bool,
    has_default_value: bool,
    default_value: Vec<u8>, // stored as byte array
    bogus_field: bool,
}

pub trait DCFieldInterface {
    fn new(name: &str, id: globals::FieldId) -> Self;
    fn generate_hash(&mut self);
    fn set_field_id(&mut self, id: globals::FieldId);
    fn set_field_name(&mut self, name: String);
    fn set_parent_struct(&mut self, parent: Arc<Mutex<DCStruct>>);
    fn set_parent_dclass(&mut self, parent: Arc<Mutex<DClass>>);
}

// ---------- Struct ---------- //

pub struct DCStruct {}

// ---------- DClass ---------- //

pub type FieldName2Field = MultiMap<String, Arc<Mutex<DCField>>>;
pub type FieldIndex2Field = MultiMap<globals::FieldId, Arc<Mutex<DCField>>>;

pub struct DClass {
    class_name: String,
    class_id: globals::DClassId,
    is_struct: bool,
    is_bogus_class: bool,

    class_parents: Vec<Arc<Mutex<DClass>>>,
    constructor: Option<Arc<Mutex<DCField>>>,
    fields: Vec<Arc<Mutex<DCField>>>,
    inherited_fields: Vec<Arc<Mutex<DCField>>>,
    field_name_2_field: FieldName2Field,
    field_index_2_field: FieldIndex2Field,
}

pub trait DClassInterface {
    fn new(name: &str, id: globals::DClassId) -> Self;
    fn generate_hash(&mut self);

    fn set_parent(&mut self, parent: Arc<Mutex<DClass>>);

    fn get_name(&mut self) -> String;
    fn get_class_id(&mut self) -> globals::DClassId;
    fn get_num_parents(&mut self) -> usize;
    fn get_parent(&mut self, index: usize) -> Option<Arc<Mutex<DClass>>>;
    fn has_constructor(&mut self) -> bool;
    fn get_constructor(&mut self) -> Option<Arc<Mutex<DCField>>>;
}

impl DClassInterface for DClass {
    fn new(name: &str, id: globals::DClassId) -> Self {
        DClass {
            class_name: name.to_owned(),
            class_id: id,
            is_struct: false,
            is_bogus_class: true,
            class_parents: vec![],
            constructor: None,
            fields: vec![],
            inherited_fields: vec![],
            field_name_2_field: MultiMap::new(),
            field_index_2_field: MultiMap::new(),
        }
    }

    fn generate_hash(&mut self) {
        todo!(); // TODO: Implement once hash gen is written
    }

    fn set_parent(&mut self, parent: Arc<Mutex<DClass>>) {
        self.class_parents.push(parent);
    }

    fn get_name(&mut self) -> String {
        self.class_name.clone()
    }

    fn get_class_id(&mut self) -> globals::DClassId {
        self.class_id
    }

    fn get_num_parents(&mut self) -> usize {
        self.class_parents.len()
    }

    fn get_parent(&mut self, index: usize) -> Option<Arc<Mutex<DClass>>> {
        // copy the reference inside the option instead of a reference to the reference
        self.class_parents.get(index).cloned()
    }

    fn has_constructor(&mut self) -> bool {
        self.constructor.is_some()
    }

    fn get_constructor(&mut self) -> Option<Arc<Mutex<DCField>>> {
        self.constructor.clone()
    }
}
