use serde_json::Value;

pub use function::ConnectorFunction;
pub use function::CONNECTOR_BEHAVIOURS;

use crate::behaviour::relation::connector::validator::ConnectorValidator;
use crate::model::PropertyInstanceGetter;
use crate::model::PropertyInstanceSetter;
use crate::model::ReactiveRelationInstance;
use crate::model_connector::ConnectorProperties::INBOUND_PROPERTY_NAME;
use crate::model_connector::ConnectorProperties::OUTBOUND_PROPERTY_NAME;
use crate::reactive::relation_behaviour;
use crate::reactive::BehaviourConnect;
use crate::reactive::BehaviourConnectFailed;
use crate::reactive::BehaviourDisconnect;
use crate::reactive::BehaviourFsm;
use crate::reactive::BehaviourInit;
use crate::reactive::BehaviourInitializationFailed;
use crate::reactive::BehaviourShutdown;
use crate::reactive::BehaviourTransitions;
use crate::reactive::PropertyObserverContainer;

pub mod function;
pub mod validator;

relation_behaviour!(
    Connector,
    ConnectorFactory,
    ConnectorFsm,
    ConnectorBehaviourTransitions,
    ConnectorValidator,
    f,
    ConnectorFunction
);

impl ConnectorBehaviourTransitions {
    fn get_outbound_property_name(&self) -> Option<String> {
        self.reactive_instance.as_string(OUTBOUND_PROPERTY_NAME.as_ref())
    }

    fn get_inbound_property_name(&self) -> Option<String> {
        self.reactive_instance.as_string(INBOUND_PROPERTY_NAME.as_ref())
    }
}

impl BehaviourInit<ReactiveRelationInstance> for ConnectorBehaviourTransitions {
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        let outbound_property_name = self.get_outbound_property_name().ok_or(BehaviourInitializationFailed {})?;
        let inbound_property_name = self.get_inbound_property_name().ok_or(BehaviourInitializationFailed {})?;
        let value = self
            .reactive_instance
            .outbound
            .get(outbound_property_name)
            .ok_or(BehaviourInitializationFailed {})?;
        let f = self.f;
        self.reactive_instance.inbound.set(inbound_property_name.clone(), f(&value));
        Ok(())
    }
}

impl BehaviourShutdown<ReactiveRelationInstance> for ConnectorBehaviourTransitions {}

impl BehaviourConnect<ReactiveRelationInstance> for ConnectorBehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let outbound_property_name = self.get_outbound_property_name().ok_or(BehaviourConnectFailed {})?;
        let inbound_property_name = self.get_inbound_property_name().ok_or(BehaviourConnectFailed {})?;
        let f = self.f;
        let inbound = self.reactive_instance.inbound.clone();
        self.outbound_property_observers
            .observe_with_handle(&outbound_property_name, move |value: &Value| {
                inbound.set(inbound_property_name.clone(), f(value));
            });
        Ok(())
    }
}

impl BehaviourTransitions<ReactiveRelationInstance> for ConnectorBehaviourTransitions {}
