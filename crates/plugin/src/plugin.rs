use crate::behaviour::entity::random_bool::RandomBoolFactory;
use crate::behaviour::entity::random_f64::RandomF64Factory;
use crate::behaviour::entity::random_f64_pseudo::RandomF64PseudoFactory;
use crate::behaviour::entity::random_f64_range::RandomF64RangeFactory;
use crate::behaviour::entity::random_i64::RandomI64Factory;
use crate::behaviour::entity::random_i64_pseudo::RandomI64PseudoFactory;
use crate::behaviour::entity::random_i64_range::RandomI64RangeFactory;
use crate::behaviour::entity::random_string::RandomStringFactory;
use crate::behaviour::entity::random_u64::RandomU64Factory;
use crate::behaviour::entity::random_u64_pseudo::RandomU64PseudoFactory;
use crate::behaviour::entity::random_u64_range::RandomU64RangeFactory;
use crate::behaviour::entity::random_uuid::RandomUuidFactory;
use inexor_rgf_model_random::BEHAVIOUR_RANDOM_BOOL;
use inexor_rgf_model_random::BEHAVIOUR_RANDOM_F64;
use inexor_rgf_model_random::BEHAVIOUR_RANDOM_F64_PSEUDO;
use inexor_rgf_model_random::BEHAVIOUR_RANDOM_F64_RANGE;
use inexor_rgf_model_random::BEHAVIOUR_RANDOM_I64;
use inexor_rgf_model_random::BEHAVIOUR_RANDOM_I64_PSEUDO;
use inexor_rgf_model_random::BEHAVIOUR_RANDOM_I64_RANGE;
use inexor_rgf_model_random::BEHAVIOUR_RANDOM_STRING;
use inexor_rgf_model_random::BEHAVIOUR_RANDOM_U64;
use inexor_rgf_model_random::BEHAVIOUR_RANDOM_U64_PSEUDO;
use inexor_rgf_model_random::BEHAVIOUR_RANDOM_U64_RANGE;
use inexor_rgf_model_random::BEHAVIOUR_RANDOM_UUID;
use inexor_rgf_model_random::ENTITY_BEHAVIOUR_RANDOM_BOOL;
use inexor_rgf_model_random::ENTITY_BEHAVIOUR_RANDOM_F64;
use inexor_rgf_model_random::ENTITY_BEHAVIOUR_RANDOM_F64_PSEUDO;
use inexor_rgf_model_random::ENTITY_BEHAVIOUR_RANDOM_F64_RANGE;
use inexor_rgf_model_random::ENTITY_BEHAVIOUR_RANDOM_I64;
use inexor_rgf_model_random::ENTITY_BEHAVIOUR_RANDOM_I64_PSEUDO;
use inexor_rgf_model_random::ENTITY_BEHAVIOUR_RANDOM_I64_RANGE;
use inexor_rgf_model_random::ENTITY_BEHAVIOUR_RANDOM_STRING;
use inexor_rgf_model_random::ENTITY_BEHAVIOUR_RANDOM_U64;
use inexor_rgf_model_random::ENTITY_BEHAVIOUR_RANDOM_U64_PSEUDO;
use inexor_rgf_model_random::ENTITY_BEHAVIOUR_RANDOM_U64_RANGE;
use inexor_rgf_model_random::ENTITY_BEHAVIOUR_RANDOM_UUID;
use std::sync::Arc;
use std::sync::RwLock;

use crate::di::*;
use crate::plugins::entity_type_provider;
use crate::plugins::plugin_context::PluginContext;
use crate::plugins::EntityTypeProvider;
use crate::plugins::EntityTypeProviderError;
use crate::plugins::Plugin;
use crate::plugins::PluginActivationError;
use crate::plugins::PluginContextDeinitializationError;
use crate::plugins::PluginContextInitializationError;
use crate::plugins::PluginDeactivationError;
use crate::providers::RandomEntityTypeProviderImpl;

#[wrapper]
pub struct PluginContextContainer(RwLock<Option<Arc<dyn PluginContext>>>);

#[provides]
fn create_empty_plugin_context_container() -> PluginContextContainer {
    PluginContextContainer(RwLock::new(None))
}

pub trait RandomPlugin: Plugin + Send + Sync {}

#[module]
pub struct RandomPluginImpl {
    entity_type_provider: Wrc<RandomEntityTypeProviderImpl>,

    context: PluginContextContainer,
}

impl RandomPluginImpl {}

interfaces!(RandomPluginImpl: dyn Plugin);

#[provides]
impl RandomPlugin for RandomPluginImpl {}

