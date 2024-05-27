use std::collections::BTreeMap;

use macro_rules_attribute::derive;
use stationeers_data::{
    enums::{basic::Class as SlotClass, Species},
    templates::SlotInfo,
};
#[cfg(feature = "tsify")]
use tsify::Tsify;
#[cfg(feature = "tsify")]
use wasm_bindgen::prelude::*;

use crate::vm::{
    object::{
        macros::ObjectInterface,
        traits::{
            Human, HumanRef, HumanRefMut, Object, StatState, Storage, StorageRef, StorageRefMut,
            Thermal,
        },
        Name, ObjectID, Slot, SlotOccupantInfo,
    },
    VM,
};

static MAX_NUTRITION: f32 = 50.0;
// static FULL_NUTRITION: f32 = 45.0;
static WARNING_NUTRITION: f32 = 15.0;
static CRITICAL_NUTRITION: f32 = 5.0;

static MAX_HYDRATION: f32 = 8.75;
static WARNING_HYDRATION: f32 = 2.0;
static CRITICAL_HYDRATION: f32 = 1.0;

static MAX_OXYGENATION: f32 = 0.024;

static MAX_FOOD_QUALITY: f32 = 1.0;

static MAX_MOOD: f32 = 1.0;
static WARNING_MOOD: f32 = 0.5;
static CRITICAL_MOOD: f32 = 0.0;

static MAX_HYGIENE: f32 = 1.25;
static WARNING_HYGIENE: f32 = 0.25;
static CRITICAL_HYGIENE: f32 = 0.0;

use serde_derive::{Deserialize, Serialize};

#[derive(ObjectInterface!)]
#[custom(implements(Object {
    Human, Storage
}))]
pub struct HumanPlayer {
    #[custom(object_id)]
    pub id: ObjectID,
    #[custom(object_prefab)]
    pub prefab: Name,
    #[custom(object_name)]
    pub name: Name,
    #[custom(object_vm_ref)]
    pub vm: std::rc::Rc<VM>,

    pub species: Species,

    pub damage: f32,
    pub hydration: f32,
    pub nutrition: f32,
    pub oxygenation: f32,
    pub food_quality: f32,
    pub mood: f32,
    pub hygiene: f32,

