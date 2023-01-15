use std::fs::File;
use std::path::Path;

use serde_json::Value;

use crate::model::*;
// TODO: import model_logical::ActionProperties instead of model_json::ActionProperties
use crate::model_json::ActionProperties::RESULT;
use crate::model_json::ActionProperties::TRIGGER;
// TODO: import model_file::FileProperties instead of model_json::FileProperties
use crate::model_json::FileProperties::FILENAME;
use crate::reactive::*;

entity_behaviour!(LoadJson, LoadJsonFactory, LoadJsonFsm, LoadJsonBehaviourTransitions, LoadJsonValidator);

behaviour_validator!(LoadJsonValidator, ReactiveEntityInstance, TRIGGER.as_ref(), RESULT.as_ref());

impl BehaviourInit<ReactiveEntityInstance> for LoadJsonBehaviourTransitions {
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        if self.reactive_instance.as_bool(TRIGGER).unwrap_or(false) {
            if let Some(filename) = self.reactive_instance.as_string(FILENAME) {
                if let Some(value) = load_json(filename) {
                    self.reactive_instance.set(RESULT, value);
                }
            }
        }
        Ok(())
    }
}

impl BehaviourConnect<ReactiveEntityInstance> for LoadJsonBehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let reactive_instance = self.reactive_instance.clone();
        self.property_observers.observe_with_handle(TRIGGER.as_ref(), move |trigger: &Value| {
            if !trigger.is_boolean() || !trigger.as_bool().unwrap_or(false) {
                // Invalid type
                return;
            }
            if let Some(filename) = reactive_instance.as_string(FILENAME) {
                if let Some(value) = load_json(filename) {
                    reactive_instance.set(RESULT, value);
                }
            }
        });
        Ok(())
    }
}

impl BehaviourShutdown<ReactiveEntityInstance> for LoadJsonBehaviourTransitions {}
impl BehaviourTransitions<ReactiveEntityInstance> for LoadJsonBehaviourTransitions {}

fn load_json(filename: String) -> Option<Value> {
    match File::open(Path::new(shellexpand::tilde(&filename).as_ref())) {
        Ok(file) => serde_json::from_reader(file).ok(),
        Err(_) => None,
    }
}
