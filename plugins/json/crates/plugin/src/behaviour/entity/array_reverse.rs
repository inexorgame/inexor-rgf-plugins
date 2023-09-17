use inexor_rgf_behaviour::entity_behaviour;
use inexor_rgf_behaviour::PropertyObserverContainer;
use inexor_rgf_behaviour_api::behaviour_validator;
use inexor_rgf_behaviour_api::prelude::*;
use inexor_rgf_graph::prelude::*;
use inexor_rgf_reactive::ReactiveEntity;
use serde_json::Value;
use uuid::Uuid;

use inexor_rgf_model_json::ArrayReverseProperties::ARRAY;
use inexor_rgf_model_result::ResultArrayProperties::RESULT;

entity_behaviour!(ArrayReverse, ArrayReverseFactory, ArrayReverseFsm, ArrayReverseBehaviourTransitions, ArrayReverseValidator);

behaviour_validator!(ArrayReverseValidator, Uuid, ReactiveEntity, ARRAY.as_ref(), RESULT.as_ref());

impl BehaviourInit<Uuid, ReactiveEntity> for ArrayReverseBehaviourTransitions {
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        if let Some(array) = self.reactive_instance.as_array(ARRAY) {
            self.reactive_instance.set(RESULT, reverse_array(&array));
        }
        Ok(())
    }
}

impl BehaviourConnect<Uuid, ReactiveEntity> for ArrayReverseBehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let reactive_instance = self.reactive_instance.clone();
        self.property_observers.observe_with_handle(ARRAY.as_ref(), move |array: &Value| {
            if let Some(array) = array.as_array() {
                reactive_instance.set(RESULT, reverse_array(array));
            }
        });
        Ok(())
    }
}

fn reverse_array(array: &Vec<Value>) -> Value {
    Value::Array(array.into_iter().rev().map(|v| v.clone()).collect())
}

impl BehaviourShutdown<Uuid, ReactiveEntity> for ArrayReverseBehaviourTransitions {}
impl BehaviourTransitions<Uuid, ReactiveEntity> for ArrayReverseBehaviourTransitions {}
