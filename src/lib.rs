//! # categorical-coordination
//!
//! Category theory abstractions for multi-agent coordination. Models agents
//! as objects, messages/interactions as morphisms, and coordination protocols
//! as functors between categories, with natural transformations for protocol
//! evolution.

use std::collections::HashMap;
use std::fmt;

/// Unique identifier for objects in a category.
pub type ObjId = String;

/// Unique identifier for morphisms.
pub type MorphId = String;

/// A morphism in a category: an arrow from source to target.
#[derive(Clone, Debug, PartialEq)]
pub struct Morphism {
    pub id: MorphId,
    pub source: ObjId,
    pub target: ObjId,
}

impl Morphism {
    pub fn new(id: impl Into<String>, source: impl Into<String>, target: impl Into<String>) -> Self {
        Self { id: id.into(), source: source.into(), target: target.into() }
    }
}

impl fmt::Display for Morphism {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {} → {}", self.id, self.source, self.target)
    }
}

/// A category with objects, morphisms, and composition.
#[derive(Clone, Debug)]
pub struct Category {
    pub name: String,
    pub objects: Vec<ObjId>,
    pub morphisms: Vec<Morphism>,
    /// Composition table: (f.id, g.id) -> h.id where h = f ∘ g.
    /// Note: f ∘ g means "f after g" (g first, then f).
    pub composition: HashMap<(MorphId, MorphId), MorphId>,
    /// Identity morphism for each object.
    pub identities: HashMap<ObjId, Morphism>,
}

impl Category {
    /// Create a new empty category.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            objects: Vec::new(),
            morphisms: Vec::new(),
            composition: HashMap::new(),
            identities: HashMap::new(),
        }
    }

    /// Add an object to the category.
    pub fn add_object(&mut self, id: impl Into<String>) -> &mut Self {
        let id = id.into();
        if !self.objects.contains(&id) {
            self.objects.push(id.clone());
            let id_morph = Morphism::new(format!("id_{}", id), &id, &id);
            self.identities.insert(id, id_morph.clone());
            self.morphisms.push(id_morph);
        }
        self
    }

    /// Add a morphism to the category.
    pub fn add_morphism(&mut self, morph: Morphism) -> &mut Self {
        if !self.objects.contains(&morph.source) {
            self.add_object(&morph.source);
        }
        if !self.objects.contains(&morph.target) {
            self.add_object(&morph.target);
        }
        self.morphisms.push(morph);
        self
    }

    /// Define composition: compose(f, g) = f ∘ g (apply g then f).
    /// Result morphism is automatically created.
    pub fn compose(&mut self, f_id: &str, g_id: &str, result_id: impl Into<String>) {
        let f = self.morphisms.iter().find(|m| m.id == f_id).cloned();
        let g = self.morphisms.iter().find(|m| m.id == g_id).cloned();
        if let (Some(f), Some(g)) = (f, g) {
            // g must have target = f's source for f ∘ g
            let result = Morphism::new(result_id, &g.source, &f.target);
            self.composition.insert((f_id.into(), g_id.into()), result.id.clone());
            self.morphisms.push(result);
        }
    }

    /// Get the identity morphism for an object.
    pub fn identity(&self, obj: &str) -> Option<&Morphism> {
        self.identities.get(obj)
    }

    /// Find a morphism by id.
    pub fn find_morphism(&self, id: &str) -> Option<&Morphism> {
        self.morphisms.iter().find(|m| m.id == id)
    }

    /// Get all morphisms from source to target.
    pub fn hom(&self, source: &str, target: &str) -> Vec<&Morphism> {
        self.morphisms.iter()
            .filter(|m| m.source == source && m.target == target)
            .collect()
    }

    /// Verify identity laws: f ∘ id_A = f and id_B ∘ f = f.
    pub fn verify_identity_laws(&self) -> bool {
        for m in &self.morphisms {
            if m.source == m.target && m.id.starts_with("id_") {
                continue; // Skip identity morphisms themselves
            }
            let id_s = format!("id_{}", m.source);
            let id_t = format!("id_{}", m.target);

            // m ∘ id_source should be m
            if let Some(comp_id) = self.composition.get(&(m.id.clone(), id_s.clone())) {
                if *comp_id != m.id { return false; }
            }

            // id_target ∘ m should be m
            if let Some(comp_id) = self.composition.get(&(id_t, m.id.clone())) {
                if *comp_id != m.id { return false; }
            }
        }
        true
    }

    /// Number of objects.
    pub fn num_objects(&self) -> usize {
        self.objects.len()
    }

    /// Number of non-identity morphisms.
    pub fn num_nontrivial_morphisms(&self) -> usize {
        self.morphisms.iter().filter(|m| !m.id.starts_with("id_")).count()
    }
}

