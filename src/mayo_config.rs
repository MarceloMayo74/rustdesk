use base::config::keys;
use hbb_common::config;

const SERVER: &str = "mayosystems.duckdns.org";
const SERVER_KEY: &str = "MdTAPgs5slv9DjXoTW5NT0fGwCTM7zeVd4maT5DUYBk=";

// Fixes the server and key, and turns the client into incoming-only. It goes
// through the same maps a signed custom client config fills, so the UI and the
// backend already honor them.
pub fn apply() {
    *config::APP_NAME.write().unwrap() = "MayoSystemsDesk".to_owned();
    {
        let mut hard = config::HARD_SETTINGS.write().unwrap();
        hard.insert("conn-type".to_owned(), "incoming".to_owned());
        hard.insert("disable-ab".to_owned(), "Y".to_owned());
        hard.insert("disable-account".to_owned(), "Y".to_owned());
    }
    {
        let mut overwrite = config::OVERWRITE_SETTINGS.write().unwrap();
        overwrite.insert(
            keys::OPTION_CUSTOM_RENDEZVOUS_SERVER.to_owned(),
            SERVER.to_owned(),
        );
        overwrite.insert(keys::OPTION_KEY.to_owned(), SERVER_KEY.to_owned());
        overwrite.insert(keys::OPTION_ALLOW_AUTO_UPDATE.to_owned(), "N".to_owned());
    }
    config::OVERWRITE_LOCAL_SETTINGS
        .write()
        .unwrap()
        .insert(keys::OPTION_ENABLE_CHECK_UPDATE.to_owned(), "N".to_owned());
    let mut builtin = config::BUILTIN_SETTINGS.write().unwrap();
    for k in [
        keys::OPTION_HIDE_SERVER_SETTINGS,
        keys::OPTION_HIDE_NETWORK_SETTINGS,
        keys::OPTION_HIDE_PROXY_SETTINGS,
        keys::OPTION_HIDE_WEBSOCKET_SETTINGS,
        keys::OPTION_DISABLE_CHANGE_ID,
    ] {
        builtin.insert(k.to_owned(), "Y".to_owned());
    }
}
