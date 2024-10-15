mod ground_track;
mod pass_list;

pub use crate::pass_list::UpcomingPasses;
pub use crate::ground_track::GroundTrack;

pub use sky_track;
use sky_track::Satellite;

enum TleSource{
    Celestrak
    //Future sources can be implemented here (including credentials if needed)
}


pub fn create_satellite_from_name(name:String,source:TleSource)->Satellite{
    todo!()
}

fn call_celestrak()->String{
    todo!()
}