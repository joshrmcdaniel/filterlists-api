#[cfg(all(feature = "reqwasm", feature = "reqwest"))]
compile_error!("feature \"reqwasm\" and \"reqwest\" cannot be enabled at the same time");

mod dtypes;
mod get;
pub use self::dtypes::*;
use crate::get::_get;

pub const FILTERLISTS_API_URL: &str = "https://filterlists.com/api/directory/";

pub async fn get_filters() -> Result<Vec<Filter>, FilterListError> {
    _get::<Vec<Filter>>(&format!("{FILTERLISTS_API_URL}/lists")).await
}

pub async fn get_filter_information(filter: FilterArgs) -> Result<FilterDetails, FilterListError> {
    let id = match filter {
        FilterArgs::U64(id) => id,
        FilterArgs::Filter(filter) => filter.id.clone(),
    };
    _get::<FilterDetails>(&format!("{FILTERLISTS_API_URL}/lists/{id}")).await
}

pub async fn get_syntaxes() -> Result<Vec<Filter>, FilterListError> {
    _get::<Vec<Filter>>(&format!("{FILTERLISTS_API_URL}/syntaxes")).await
}

pub async fn get_licenses() -> Result<Vec<FilterLicense>, FilterListError> {
    _get::<Vec<FilterLicense>>(&format!("{FILTERLISTS_API_URL}/licenses")).await
}

pub async fn get_software_list() -> Result<Vec<FilterSoftware>, FilterListError> {
    _get::<Vec<FilterSoftware>>(&format!("{FILTERLISTS_API_URL}/software")).await
}

pub async fn get_languages() -> Result<Vec<FilterLanguage>, FilterListError> {
    _get::<Vec<FilterLanguage>>(&format!("{FILTERLISTS_API_URL}/languages")).await
}

pub async fn get_tags() -> Result<Vec<FilterTag>, FilterListError> {
    _get::<Vec<FilterTag>>(&format!("{FILTERLISTS_API_URL}/tags")).await
}

pub async fn get_maintainers() -> Result<Vec<FilterMaintainer>, FilterListError> {
    _get::<Vec<FilterMaintainer>>(&format!("{FILTERLISTS_API_URL}/maintainers")).await
}