/// A functor between categories: maps objects and morphisms preserving structure.
#[derive(Clone, Debug)]
pub struct Functor {
    pub name: String,
    pub source_cat: String,
    pub target_cat: String,
    /// Object mapping: source_obj -> target_obj.
    pub obj_map: HashMap<ObjId, ObjId>,
    /// Morphism mapping: source_morph -> target_morph.
    pub morph_map: HashMap<MorphId, MorphId>,
}

impl Functor {
    /// Create a new functor.
    pub fn new(name: impl Into<String>, source: impl Into<String>, target: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            source_cat: source.into(),
            target_cat: target.into(),
            obj_map: HashMap::new(),
            morph_map: HashMap::new(),
        }
    }

    /// Map an object.
    pub fn map_object(&mut self, source: impl Into<String>, target: impl Into<String>) -> &mut Self {
        self.obj_map.insert(source.into(), target.into());
        self
    }

    /// Map a morphism.
    pub fn map_morphism(&mut self, source: impl Into<String>, target: impl Into<String>) -> &mut Self {
        self.morph_map.insert(source.into(), target.into());
        self
    }

    /// Verify functoriality: F(id) = id and F(f ∘ g) = F(f) ∘ F(g).
    pub fn verify_functoriality(&self, source: &Category, target: &Category) -> bool {
        // Check F(identity) = identity
        for (src_obj, tgt_obj) in &self.obj_map {
            let src_id = format!("id_{}", src_obj);
            let tgt_id = format!("id_{}", tgt_obj);
            if let Some(mapped) = self.morph_map.get(&src_id) {
                if mapped != &tgt_id { return false; }
            }
        }

        // Check F(f ∘ g) = F(f) ∘ F(g)
        for (f, g) in source.composition.keys() {
            if let (Some(f_map), Some(g_map)) = (self.morph_map.get(f), self.morph_map.get(g)) {
                // F(f ∘ g) should exist
                let comp_key = (f.clone(), g.clone());
                if let Some(comp_result) = source.composition.get(&comp_key) {
                    if let Some(fg_mapped) = self.morph_map.get(comp_result) {
                        // Check that F(f) ∘ F(g) exists in target
                        let target_key = (f_map.clone(), g_map.clone());
                        if let Some(target_comp) = target.composition.get(&target_key) {
                            if target_comp != fg_mapped { return false; }
                        }
                    }
                }
            }
        }
        true
    }

    /// Apply functor to an object.
    pub fn apply_to_object(&self, obj: &str) -> Option<&ObjId> {
        self.obj_map.get(obj)
    }

    /// Apply functor to a morphism.
    pub fn apply_to_morphism(&self, morph: &str) -> Option<&MorphId> {
        self.morph_map.get(morph)
    }
}

/// A natural transformation between two functors.
#[derive(Clone, Debug)]
pub struct NaturalTransformation {
    pub name: String,
    pub source_functor: String,
    pub target_functor: String,
    /// Component for each object: α_A : F(A) → G(A).
    pub components: HashMap<ObjId, MorphId>,
}

impl NaturalTransformation {
    /// Create a new natural transformation.
    pub fn new(name: impl Into<String>, source: impl Into<String>, target: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            source_functor: source.into(),
            target_functor: target.into(),
            components: HashMap::new(),
        }
    }

    /// Set the component for an object.
    pub fn set_component(&mut self, obj: impl Into<String>, morphism: impl Into<String>) -> &mut Self {
        self.components.insert(obj.into(), morphism.into());
        self
    }

    /// Verify naturality: for every f: A → B, α_B ∘ F(f) = G(f) ∘ α_A.
    pub fn verify_naturality(
        &self,
        f: &Functor,
        g: &Functor,
        cat_source: &Category,
        cat_target: &Category,
    ) -> bool {
        for morph in &cat_source.morphisms {
            if morph.id.starts_with("id_") { continue; }
            let a = &morph.source;
            let b = &morph.target;

            let alpha_a = match self.components.get(a) {
                Some(c) => c,
                None => continue,
            };
            let alpha_b = match self.components.get(b) {
                Some(c) => c,
                None => continue,
            };

            let f_morph = match f.apply_to_morphism(&morph.id) {
                Some(m) => m,
                None => continue,
            };
            let g_morph = match g.apply_to_morphism(&morph.id) {
                Some(m) => m,
                None => continue,
            };

            // Check: α_B ∘ F(f) and G(f) ∘ α_A should both exist and be equal
            let lhs_key = (alpha_b.clone(), f_morph.clone());
            let rhs_key = (g_morph.clone(), alpha_a.clone());

            if let (Some(lhs), Some(rhs)) =
                (cat_target.composition.get(&lhs_key), cat_target.composition.get(&rhs_key))
            {
                if lhs != rhs { return false; }
            }
        }
        true
    }
}

