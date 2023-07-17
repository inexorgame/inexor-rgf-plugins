pub use component::config_file::*;
pub use entity::config_file::*;

pub mod component;
pub mod entity;

use inexor_rgf_core_model as model;
use inexor_rgf_model_file as model_file;
use inexor_rgf_model_result as model_result;
use inexor_rgf_model_runtime as model_runtime;

pub const NAMESPACE_CONFIG: &str = "config";
