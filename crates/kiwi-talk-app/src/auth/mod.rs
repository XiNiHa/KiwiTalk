pub mod constants;

use anyhow::Context;
use talk_api_client::{
    auth::{
        resources::LoginData, AccountLoginForm, AuthClientConfig, AuthDeviceConfig, LoginMethod,
        TalkAuthClient,
    },
    response::TalkStatusResponse,
};
use tauri::{
    generate_handler,
    plugin::{Builder, TauriPlugin},
    Runtime, State,
};

use crate::{app::constants::TALK_VERSION, system::SystemInfo, result::TauriResult};

use self::constants::{TALK_AGENT, XVC_HASHER};

pub fn init_plugin<R: Runtime>(name: &'static str) -> TauriPlugin<R> {
    Builder::new(name)
        .invoke_handler(generate_handler![login, register_device, request_passcode])
        .build()
}

#[tauri::command(async)]
async fn login(
    email: String,
    password: String,
    forced: bool,
    app_info: State<'_, SystemInfo>,
) -> TauriResult<TalkStatusResponse<LoginData>> {
    let client = TalkAuthClient::new(create_config(&app_info), XVC_HASHER);

    let res = client
        .login(
            LoginMethod::Account(AccountLoginForm {
                email: &email,
                password: &password,
            }),
            forced,
        )
        .await.context("login request failed")?;

    Ok(res)
}

#[tauri::command(async)]
async fn request_passcode(
    email: String,
    password: String,
    app_info: State<'_, SystemInfo>,
) -> TauriResult<TalkStatusResponse<()>> {
    let client = TalkAuthClient::new(create_config(&app_info), XVC_HASHER);

    let res = client
        .request_passcode(AccountLoginForm {
            email: &email,
            password: &password,
        })
        .await.context("request_passcode request failed")?;

    Ok(res)
}

#[tauri::command(async)]
async fn register_device(
    passcode: String,
    email: String,
    password: String,
    permanent: bool,
    app_info: State<'_, SystemInfo>,
) -> TauriResult<TalkStatusResponse<()>> {
    let client = TalkAuthClient::new(create_config(&app_info), XVC_HASHER);

    let res = client
        .register_device(
            &passcode,
            AccountLoginForm {
                email: &email,
                password: &password,
            },
            permanent,
        )
        .await.context("register_device request failed")?;

    Ok(res)
}

fn create_config<'a>(info: &'a State<'_, SystemInfo>) -> AuthClientConfig<'a> {
    AuthClientConfig {
        device: AuthDeviceConfig {
            name: &info.device_info.name,
            model: None,
            uuid: info.device_info.device_uuid.as_str(),
        },
        language: info.device_info.language(),
        version: TALK_VERSION,
        agent: TALK_AGENT,
    }
}