    left_hand_slot: Slot,
    right_hand_slot: Slot,
    suit_slot: Slot,
    helmet_slot: Slot,
    glasses_slot: Slot,
    backpack_slot: Slot,
    uniform_slot: Slot,
    toolbelt_slot: Slot,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(feature = "tsify", derive(Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
pub struct EntityInfo {
    pub hydration: f32,
    pub nutrition: f32,
    pub oxygenation: f32,
    pub food_quality: f32,
    pub mood: f32,
    pub hygiene: f32,
}

impl HumanPlayer {
    pub fn new(id: ObjectID, vm: std::rc::Rc<VM>) -> Self {
        HumanPlayer {
            id,
            prefab: Name::new(""),
            name: Name::new(""),
            vm,
            species: Species::Human,
            damage: 0.0,
            hydration: 5.0,
            nutrition: 50.0,
            oxygenation: 0.024,
            food_quality: 0.75,
            mood: 1.0,
            hygiene: 1.0,
            left_hand_slot: Slot::new(id, 0, "LeftHand".to_string(), SlotClass::None),
            right_hand_slot: Slot::new(id, 1, "RightHand".to_string(), SlotClass::None),
            suit_slot: Slot::new(id, 2, "Suit".to_string(), SlotClass::Suit),
            helmet_slot: Slot::new(id, 3, "Helmet".to_string(), SlotClass::Helmet),
            glasses_slot: Slot::new(id, 4, "Glasses".to_string(), SlotClass::Glasses),
            backpack_slot: Slot::new(id, 5, "Back".to_string(), SlotClass::Back),
            uniform_slot: Slot::new(id, 6, "Uniform".to_string(), SlotClass::Uniform),
            toolbelt_slot: Slot::new(id, 7, "Belt".to_string(), SlotClass::Belt),
        }
    }
    pub fn with_species(id: ObjectID, vm: std::rc::Rc<VM>, species: Species) -> Self {
        let uniform_slot = if species == Species::Robot {
            Slot::new(id, 6, "Battery".to_string(), SlotClass::Battery)
        } else {
            Slot::new(id, 6, "Uniform".to_string(), SlotClass::Uniform)
        };
        HumanPlayer {
            id,
            prefab: Name::new(""),
            name: Name::new(""),
            vm,
            species,
            damage: 0.0,
            hydration: 5.0,
            nutrition: 50.0,
            oxygenation: 0.024,
            food_quality: 0.75,
            mood: 1.0,
            hygiene: 1.0,
            left_hand_slot: Slot::new(id, 0, "LeftHand".to_string(), SlotClass::None),
            right_hand_slot: Slot::new(id, 1, "RightHand".to_string(), SlotClass::None),
            suit_slot: Slot::new(id, 2, "Suit".to_string(), SlotClass::Suit),
            helmet_slot: Slot::new(id, 3, "Helmet".to_string(), SlotClass::Helmet),
            glasses_slot: Slot::new(id, 4, "Glasses".to_string(), SlotClass::Glasses),
            backpack_slot: Slot::new(id, 5, "Back".to_string(), SlotClass::Back),
            uniform_slot,
            toolbelt_slot: Slot::new(id, 7, "Belt".to_string(), SlotClass::Belt),
        }
    }

    pub fn update_entity_info(&mut self, info: &EntityInfo) {
        self.hydration = info.hydration;
        self.nutrition = info.nutrition;
        self.oxygenation = info.oxygenation;
        self.food_quality = info.food_quality;
        self.mood = info.mood;
        self.hygiene = info.hygiene;
    }

    pub fn update_slots_from_info(&mut self, info: &BTreeMap<u32, SlotOccupantInfo>) {
        for (index, slot_info) in info {
            match index {
                0 => {
                    self.left_hand_slot.occupant.replace(slot_info.clone());
                }
                1 => {
                    self.right_hand_slot.occupant.replace(slot_info.clone());
                }
                2 => {
                    self.helmet_slot.occupant.replace(slot_info.clone());
                }
                3 => {
                    self.suit_slot.occupant.replace(slot_info.clone());
                }
                4 => {
                    self.backpack_slot.occupant.replace(slot_info.clone());
                }
                5 => {
                    self.uniform_slot.occupant.replace(slot_info.clone());
                }
                6 => {
                    self.toolbelt_slot.occupant.replace(slot_info.clone());
                }
                7 => {
                    self.glasses_slot.occupant.replace(slot_info.clone());
                }
                _ => {}
            }
        }
    }
}

impl Thermal for HumanPlayer {
    fn get_radiation_factor(&self) -> f32 {
        0.1
    }
    fn get_convection_factor(&self) -> f32 {
        0.1
    }
}

impl Storage for HumanPlayer {
    fn get_slots(&self) -> Vec<&Slot> {
        vec![
            &self.left_hand_slot,
            &self.right_hand_slot,
            &self.helmet_slot,
            &self.suit_slot,
            &self.backpack_slot,
            &self.uniform_slot,
            &self.toolbelt_slot,
            &self.glasses_slot,
        ]
    }

    fn get_slots_mut(&mut self) -> Vec<&mut Slot> {
        vec![
            &mut self.left_hand_slot,
            &mut self.right_hand_slot,
            &mut self.helmet_slot,
            &mut self.suit_slot,
            &mut self.backpack_slot,
            &mut self.uniform_slot,
            &mut self.toolbelt_slot,
            &mut self.glasses_slot,
        ]
    }

    fn slots_count(&self) -> usize {
        8
    }

    fn get_slot(&self, index: usize) -> Option<&Slot> {
        match index {
            0 => Some(&self.left_hand_slot),
            1 => Some(&self.right_hand_slot),
            2 => Some(&self.helmet_slot),
            3 => Some(&self.suit_slot),
            4 => Some(&self.backpack_slot),
            5 => Some(&self.uniform_slot),
            6 => Some(&self.toolbelt_slot),
            7 => Some(&self.glasses_slot),
            _ => None,
        }
    }
    fn get_slot_mut(&mut self, index: usize) -> Option<&mut Slot> {
        match index {
            0 => Some(&mut self.left_hand_slot),
            1 => Some(&mut self.right_hand_slot),
            2 => Some(&mut self.helmet_slot),
            3 => Some(&mut self.suit_slot),
            4 => Some(&mut self.backpack_slot),
            5 => Some(&mut self.uniform_slot),
            6 => Some(&mut self.toolbelt_slot),
            7 => Some(&mut self.glasses_slot),
            _ => None,
        }
    }
}

impl Human for HumanPlayer {
    fn get_species(&self) -> Species {
        self.species
    }
    fn get_damage(&self) -> f32 {
        self.damage
    }
    fn set_damage(&mut self, damage: f32) {
        self.damage = damage;
    }
    fn get_hydration(&self) -> f32 {
        self.hydration
    }
    fn set_hydration(&mut self, hydration: f32) {
        self.hydration = hydration.clamp(0.0, MAX_HYDRATION);
    }
    fn hydration_state(&self) -> super::traits::StatState {
        if self.hydration < CRITICAL_HYDRATION {
            return StatState::Critical;
        }
        if self.hydration < WARNING_HYDRATION {
            return StatState::Warning;
        }
        StatState::Normal
    }
    fn get_nutrition(&self) -> f32 {
        self.nutrition
    }
    fn set_nutrition(&mut self, nutrition: f32) {
        self.nutrition = nutrition.clamp(0.0, MAX_NUTRITION);
    }
    fn nutrition_state(&self) -> StatState {
        if self.nutrition < CRITICAL_NUTRITION {
            return StatState::Critical;
        }
        if self.nutrition < WARNING_NUTRITION {
            return StatState::Warning;
        }
        StatState::Normal
    }
    fn get_oxygenation(&self) -> f32 {
        self.oxygenation
    }
    fn set_oxygenation(&mut self, oxygenation: f32) {
        self.oxygenation = oxygenation.clamp(0.0, MAX_OXYGENATION);
    }
    fn get_food_quality(&self) -> f32 {
        self.food_quality
    }
    fn set_food_quality(&mut self, quality: f32) {
        self.food_quality = quality.clamp(0.0, MAX_FOOD_QUALITY);
    }
    fn get_mood(&self) -> f32 {
        self.mood
    }
    fn set_mood(&mut self, mood: f32) {
        self.mood = mood.clamp(0.0, MAX_MOOD);
    }
    fn mood_state(&self) -> StatState {
        if self.mood < CRITICAL_MOOD {
            return StatState::Critical;
        }
        if self.mood < WARNING_MOOD {
            return StatState::Warning;
        }
        StatState::Normal
    }
    fn get_hygiene(&self) -> f32 {
        self.hygiene
    }
    fn set_hygiene(&mut self, hygiene: f32) {
        self.hygiene = hygiene.clamp(0.0, MAX_HYGIENE);
    }
    fn hygine_state(&self) -> StatState {
        if self.hygiene < CRITICAL_HYGIENE {
            return StatState::Critical;
        }
        if self.hygiene < WARNING_HYGIENE {
            return StatState::Warning;
        }
        StatState::Normal
    }
    fn is_artificial(&self) -> bool {
        self.species == Species::Robot
    }
    fn robot_battery(&self) -> Option<super::VMObject> {
        if self.species != Species::Robot {
            return None;
        }

        self.uniform_slot()
            .occupant
            .as_ref()
            .and_then(|info| self.vm.get_object(info.id))
    }
    fn suit_slot(&self) -> &Slot {
        &self.suit_slot
    }
    fn suit_slot_mut(&mut self) -> &mut Slot {
        &mut self.suit_slot
    }
    fn helmet_slot(&self) -> &Slot {
        &self.helmet_slot
    }
    fn helmet_slot_mut(&mut self) -> &mut Slot {
        &mut self.helmet_slot
    }
    fn glasses_slot(&self) -> &Slot {
        &self.glasses_slot
    }
    fn glasses_slot_mut(&mut self) -> &mut Slot {
        &mut self.glasses_slot
    }
    fn backpack_slot(&self) -> &Slot {
        &self.backpack_slot
    }
    fn backpack_slot_mut(&mut self) -> &mut Slot {
        &mut self.backpack_slot
    }
    fn uniform_slot(&self) -> &Slot {
        &self.uniform_slot
    }
    fn uniform_slot_mut(&mut self) -> &mut Slot {
        &mut self.uniform_slot
    }
    fn toolbelt_slot(&self) -> &Slot {
        &self.toolbelt_slot
    }
    fn toolbelt_slot_mut(&mut self) -> &mut Slot {
        &mut self.toolbelt_slot
    }
    fn left_hand_slot(&self) -> &Slot {
        &self.left_hand_slot
    }
    fn left_hand_slot_mut(&mut self) -> &mut Slot {
        &mut self.left_hand_slot
    }
    fn right_hand_slot(&self) -> &Slot {
        &self.right_hand_slot
    }
    fn right_hand_slot_mut(&mut self) -> &mut Slot {
        &mut self.right_hand_slot
    }
}
