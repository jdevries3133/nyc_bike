/*****************************************************************************
 * generic constants
 */

pub const IS_PRODUCTION: bool = match option_env!("IS_PRODUCTION") {
    Some(_) => true,
    None => false,
};
const PROTOCOL: &str = if IS_PRODUCTION { "https://" } else { "http://" };

/*****************************************************************************
 * frontend domain and URLs
 */

const WEB_FRONT_PROD_DOMAIN: &str = "nyc-bike.jackdevries.com";
const WEB_FRONT_DEV_DOMAIN: &str = "localhost:8080";

pub const WEB_FRONT_ORIGIN: &str = if IS_PRODUCTION {
    WEB_FRONT_PROD_DOMAIN
} else {
    WEB_FRONT_DEV_DOMAIN
};

pub fn get_web_front_base_url() -> String {
    format!("{}{}", PROTOCOL, WEB_FRONT_ORIGIN)
}

/*****************************************************************************
 * backend domain and URLs
 */

const BACKEND_PROD_DOMAIN: &str = "nyc-bike-back.jackdevries.com";
const BACKEND_DEV_DOMAIN: &str = "localhost:8000";

pub const BACKEND_ORIGIN: &str = if IS_PRODUCTION {
    BACKEND_PROD_DOMAIN
} else {
    BACKEND_DEV_DOMAIN
};

pub fn get_backend_base_url() -> String {
    format!("{}{}", PROTOCOL, BACKEND_ORIGIN)
}