/// Compute the pullback (fiber product) of two morphisms with a common target.
pub fn pullback(
    cat: &mut Category,
    f: &str,  // morphism A → C
    g: &str,  // morphism B → C
    pullback_obj: impl Into<String>,
) -> Option<(MorphId, MorphId)> {
    let f_morph = cat.find_morphism(f)?.clone();
    let g_morph = cat.find_morphism(g)?.clone();
    assert_eq!(f_morph.target, g_morph.target, "Pullback requires common target");

    let pb_obj = pullback_obj.into();
    cat.add_object(&pb_obj);

    let p1_name = format!("pb_proj1_{}", pb_obj);
    let p2_name = format!("pb_proj2_{}", pb_obj);

    let p1 = Morphism::new(&p1_name, &pb_obj, &f_morph.source);
    let p2 = Morphism::new(&p2_name, &pb_obj, &g_morph.source);

    let p1_id = p1.id.clone();
    let p2_id = p2.id.clone();

    cat.add_morphism(p1);
    cat.add_morphism(p2);

    Some((p1_id, p2_id))
}

/// Compute the pushout of two morphisms with a common source.
pub fn pushout(
    cat: &mut Category,
    f: &str,  // morphism C → A
    g: &str,  // morphism C → B
    pushout_obj: impl Into<String>,
) -> Option<(MorphId, MorphId)> {
    let f_morph = cat.find_morphism(f)?.clone();
    let g_morph = cat.find_morphism(g)?.clone();
    assert_eq!(f_morph.source, g_morph.source, "Pushout requires common source");

    let po_obj = pushout_obj.into();
    cat.add_object(&po_obj);

    let i1_name = format!("po_inj1_{}", po_obj);
    let i2_name = format!("po_inj2_{}", po_obj);

    let i1 = Morphism::new(&i1_name, &f_morph.target, &po_obj);
    let i2 = Morphism::new(&i2_name, &g_morph.target, &po_obj);

    let i1_id = i1.id.clone();
    let i2_id = i2.id.clone();

    cat.add_morphism(i1);
    cat.add_morphism(i2);

    Some((i1_id, i2_id))
}

/// A coordination protocol modeled categorically.
#[derive(Clone, Debug)]
pub struct CoordinationProtocol {
    pub name: String,
    pub category: Category,
    /// Agent states as objects with associated data.
    pub agent_states: HashMap<ObjId, String>,
}

impl CoordinationProtocol {
    /// Create a new coordination protocol.
    pub fn new(name: impl Into<String>) -> Self {
        let name_str = name.into();
        Self {
            name: name_str.clone(),
            category: Category::new(format!("{}_cat", name_str)),
            agent_states: HashMap::new(),
        }
    }

    /// Register an agent.
    pub fn register_agent(&mut self, id: impl Into<String>, state: impl Into<String>) {
        let id = id.into();
        self.category.add_object(&id);
        self.agent_states.insert(id, state.into());
    }

    /// Send a message (creates a morphism from sender to receiver).
    pub fn send_message(&mut self, msg_id: impl Into<String>, from: &str, to: &str) {
        let morph = Morphism::new(msg_id, from, to);
        self.category.add_morphism(morph);
    }

    /// Get the state of an agent.
    pub fn get_state(&self, agent: &str) -> Option<&String> {
        self.agent_states.get(agent)
    }

    /// Update agent state.
    pub fn update_state(&mut self, agent: &str, state: impl Into<String>) {
        if let Some(s) = self.agent_states.get_mut(agent) {
            *s = state.into();
        }
    }

    /// Get all messages sent by an agent.
    pub fn messages_from(&self, agent: &str) -> Vec<&Morphism> {
        self.category.morphisms.iter()
            .filter(|m| m.source == agent && !m.id.starts_with("id_"))
            .collect()
    }

    /// Get all messages received by an agent.
    pub fn messages_to(&self, agent: &str) -> Vec<&Morphism> {
        self.category.morphisms.iter()
            .filter(|m| m.target == agent && !m.id.starts_with("id_"))
            .collect()
    }

    /// Number of registered agents.
    pub fn num_agents(&self) -> usize {
        self.agent_states.len()
    }

