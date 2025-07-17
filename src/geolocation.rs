use serde::Deserialize;
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;

#[derive(Deserialize)]
pub struct IpGeolocation {
    pub latitude: f64,
    pub longitude: f64,
    pub city: String,
    pub region: String,
    pub country: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct NominatimResult {
    pub display_name: String,
    pub lat: String,
    pub lon: String,
    pub address: Option<NominatimAddress>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct NominatimAddress {
    pub city: Option<String>,
    pub town: Option<String>,
    pub village: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
}

#[derive(Clone, Debug)]
pub struct CitySearchResult {
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub display_name: String,
    pub score: i64,
}

pub async fn get_current_location() -> Result<IpGeolocation, reqwest::Error> {
    let url = "https://ipapi.co/json/";
    
    let response = reqwest::Client::new()
        .get(url)
        .header(reqwest::header::USER_AGENT, "sh.tangled.one.weird.nandi.weather-applet")
        .send()
        .await?;
    
    let location = response.json::<IpGeolocation>().await?;
    Ok(location)
}

pub async fn search_cities(query: String) -> Result<Vec<CitySearchResult>, reqwest::Error> {
    if query.trim().is_empty() {
        return Ok(Vec::new());
    }

    let url = format!(
        "https://nominatim.openstreetmap.org/search?q={}&format=json&limit=10&addressdetails=1",
        urlencoding::encode(&query)
    );
    
    let response = reqwest::Client::new()
        .get(&url)
        .header(reqwest::header::USER_AGENT, "sh.tangled.one.weird.nandi.weather-applet")
        .send()
        .await?;
    
    let results: Vec<NominatimResult> = response.json().await.unwrap_or_default();
    
    let matcher = SkimMatcherV2::default();
    let mut scored_results: Vec<CitySearchResult> = results
        .into_iter()
        .filter_map(|result| {
            let lat = result.lat.parse::<f64>().ok()?;
            let lon = result.lon.parse::<f64>().ok()?;
            
            let search_text = result.display_name.clone();
            
            let city_name = result.address.as_ref().and_then(|addr| {
                addr.city.as_ref()
                    .or(addr.town.as_ref())
                    .or(addr.village.as_ref())
            }).map(|s| s.as_str()).unwrap_or_else(|| {
                search_text.split(',').next().unwrap_or("Unknown")
            });
            
            matcher.fuzzy_match(&search_text, &query).map(|score| CitySearchResult {
                name: city_name.to_string(),
                latitude: lat,
                longitude: lon,
                display_name: result.display_name,
                score,
            })
        })
        .collect();
    
    scored_results.sort_by(|a, b| b.score.cmp(&a.score));
    scored_results.truncate(5);
    
    Ok(scored_results)
}