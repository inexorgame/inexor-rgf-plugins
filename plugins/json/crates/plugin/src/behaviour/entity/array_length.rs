use inexor_rgf_behaviour::entity_behaviour;
use inexor_rgf_behaviour::PropertyObserverContainer;
use inexor_rgf_behaviour_api::behaviour_validator;
use inexor_rgf_behaviour_api::prelude::*;
use inexor_rgf_graph::prelude::*;
use inexor_rgf_reactive::ReactiveEntity;
use serde_json::json;
use serde_json::Value;
use uuid::Uuid;

use inexor_rgf_model_json::ArrayLengthProperties::ARRAY;
use inexor_rgf_model_json::ArrayLengthProperties::LENGTH;

entity_behaviour!(ArrayLength, ArrayLengthFactory, ArrayLengthFsm, ArrayLengthBehaviourTransitions, ArrayLengthValidator);

behaviour_validator!(ArrayLengthValidator, Uuid, ReactiveEntity, ARRAY.as_ref(), LENGTH.as_ref());

impl BehaviourInit<Uuid, ReactiveEntity> for ArrayLengthBehaviourTransitions {
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        if let Some(array) = self.reactive_instance.as_array(ARRAY) {
            self.reactive_instance.set(LENGTH, json!(array.len()));
        }
        Ok(())
    }
}

impl BehaviourConnect<Uuid, ReactiveEntity> for ArrayLengthBehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let reactive_instance = self.reactive_instance.clone();
        self.property_observers.observe_with_handle(ARRAY.as_ref(), move |array: &Value| {
            if let Some(array) = array.as_array() {
                reactive_instance.set(LENGTH, json!(array.len()));
            }
        });
        Ok(())
    }
}

impl BehaviourShutdown<Uuid, ReactiveEntity> for ArrayLengthBehaviourTransitions {}
impl BehaviourTransitions<Uuid, ReactiveEntity> for ArrayLengthBehaviourTransitions {}
