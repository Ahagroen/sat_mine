use core::panic;

use sky_track::Satellite;
use reqwest::Error;
pub enum TleSource{
    Celestrak
    //Future sources can be implemented here (including credentials if needed)
}
pub async fn create_satellite_from_name(source:TleSource,name:Option<String>,sat_id:Option<String>)->Result<Satellite,Error>{
    let tle_string:String;
    match source{
        TleSource::Celestrak => {tle_string = call_celestrak(name,sat_id).await?},
    }
    Ok(Satellite::new_from_tle(&tle_string))//Currently only returns the first satellite found, not sure how to handle this down the line
}

async fn call_celestrak(name:Option<String>,id:Option<String>)->Result<String,Error>{
    match id{
        Some(sat_id) => Ok(reqwest::get(format!("ttps://celestrak.org/NORAD/elements/gp.php?CATNR={}&FORMAT=TLE",sat_id))
        .await?
        .text()
        .await?),
        None => match name{
            Some(sat_name) => Ok(reqwest::get(format!("ttps://celestrak.org/NORAD/elements/gp.php?NAME={}&FORMAT=TLE",sat_name))
            .await?
            .text()
            .await?),
            None => panic!("Didn't provide a name or an ID"),//Convert to error later
        },
    }
}
