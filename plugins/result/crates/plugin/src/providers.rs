use inexor_rgf_plugin_api::prelude::providers::*;

#[derive(TypeProvider, Component)]
#[type_provider(tys = "Components", path = "types/components")]
pub struct ResultComponentsProvider {}
