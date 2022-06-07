const PROD_BASE_URL: &str = "https://bikeapi.jackdevries.com";
const DEV_BASE_URL: &str = "http://localhost:8000";

/**
 * Determined by whether the `IS_PRODUCTION` environment variable was set
 * during build time.
 */
pub const IS_PRODUCTION: bool = match option_env!("IS_PRODUCTION")  {
    Some(_) => true,
    None => false
};

/**
 * For example, localhost:8000 in development
 */
pub const BACKEND_BASE_URL: &str = {
    if IS_PRODUCTION {
        PROD_BASE_URL
    } else {
        DEV_BASE_URL
    }
};


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_production() {
        let compile_time_var = match option_env!("IS_PRODUCTION")  {
            Some(_) => true,
            None => false
        };
        assert_eq!(compile_time_var, IS_PRODUCTION)
    }

    #[test]
    fn backend_base_url() {
        if IS_PRODUCTION {
            assert_eq!(BACKEND_BASE_URL, "https://bikeapi.jackdevries.com");
        } else {
            assert_eq!(BACKEND_BASE_URL, "http://localhost:8000");
        }
    }
}