    /// Number of messages exchanged.
    pub fn num_messages(&self) -> usize {
        self.category.num_nontrivial_morphisms()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn simple_category() -> Category {
        let mut cat = Category::new("Simple");
        cat.add_object("A");
        cat.add_object("B");
        cat.add_object("C");
        cat.add_morphism(Morphism::new("f", "A", "B"));
        cat.add_morphism(Morphism::new("g", "B", "C"));
        cat.compose("g", "f", "gf");
        cat
    }

    #[test]
    fn test_category_creation() {
        let cat = simple_category();
        assert_eq!(cat.num_objects(), 3);
        assert!(cat.find_morphism("f").is_some());
        assert!(cat.find_morphism("g").is_some());
        assert!(cat.find_morphism("gf").is_some());
    }

    #[test]
    fn test_identity_morphisms() {
        let cat = simple_category();
        let id_a = cat.identity("A").unwrap();
        assert_eq!(id_a.source, "A");
        assert_eq!(id_a.target, "A");
    }

    #[test]
    fn test_hom_set() {
        let cat = simple_category();
        let hom_ab = cat.hom("A", "B");
        assert_eq!(hom_ab.len(), 1);
        assert_eq!(hom_ab[0].id, "f");
    }

    #[test]
    fn test_composition() {
        let cat = simple_category();
        let gf = cat.find_morphism("gf").unwrap();
        assert_eq!(gf.source, "A");
        assert_eq!(gf.target, "C");
    }

    #[test]
    fn test_identity_laws() {
        let mut cat = Category::new("Test");
        cat.add_object("X");
        cat.add_object("Y");
        cat.add_morphism(Morphism::new("h", "X", "Y"));
        // Define h ∘ id_X = h and id_Y ∘ h = h
        cat.compose("h", "id_X", "h");
        cat.compose("id_Y", "h", "h");
        assert!(cat.verify_identity_laws());
    }

    #[test]
    fn test_functor_basic() {
        let mut f = Functor::new("F", "C1", "C2");
        f.map_object("A", "X");
        f.map_object("B", "Y");
        assert_eq!(f.apply_to_object("A"), Some(&"X".to_string()));
        assert_eq!(f.apply_to_object("B"), Some(&"Y".to_string()));
        assert!(f.apply_to_object("Z").is_none());
    }

    #[test]
    fn test_functor_mapping() {
        let mut f = Functor::new("F", "C1", "C2");
        f.map_morphism("f", "f'");
        f.map_morphism("g", "g'");
        assert_eq!(f.apply_to_morphism("f"), Some(&"f'".to_string()));
    }

    #[test]
    fn test_natural_transformation() {
        let mut nt = NaturalTransformation::new("alpha", "F", "G");
        nt.set_component("A", "alpha_A");
        nt.set_component("B", "alpha_B");
        assert_eq!(nt.components.get("A"), Some(&"alpha_A".to_string()));
        assert_eq!(nt.components.get("B"), Some(&"alpha_B".to_string()));
    }

    #[test]
    fn test_pullback() {
        let mut cat = Category::new("PB");
        cat.add_object("A");
        cat.add_object("B");
        cat.add_object("C");
        cat.add_morphism(Morphism::new("f", "A", "C"));
        cat.add_morphism(Morphism::new("g", "B", "C"));

        let result = pullback(&mut cat, "f", "g", "A×_C B");
        assert!(result.is_some());
        let (p1, p2) = result.unwrap();
        assert!(cat.find_morphism(&p1).is_some());
        assert!(cat.find_morphism(&p2).is_some());
        assert_eq!(cat.num_objects(), 4); // A, B, C, A×_C B
    }

    #[test]
    fn test_pushout() {
        let mut cat = Category::new("PO");
        cat.add_object("A");
        cat.add_object("B");
        cat.add_object("C");
        cat.add_morphism(Morphism::new("f", "C", "A"));
        cat.add_morphism(Morphism::new("g", "C", "B"));

        let result = pushout(&mut cat, "f", "g", "A+_C B");
        assert!(result.is_some());
        let (i1, i2) = result.unwrap();
        assert!(cat.find_morphism(&i1).is_some());
        assert!(cat.find_morphism(&i2).is_some());
        assert_eq!(cat.num_objects(), 4);
    }

    #[test]
    fn test_coordination_protocol() {
        let mut proto = CoordinationProtocol::new("test_proto");
        proto.register_agent("agent1", "idle");
        proto.register_agent("agent2", "idle");
        assert_eq!(proto.num_agents(), 2);
        assert_eq!(proto.num_messages(), 0);
    }

    #[test]
    fn test_coordination_messages() {
        let mut proto = CoordinationProtocol::new("msg_test");
        proto.register_agent("alice", "ready");
        proto.register_agent("bob", "ready");
        proto.send_message("hello", "alice", "bob");
        proto.send_message("reply", "bob", "alice");

        assert_eq!(proto.num_messages(), 2);
        assert_eq!(proto.messages_from("alice").len(), 1);
        assert_eq!(proto.messages_to("alice").len(), 1);
        assert_eq!(proto.get_state("alice"), Some(&"ready".to_string()));
    }

    #[test]
    fn test_coordination_state_update() {
        let mut proto = CoordinationProtocol::new("state_test");
        proto.register_agent("agent", "idle");
        proto.update_state("agent", "busy");
        assert_eq!(proto.get_state("agent"), Some(&"busy".to_string()));
    }
}
