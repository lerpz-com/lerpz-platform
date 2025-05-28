use leptos::{
    logging::debug_warn,
    prelude::{ReadSignal, WriteSignal, provide_context, signal, use_context},
};

#[derive(Clone)]
pub struct AzureAuth(pub ReadSignal<Option<String>>, pub WriteSignal<Option<String>>);

pub fn use_azure_auth() -> AzureAuth {
    match use_context::<AzureAuth>() {
        None => {
            debug_warn!(
                "ThemeContext was not found. Did you forget to add the
                `<ThemeProvider />` component?"
            );
            let (token, set_token) = signal(Some("token".to_string()));
            let azure_auth = AzureAuth(token, set_token);
            provide_context(azure_auth.clone());
            azure_auth
        }
        Some(theme) => theme,
    }
}
