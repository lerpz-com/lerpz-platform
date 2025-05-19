use leptos::prelude::{Signal, WriteSignal};

#[derive(Clone)]
pub struct AzureAuthCtx(
    Signal<Option<String>>,
    WriteSignal<Option<String>>
);

pub fn use_azure_auth() {

}
