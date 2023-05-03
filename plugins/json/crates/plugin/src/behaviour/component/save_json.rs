use std::fs::File;

use log::trace;
use serde_json::Value;

use crate::model::*;
use crate::model_file::FileProperties::FILENAME;
use crate::model_json::SaveJsonProperties::PAYLOAD;
use crate::model_runtime::ActionProperties::TRIGGER;
use crate::reactive::*;

entity_behaviour!(SaveJson, SaveJsonFactory, SaveJsonFsm, SaveJsonBehaviourTransitions, SaveJsonValidator);

behaviour_validator!(SaveJsonValidator, ReactiveEntityInstance, TRIGGER.as_ref(), FILENAME.as_ref(), PAYLOAD.as_ref());

impl BehaviourInit<ReactiveEntityInstance> for SaveJsonBehaviourTransitions {}

impl BehaviourConnect<ReactiveEntityInstance> for SaveJsonBehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let reactive_instance = self.reactive_instance.clone();
        self.property_observers.observe_with_handle(TRIGGER.as_ref(), move |trigger: &Value| {
            if !trigger.as_bool().unwrap_or(false) {
                return;
            }
            if let Some(filename) = reactive_instance.get(FILENAME).and_then(|v| v.as_str().map(String::from)) {
                match File::open(&filename) {
                    Ok(file) => {
                        if let Some(value) = reactive_instance.get(PAYLOAD) {
                            if let Ok(_) = serde_json::to_writer_pretty(file, &value) {
                                trace!("Wrote payload to existing file {filename}");
                            }
                        }
                    }
                    Err(_) => match File::create(&filename) {
                        Ok(file) => {
                            if let Some(value) = reactive_instance.get(PAYLOAD) {
                                if let Ok(_) = serde_json::to_writer_pretty(file, &value) {
                                    trace!("Wrote payload to new file {filename}");
                                }
                            }
                        }
                        Err(_) => {}
                    },
                }
            }
        });
        Ok(())
    }
}

impl BehaviourShutdown<ReactiveEntityInstance> for SaveJsonBehaviourTransitions {}
impl BehaviourTransitions<ReactiveEntityInstance> for SaveJsonBehaviourTransitions {}
