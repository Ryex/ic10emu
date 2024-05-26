use macro_rules_attribute::derive;
use stationeers_data::enums::basic::Class as SlotClass;

use crate::vm::{
    object::{
        macros::ObjectInterface,
        traits::{Human, HumanRef, HumanRefMut, Object, Storage, StorageRef, StorageRefMut},
        Name, ObjectID, Slot,
    },
    VM,
};

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

    pub hydration: f32,
    pub nutrition: f32,
    pub oxygenation: f32,
    pub food_quality: f32,
    pub mood: f32,
    pub hygine: f32,

    left_hand_slot: Slot,
    right_hand_slot: Slot,
    suit_slot: Slot,
    helmet_slot: Slot,
    glasses_slot: Slot,
    backpack_slot: Slot,
    uniform_slot: Slot,
    toolbelt_slot: Slot,
}

impl HumanPlayer {
    pub fn new(id: ObjectID, vm: std::rc::Rc<VM>) -> Self {
        HumanPlayer {
            id,
            prefab: Name::new(""),
            name: Name::new(""),
            vm,
            hydration: 5.0,
            nutrition: 5.0,
            oxygenation: 1.0,
            food_quality: 0.75,
            mood: 1.0,
            hygine: 1.0,
            left_hand_slot: Slot::new(id, 0, "LeftHand".to_string(), SlotClass::None),
            right_hand_slot: Slot::new(id, 1, "RightHand".to_string(), SlotClass::None),
            suit_slot: Slot::new(id, 2, "Helmet".to_string(), SlotClass::Suit),
            helmet_slot: Slot::new(id, 3, "LeftHand".to_string(), SlotClass::Helmet),
            glasses_slot: Slot::new(id, 4, "LeftHand".to_string(), SlotClass::Glasses),
            backpack_slot: Slot::new(id, 5, "LeftHand".to_string(), SlotClass::Back),
            uniform_slot: Slot::new(id, 6, "LeftHand".to_string(), SlotClass::Uniform),
            toolbelt_slot: Slot::new(id, 7, "LeftHand".to_string(), SlotClass::Belt),
        }
    }
}

impl Storage for HumanPlayer {
    fn get_slots(&self) -> Vec<&Slot> {
        vec![
            &self.left_hand_slot,
            &self.right_hand_slot,
            &self.suit_slot,
            &self.helmet_slot,
            &self.glasses_slot,
            &self.backpack_slot,
            &self.uniform_slot,
            &self.toolbelt_slot,
        ]
    }

    fn get_slots_mut(&mut self) -> Vec<&mut Slot> {
        vec![
            &mut self.left_hand_slot,
            &mut self.right_hand_slot,
            &mut self.suit_slot,
            &mut self.helmet_slot,
            &mut self.glasses_slot,
            &mut self.backpack_slot,
            &mut self.uniform_slot,
            &mut self.toolbelt_slot,
        ]
    }

    fn slots_count(&self) -> usize {
        8
    }

    fn get_slot(&self, index: usize) -> Option<&Slot> {
        match index {
            0 => Some(&self.left_hand_slot),
            1 => Some(&self.right_hand_slot),
            2 => Some(&self.suit_slot),
            3 => Some(&self.helmet_slot),
            4 => Some(&self.glasses_slot),
            5 => Some(&self.backpack_slot),
            6 => Some(&self.uniform_slot),
            7 => Some(&self.toolbelt_slot),
            _ => None,
        }
    }
    fn get_slot_mut(&mut self, index: usize) -> Option<&mut Slot> {
        match index {
            0 => Some(&mut self.left_hand_slot),
            1 => Some(&mut self.right_hand_slot),
            2 => Some(&mut self.suit_slot),
            3 => Some(&mut self.helmet_slot),
            4 => Some(&mut self.glasses_slot),
            5 => Some(&mut self.backpack_slot),
            6 => Some(&mut self.uniform_slot),
            7 => Some(&mut self.toolbelt_slot),
            _ => None,
        }
    }
}

impl Human for HumanPlayer {
    fn get_hydration(&self) -> f32 {
        self.hydration
    }
    fn set_hydration(&mut self, hydration: f32) {
        self.hydration = hydration;
    }
    fn get_nutrition(&self) -> f32 {
        self.nutrition
    }
    fn set_nutrition(&mut self, nutrition: f32) {
        self.nutrition = nutrition;
    }
    fn get_oxygenation(&self) -> f32 {
        self.oxygenation
    }
    fn set_oxygenation(&mut self, oxygenation: f32) {
        self.oxygenation = oxygenation;
    }
    fn get_food_quality(&self) -> f32 {
        self.food_quality
    }
    fn set_food_quality(&mut self, quality: f32) {
        self.food_quality = quality;
    }
    fn get_mood(&self) -> f32 {
        self.mood
    }
    fn set_mood(&mut self, mood: f32) {
        self.mood = mood;
    }
    fn get_hygine(&self) -> f32 {
        self.hygine
    }
    fn set_hygine(&mut self, hygine: f32) {
        self.hygine = hygine;
    }
    fn is_artificial(&self) -> bool {
        false
    }
    fn robot_battery(&self) -> Option<super::VMObject> {
        None
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
