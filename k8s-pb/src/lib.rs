pub mod api;
pub mod apiextensions_apiserver;
pub mod apimachinery;
pub mod kube_aggregator;
pub mod metrics;

pub trait Resource {
    const API_VERSION: &'static str;
    const GROUP: &'static str;
    const VERSION: &'static str;
    const KIND: &'static str;
    const NAME: &'static str;
}
pub trait HasMetadata {
    type Metadata;
    fn metadata(&self) -> Option<&Self::Metadata>;
    fn metadata_mut(&mut self) -> Option<&mut Self::Metadata>;
}
pub trait HasSpec {
    type Spec;
    fn spec(&self) -> Option<&Self::Spec>;
    fn spec_mut(&mut self) -> Option<&mut Self::Spec>;
}
pub trait HasStatus {
    type Status;
    fn status(&self) -> Option<&Self::Status>;
    fn status_mut(&mut self) -> Option<&mut Self::Status>;
}
pub trait HasConditions {
    type Condition;
    fn conditions(&self) -> Option<&[Self::Condition]>;
    fn conditions_mut(&mut self) -> Option<&mut Vec<Self::Condition>>;
}

