use crate::utils::types_enums::EventType;
use crate::world::monde::Monde;

#[derive(Clone)]
pub struct EventData {
    pub description: String,
    pub _target_id: Option<usize>,
}

pub struct Evenement {
    pub _id: usize,
    pub trigger_time: u32,
    pub event_type: EventType,
    pub data: Option<EventData>,
}

impl Evenement {
    pub fn new(
        id: usize,
        trigger_time: u32,
        event_type: EventType,
        data: Option<EventData>,
    ) -> Self {
        Self {
            _id: id,
            trigger_time,
            event_type,
            data,
        }
    }

    pub fn execute(&self, _monde: &mut Monde) {
        match self.event_type {
            EventType::RandomEncounter => {
                if let Some(info) = &self.data {
                    let _desc = &info.description;
                }
            }
            EventType::PoliceRaid => {
                if let Some(info) = &self.data {
                    let _desc = &info.description;
                }
            }
            EventType::ScheduledMeeting => {
                if let Some(info) = &self.data {
                    let _desc = &info.description;
                }
            }
            EventType::Other => {
                if let Some(info) = &self.data {
                    let _desc = &info.description;
                }
            }
        }
    }
}