impl Plugin for RandomPluginImpl {
    fn activate(&self) -> Result<(), PluginActivationError> {
        let guard = self.context.0.read().unwrap();
        if let Some(context) = guard.clone() {
            let entity_behaviour_registry = context.get_entity_behaviour_registry();

            // Random Bool
            let factory = Arc::new(RandomBoolFactory::new(BEHAVIOUR_RANDOM_BOOL.clone()));
            entity_behaviour_registry.register(ENTITY_BEHAVIOUR_RANDOM_BOOL.clone(), factory);

            // Random F64 Pseudo
            let factory = Arc::new(RandomF64PseudoFactory::new(BEHAVIOUR_RANDOM_F64_PSEUDO.clone()));
            entity_behaviour_registry.register(ENTITY_BEHAVIOUR_RANDOM_F64_PSEUDO.clone(), factory);

            // Random F64 Range
            let factory = Arc::new(RandomF64RangeFactory::new(BEHAVIOUR_RANDOM_F64_RANGE.clone()));
            entity_behaviour_registry.register(ENTITY_BEHAVIOUR_RANDOM_F64_RANGE.clone(), factory);

            // Random F64
            let factory = Arc::new(RandomF64Factory::new(BEHAVIOUR_RANDOM_F64.clone()));
            entity_behaviour_registry.register(ENTITY_BEHAVIOUR_RANDOM_F64.clone(), factory);

            // Random I64 Pseudo
            let factory = Arc::new(RandomI64PseudoFactory::new(BEHAVIOUR_RANDOM_I64_PSEUDO.clone()));
            entity_behaviour_registry.register(ENTITY_BEHAVIOUR_RANDOM_I64_PSEUDO.clone(), factory);

            // Random I64 Range
            let factory = Arc::new(RandomI64RangeFactory::new(BEHAVIOUR_RANDOM_I64_RANGE.clone()));
            entity_behaviour_registry.register(ENTITY_BEHAVIOUR_RANDOM_I64_RANGE.clone(), factory);

            // Random I64
            let factory = Arc::new(RandomI64Factory::new(BEHAVIOUR_RANDOM_I64.clone()));
            entity_behaviour_registry.register(ENTITY_BEHAVIOUR_RANDOM_I64.clone(), factory);

            // Random String
            let factory = Arc::new(RandomStringFactory::new(BEHAVIOUR_RANDOM_STRING.clone()));
            entity_behaviour_registry.register(ENTITY_BEHAVIOUR_RANDOM_STRING.clone(), factory);

            // Random U64 Pseudo
            let factory = Arc::new(RandomU64PseudoFactory::new(BEHAVIOUR_RANDOM_U64_PSEUDO.clone()));
            entity_behaviour_registry.register(ENTITY_BEHAVIOUR_RANDOM_U64_PSEUDO.clone(), factory);

            // Random U64 Range
            let factory = Arc::new(RandomU64RangeFactory::new(BEHAVIOUR_RANDOM_U64_RANGE.clone()));
            entity_behaviour_registry.register(ENTITY_BEHAVIOUR_RANDOM_U64_RANGE.clone(), factory);

            // Random U64
            let factory = Arc::new(RandomU64Factory::new(BEHAVIOUR_RANDOM_U64.clone()));
            entity_behaviour_registry.register(ENTITY_BEHAVIOUR_RANDOM_U64.clone(), factory);

            // Random UUID
            let factory = Arc::new(RandomUuidFactory::new(BEHAVIOUR_RANDOM_UUID.clone()));
            entity_behaviour_registry.register(ENTITY_BEHAVIOUR_RANDOM_UUID.clone(), factory);
        }
        Ok(())
    }

    fn deactivate(&self) -> Result<(), PluginDeactivationError> {
        let guard = self.context.0.read().unwrap();
        if let Some(context) = guard.clone() {
            let entity_behaviour_registry = context.get_entity_behaviour_registry();
            entity_behaviour_registry.unregister(&ENTITY_BEHAVIOUR_RANDOM_BOOL);
            entity_behaviour_registry.unregister(&ENTITY_BEHAVIOUR_RANDOM_F64_PSEUDO);
            entity_behaviour_registry.unregister(&ENTITY_BEHAVIOUR_RANDOM_F64_RANGE);
            entity_behaviour_registry.unregister(&ENTITY_BEHAVIOUR_RANDOM_F64);
            entity_behaviour_registry.unregister(&ENTITY_BEHAVIOUR_RANDOM_I64_PSEUDO);
            entity_behaviour_registry.unregister(&ENTITY_BEHAVIOUR_RANDOM_I64_RANGE);
            entity_behaviour_registry.unregister(&ENTITY_BEHAVIOUR_RANDOM_I64);
            entity_behaviour_registry.unregister(&ENTITY_BEHAVIOUR_RANDOM_STRING);
            entity_behaviour_registry.unregister(&ENTITY_BEHAVIOUR_RANDOM_U64_PSEUDO);
            entity_behaviour_registry.unregister(&ENTITY_BEHAVIOUR_RANDOM_U64_RANGE);
            entity_behaviour_registry.unregister(&ENTITY_BEHAVIOUR_RANDOM_U64);
            entity_behaviour_registry.unregister(&ENTITY_BEHAVIOUR_RANDOM_UUID);
        }
        Ok(())
    }

    fn set_context(&self, context: Arc<dyn PluginContext>) -> Result<(), PluginContextInitializationError> {
        self.context.0.write().unwrap().replace(context.clone());
        Ok(())
    }

    fn remove_context(&self) -> Result<(), PluginContextDeinitializationError> {
        let mut writer = self.context.0.write().unwrap();
        *writer = None;
        Ok(())
    }

    fn get_entity_type_provider(&self) -> Result<Option<Arc<dyn EntityTypeProvider>>, EntityTypeProviderError> {
        entity_type_provider!(self.entity_type_provider)
    }
}
