use js_sys::Array;
use js_sys::JSON;
use std::mem;
use uuid::Uuid;
use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = wasm_bindgen)]
    fn memory() -> JsValue;
}

#[wasm_bindgen]
pub fn get_memory_size_mb() -> f64 {
    let mem = wasm_bindgen::memory();
    let bytes = js_sys::Reflect::get(&mem, &JsValue::from_str("buffer"))
        .ok()
        .and_then(|buf| js_sys::Reflect::get(&buf, &JsValue::from_str("byteLength")).ok())
        .and_then(|val| val.as_f64())
        .unwrap_or(0.0);
    bytes / 1024.0 / 1024.0
}

#[wasm_bindgen]
pub fn get_instance_ptr(instance: &SubTree) -> u32 {
    instance as *const SubTree as u32
}

fn generate_uuid() -> String {
    Uuid::new_v4().to_string()
}

// --- Property ---
#[wasm_bindgen]
#[derive(Clone)]
pub struct Property {
    id: String,
    name: String,
    value: String,
    class_name: String,
}

#[wasm_bindgen]
impl Property {
    #[wasm_bindgen(constructor)]
    pub fn new(
        name: String,
        value: String,
        class_name: Option<String>,
        id: Option<String>,
    ) -> Property {
        Property {
            id: id.unwrap_or_else(generate_uuid),
            name,
            value,
            class_name: class_name.unwrap_or_default(),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn id(&self) -> String {
        self.id.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn value(&self) -> String {
        self.value.clone()
    }

    // Add getter for className (using js_name to match JavaScript convention)
    #[wasm_bindgen(getter, js_name = className)]
    pub fn class_name(&self) -> String {
        self.class_name.clone()
    }

    #[wasm_bindgen]
    pub fn set_value(&mut self, val: String) {
        self.value = val;
    }

    // Add setter for className
    #[wasm_bindgen(js_name = setClassName)]
    pub fn set_class_name(&mut self, class_name: String) {
        self.class_name = class_name;
    }
}

// --- SubTreeNode ---
#[wasm_bindgen]
pub struct SubTreeNode {
    id: String,
    value: String,
    children: Vec<SubTreeNode>,
    properties: Vec<Property>,
}

impl Clone for SubTreeNode {
    fn clone(&self) -> Self {
        SubTreeNode {
            id: self.id.clone(),
            value: self.value.clone(),
            children: self.children.clone(),
            properties: self.properties.clone(),
        }
    }
}

#[wasm_bindgen]
impl SubTreeNode {
    #[wasm_bindgen(constructor)]
    pub fn new(value: String, id: Option<String>) -> SubTreeNode {
        SubTreeNode {
            id: id.unwrap_or_else(generate_uuid),
            value,
            children: Vec::new(),
            properties: Vec::new(),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn id(&self) -> String {
        self.id.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn value(&self) -> String {
        self.value.clone()
    }

    // Keep these for backward compatibility, but discourage their use
    #[wasm_bindgen]
    pub fn add_child(&mut self, child: &SubTreeNode) {
        self.children.push(child.clone());
    }

    #[wasm_bindgen]
    pub fn get_children(&self) -> Array {
        self.children.iter().cloned().map(JsValue::from).collect()
    }

    #[wasm_bindgen(js_name = set_property)]
    pub fn set_property(&mut self, name: &str, value: JsValue) {
        let string_value = if value.is_string() {
            value.as_string().unwrap_or_default()
        } else {
            match JSON::stringify(&value) {
                Ok(s) => s.as_string().unwrap_or_default(),
                Err(_) => format!("{:?}", value),
            }
        };

        console::log_2(
            &JsValue::from_str("🦀 set_property →"),
            &JsValue::from_str(&string_value),
        );

        if let Some(prop) = self.properties.iter_mut().find(|p| p.name == name) {
            prop.set_value(string_value);
        } else {
            // Note: When creating new properties via set_property, className will be empty
            // You might want to add a set_property_with_class method if you need to set className here
            self.properties
                .push(Property::new(name.to_string(), string_value, None, None));
        }
    }

    // Add this method to SubTreeNode
    #[wasm_bindgen]
    pub fn set_property_with_class_name(
        &mut self,
        name: &str,
        value: JsValue,
        class_name: Option<String>,
    ) {
        let string_value = if value.is_string() {
            value.as_string().unwrap_or_default()
        } else {
            match JSON::stringify(&value) {
                Ok(s) => s.as_string().unwrap_or_default(),
                Err(_) => format!("{:?}", value),
            }
        };

        if let Some(prop) = self.properties.iter_mut().find(|p| p.name == name) {
            prop.set_value(string_value);
            if let Some(cn) = class_name {
                prop.set_class_name(cn);
            }
        } else {
            self.properties.push(Property::new(
                name.to_string(),
                string_value,
                class_name,
                None,
            ));
        }
    }

    #[wasm_bindgen]
    pub fn get_property(&self, name: String) -> JsValue {
        self.properties
            .iter()
            .find(|p| p.name == name)
            .map(|p| {
                // try to parse as JSON first, otherwise return as string
                if let Ok(parsed) = JSON::parse(&p.value) {
                    parsed
                } else {
                    JsValue::from_str(&p.value)
                }
            })
            .unwrap_or(JsValue::undefined())
    }

    #[wasm_bindgen]
    pub fn get_property_count(&self) -> usize {
        console::log_2(
            &JsValue::from_str("🦀 get_property_count →"),
            &JsValue::from_f64(self.properties.len() as f64),
        );
        self.properties.len()
    }

    #[wasm_bindgen]
    pub fn get_property_id(&self, idx: usize) -> String {
        self.properties
            .get(idx)
            .map(|p| p.id.clone())
            .unwrap_or_default()
    }

    #[wasm_bindgen]
    pub fn get_property_name(&self, idx: usize) -> String {
        self.properties
            .get(idx)
            .map(|p| p.name.clone())
            .unwrap_or_default()
    }

    #[wasm_bindgen]
    pub fn get_property_value(&self, idx: usize) -> JsValue {
        self.properties
            .get(idx)
            .map(|p| {
                if let Ok(parsed) = JSON::parse(&p.value) {
                    parsed
                } else {
                    JsValue::from_str(&p.value)
                }
            })
            .unwrap_or(JsValue::undefined())
    }

    #[wasm_bindgen]
    pub fn get_property_class_name(&self, idx: usize) -> String {
        self.properties
            .get(idx)
            .map(|p| p.class_name.clone())
            .unwrap_or_default()
    }

    #[wasm_bindgen]
    pub fn has_property(&self, name: String) -> bool {
        self.properties.iter().any(|p| p.name == name)
    }

    // ========== NEW: size_state helpers on node ==========
    // Store a JS object (SizeState) on the node as a property named "size_state"
    #[wasm_bindgen(js_name = set_size_state)]
    pub fn set_size_state(&mut self, state: &JsValue) {
        // stringify the JS value for storage
        let string_value = match JSON::stringify(state) {
            Ok(s) => s.as_string().unwrap_or_default(),
            Err(_) => String::new(),
        };

        if let Some(prop) = self.properties.iter_mut().find(|p| p.name == "size_state") {
            prop.set_value(string_value);
        } else {
            self.properties.push(Property::new(
                "size_state".to_string(),
                string_value,
                None,
                None,
            ));
        }
    }

    #[wasm_bindgen(js_name = get_size_state)]
    pub fn get_size_state(&self) -> JsValue {
        if let Some(prop) = self.properties.iter().find(|p| p.name == "size_state") {
            // try to parse the stored string back into JsValue
            match JSON::parse(&prop.value) {
                Ok(parsed) => parsed,
                Err(_) => JsValue::from_str(&prop.value),
            }
        } else {
            JsValue::undefined()
        }
    }

    // ========== NEW: tailwind_size helpers on node ==========
    #[wasm_bindgen(js_name = set_tailwind_size)]
    pub fn set_tailwind_size(&mut self, tailwind: &str) {
        if let Some(prop) = self
            .properties
            .iter_mut()
            .find(|p| p.name == "tailwind_size")
        {
            prop.set_value(tailwind.to_string());
        } else {
            self.properties.push(Property::new(
                "tailwind_size".to_string(),
                tailwind.to_string(),
                None,
                None,
            ));
        }
    }

    #[wasm_bindgen(js_name = get_tailwind_size)]
    pub fn get_tailwind_size(&self) -> JsValue {
        if let Some(prop) = self.properties.iter().find(|p| p.name == "tailwind_size") {
            JsValue::from_str(&prop.value)
        } else {
            JsValue::undefined()
        }
    }

    // ========== NEW: style_state helpers on node ==========
    #[wasm_bindgen(js_name = set_style_state)]
    pub fn set_style_state(&mut self, state: &JsValue) {
        // stringify the JS value for storage
        let string_value = match JSON::stringify(state) {
            Ok(s) => s.as_string().unwrap_or_default(),
            Err(_) => String::new(),
        };

        if let Some(prop) = self.properties.iter_mut().find(|p| p.name == "style_state") {
            prop.set_value(string_value);
        } else {
            self.properties.push(Property::new(
                "style_state".to_string(),
                string_value,
                None,
                None,
            ));
        }
    }

    #[wasm_bindgen(js_name = get_style_state)]
    pub fn get_style_state(&self) -> JsValue {
        if let Some(prop) = self.properties.iter().find(|p| p.name == "style_state") {
            // try to parse the stored string back into JsValue
            match JSON::parse(&prop.value) {
                Ok(parsed) => parsed,
                Err(_) => JsValue::from_str(&prop.value),
            }
        } else {
            JsValue::undefined()
        }
    }

    #[wasm_bindgen(js_name = set_tailwind_style)]
    pub fn set_tailwind_style(&mut self, tailwind: &str) {
        if let Some(prop) = self
            .properties
            .iter_mut()
            .find(|p| p.name == "tailwind_style")
        {
            prop.set_value(tailwind.to_string());
        } else {
            self.properties.push(Property::new(
                "tailwind_style".to_string(),
                tailwind.to_string(),
                None,
                None,
            ));
        }
    }

    #[wasm_bindgen(js_name = get_tailwind_style)]
    pub fn get_tailwind_style(&self) -> JsValue {
        if let Some(prop) = self.properties.iter().find(|p| p.name == "tailwind_style") {
            JsValue::from_str(&prop.value)
        } else {
            JsValue::undefined()
        }
    }
}

// --- SubTree with In-Place Operations ---
#[wasm_bindgen]
pub struct SubTree {
    root: Option<SubTreeNode>,
}

#[wasm_bindgen]
impl SubTree {
    #[wasm_bindgen(constructor)]
    pub fn new() -> SubTree {
        SubTree { root: None }
    }

    #[wasm_bindgen]
    pub fn add_node(
        &mut self,
        value: String,
        parent_id: Option<String>,
        node_id: Option<String>,
    ) -> SubTreeNode {
        let node = SubTreeNode::new(value, node_id);
        if let Some(pid) = parent_id {
            // Safe handling without panicking
            if let Some(root) = self.root.as_mut() {
                if let Some(parent) = SubTree::find_node_internal_mut(root, &pid) {
                    parent.add_child(&node);
                } else {
                    console::error_1(&JsValue::from_str(&format!("Parent not found: {}", pid)));
                    // Still return the node even if parent wasn't found
                    return node;
                }
            } else {
                console::error_1(&JsValue::from_str("Tree is empty - cannot add to parent"));
                // Return node anyway, could be added as root later
                return node;
            }
        } else {
            self.root = Some(node.clone());
        }
        node // Remove the Ok() wrapper since return type is SubTreeNode, not Result
    }

    // ========== NEW IN-PLACE OPERATIONS ==========

    /// Set a property on a node without cloning the node
    #[wasm_bindgen]
    pub fn set_node_property(&mut self, node_id: String, name: String, value: JsValue) -> bool {
        console::log_3(
            &JsValue::from_str("🦀 set_node_property → "),
            &JsValue::from_str(&node_id),
            &JsValue::from_str(&name),
        );

        if let Some(root) = self.root.as_mut() {
            if let Some(node) = SubTree::find_node_internal_mut(root, &node_id) {
                node.set_property(&name, value);
                return true;
            }
        }
        false
    }

    // And add this to SubTree for the in-place operations
    #[wasm_bindgen]
    pub fn set_node_property_with_class_name(
        &mut self,
        node_id: String,
        name: String,
        value: JsValue,
        class_name: Option<String>,
    ) -> bool {
        console::log_4(
            &JsValue::from_str("🦀 set_node_property_with_class_name →"),
            &JsValue::from_str(&node_id),
            &JsValue::from_str(&name),
            &class_name
                .as_ref()
                .map(|cn| JsValue::from_str(cn))
                .unwrap_or(JsValue::null()),
        );

        if let Some(root) = self.root.as_mut() {
            if let Some(node) = SubTree::find_node_internal_mut(root, &node_id) {
                node.set_property_with_class_name(&name, value, class_name);
                return true;
            }
        }
        false
    }

    /// Get a property from a node without cloning the node
    #[wasm_bindgen]
    pub fn get_node_property(&self, node_id: String, name: String) -> JsValue {
        if let Some(root) = &self.root {
            if let Some(node) = SubTree::find_node_internal(root, &node_id) {
                return node.get_property(name);
            }
        }
        JsValue::undefined()
    }

    /// Get property ID by index without cloning the node
    #[wasm_bindgen]
    pub fn get_node_property_id(&self, node_id: String, idx: usize) -> String {
        if let Some(root) = &self.root {
            if let Some(node) = SubTree::find_node_internal(root, &node_id) {
                return node.get_property_id(idx);
            }
        }
        String::new()
    }

    /// Get property count for a node without cloning the node
    #[wasm_bindgen]
    pub fn get_node_property_count(&self, node_id: String) -> usize {
        if let Some(root) = &self.root {
            if let Some(node) = SubTree::find_node_internal(root, &node_id) {
                return node.properties.len();
            }
        }
        0
    }

    /// Get property name by index without cloning the node
    #[wasm_bindgen]
    pub fn get_node_property_name(&self, node_id: String, idx: usize) -> String {
        if let Some(root) = &self.root {
            if let Some(node) = SubTree::find_node_internal(root, &node_id) {
                return node.get_property_name(idx);
            }
        }
        String::new()
    }

    /// Get property value by index without cloning the node
    #[wasm_bindgen]
    pub fn get_node_property_value(&self, node_id: String, idx: usize) -> JsValue {
        if let Some(root) = &self.root {
            if let Some(node) = SubTree::find_node_internal(root, &node_id) {
                return node.get_property_value(idx);
            }
        }
        JsValue::undefined()
    }

    /// Get property className by index without cloning the node
    #[wasm_bindgen]
    pub fn get_node_property_class_name(&self, node_id: String, idx: usize) -> String {
        if let Some(root) = &self.root {
            if let Some(node) = SubTree::find_node_internal(root, &node_id) {
                return node.get_property_class_name(idx);
            }
        }
        String::new()
    }

    /// Get all child IDs for a node without cloning
    #[wasm_bindgen]
    pub fn get_node_child_ids(&self, node_id: String) -> Array {
        if let Some(root) = &self.root {
            if let Some(node) = SubTree::find_node_internal(root, &node_id) {
                return node
                    .children
                    .iter()
                    .map(|child| JsValue::from_str(&child.id))
                    .collect();
            }
        }
        Array::new()
    }

    /// Check if a node exists
    #[wasm_bindgen]
    pub fn node_exists(&self, node_id: String) -> bool {
        if let Some(root) = &self.root {
            return SubTree::find_node_internal(root, &node_id).is_some();
        }
        false
    }

    /// Get node value without cloning
    #[wasm_bindgen]
    pub fn get_node_value(&self, node_id: String) -> String {
        if let Some(root) = &self.root {
            if let Some(node) = SubTree::find_node_internal(root, &node_id) {
                return node.value.clone();
            }
        }
        String::new()
    }

    // ========== NEW IN-PLACE helpers for size_state + tailwind_size ==========

    /// Set size_state (JS object) on a node without cloning
    #[wasm_bindgen(js_name = set_node_size_state)]
    pub fn set_node_size_state(&mut self, node_id: String, state: &JsValue) -> bool {
        console::log_2(
            &JsValue::from_str("🦀 set_node_size_state →"),
            &JsValue::from_str(&node_id),
        );
        if let Some(root) = self.root.as_mut() {
            if let Some(node) = SubTree::find_node_internal_mut(root, &node_id) {
                node.set_size_state(state);
                return true;
            }
        }
        false
    }

    /// Get size_state (JS object) from a node without cloning
    #[wasm_bindgen(js_name = get_node_size_state)]
    pub fn get_node_size_state(&self, node_id: String) -> JsValue {
        if let Some(root) = &self.root {
            if let Some(node) = SubTree::find_node_internal(root, &node_id) {
                return node.get_size_state();
            }
        }
        JsValue::undefined()
    }

    /// Set tailwind_size string on a node without cloning
    #[wasm_bindgen(js_name = set_node_tailwind_size)]
    pub fn set_node_tailwind_size(&mut self, node_id: String, tailwind: &str) -> bool {
        console::log_2(
            &JsValue::from_str("🦀 set_node_tailwind_size →"),
            &JsValue::from_str(&node_id),
        );
        if let Some(root) = self.root.as_mut() {
            if let Some(node) = SubTree::find_node_internal_mut(root, &node_id) {
                node.set_tailwind_size(tailwind);
                return true;
            }
        }
        false
    }

    /// Get tailwind_size string from a node without cloning
    #[wasm_bindgen(js_name = get_node_tailwind_size)]
    pub fn get_node_tailwind_size(&self, node_id: String) -> JsValue {
        if let Some(root) = &self.root {
            if let Some(node) = SubTree::find_node_internal(root, &node_id) {
                return node.get_tailwind_size();
            }
        }
        JsValue::undefined()
    }

    // ========== LEGACY METHODS (discouraged but kept for compatibility) ==========

    /// ⚠️ LEGACY: Returns a clone - properties might not persist
    #[wasm_bindgen]
    pub fn find_node(&self, id: String) -> Option<SubTreeNode> {
        console::log_1(&JsValue::from_str(
            "⚠️ Using legacy find_node - properties may not persist",
        ));
        self.root
            .as_ref()
            .and_then(|r| SubTree::find_node_internal(r, &id))
    }

    /// ⚠️ LEGACY: Returns a clone - properties might not persist  
    #[wasm_bindgen]
    pub fn get_root(&self) -> Option<SubTreeNode> {
        console::log_1(&JsValue::from_str(
            "⚠️ Using legacy get_root - properties may not persist",
        ));
        self.root.clone()
    }

    // ========== STANDARD TREE OPERATIONS ==========

    #[wasm_bindgen]
    pub fn remove_node(&mut self, id: String) {
        if let Some(root) = &mut self.root {
            if root.id == id {
                self.root = None;
                return;
            }
            SubTree::remove_node_internal(root, &id);
        }
    }

    #[wasm_bindgen]
    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    #[wasm_bindgen]
    pub fn get_node_count(&self) -> usize {
        self.root.as_ref().map_or(0, |r| SubTree::count_internal(r))
    }

    #[wasm_bindgen]
    pub fn get_height(&self) -> usize {
        self.root
            .as_ref()
            .map_or(0, |r| SubTree::height_internal(r))
    }

    #[wasm_bindgen]
    pub fn is_balanced(&self) -> bool {
        self.root
            .as_ref()
            .map_or(true, |r| SubTree::balanced_internal(r))
    }

    #[wasm_bindgen]
    pub fn clear_subtree_except_root(&mut self) {
        console::log_1(&"🚨🦀 CLEARING SUBTREE - BEFORE".into());
        if let Some(root) = self.root.as_ref() {
            console::log_2(
                &"🚨🦀 Children count before clear:".into(),
                &JsValue::from_f64(root.children.len() as f64),
            );
        }

        if let Some(root) = self.root.as_mut() {
            let old_children = mem::take(&mut root.children);
            drop(old_children);
        }

        console::log_1(&"🚨🦀 CLEARING SUBTREE - AFTER".into());
    }

    // ========== INTERNAL HELPER METHODS ==========

    fn find_node_internal(current: &SubTreeNode, target: &str) -> Option<SubTreeNode> {
        if current.id == target {
            return Some(current.clone());
        }
        for c in &current.children {
            if let Some(found) = SubTree::find_node_internal(c, target) {
                return Some(found);
            }
        }
        None
    }

    fn find_node_internal_mut<'a>(
        current: &'a mut SubTreeNode,
        target: &str,
    ) -> Option<&'a mut SubTreeNode> {
        if current.id == target {
            return Some(current);
        }
        for c in &mut current.children {
            if let Some(found) = SubTree::find_node_internal_mut(c, target) {
                return Some(found);
            }
        }
        None
    }

    fn remove_node_internal(current: &mut SubTreeNode, target: &str) -> bool {
        if let Some(pos) = current.children.iter().position(|c| c.id == target) {
            current.children.remove(pos);
            return true;
        }
        for c in &mut current.children {
            if SubTree::remove_node_internal(c, target) {
                return true;
            }
        }
        false
    }

    fn count_internal(node: &SubTreeNode) -> usize {
        1 + node
            .children
            .iter()
            .map(|c| SubTree::count_internal(c))
            .sum::<usize>()
    }

    fn height_internal(node: &SubTreeNode) -> usize {
        let h = node
            .children
            .iter()
            .map(|c| SubTree::height_internal(c))
            .max()
            .unwrap_or(0);
        1 + h
    }

    fn balanced_internal(node: &SubTreeNode) -> bool {
        let children = &node.children;
        if children.is_empty() {
            return true;
        }
        let heights: Vec<_> = children
            .iter()
            .map(|c| SubTree::height_internal(c))
            .collect();
        let min_h = heights.iter().min().copied().unwrap_or(0);
        let max_h = heights.iter().max().copied().unwrap_or(0);
        (max_h - min_h <= 1) && children.iter().all(|c| SubTree::balanced_internal(c))
    }
}
