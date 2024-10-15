use sky_track::{GroundStation, SatAngle, Satellite};
pub struct UpcomingPasses<'b>{
    satellite:&'b Satellite,
    ground_station:&'b GroundStation,
    start_time:i64,
    passes:Vec<SatPass>,
    duration:i64,
}
impl UpcomingPasses<'_>{
    pub fn new<'b>(sat:&'b Satellite,ground_station:&'b GroundStation, start_time:i64,duration:i64)->UpcomingPasses<'b>{
        let mut passes:Vec<SatPass> = vec![];
        let mut searching:bool = true;
        let mut guess_time = start_time;
        let max_time = start_time + duration;
        while searching{
            let next_pass = SatPass::new(sat, ground_station, guess_time,max_time);
            match next_pass{
                Some(pass) => {
                    guess_time = pass.aos_time;
                    passes.push(pass.clone())
                },
                None => searching = false,
            }
        }
        UpcomingPasses { satellite: sat, ground_station, start_time, passes, duration }
    }
    pub fn update(&mut self,new_start_time:i64,new_duration:i64){
        let mut passes:Vec<SatPass> = vec![];
        let mut searching:bool = true;
        let mut guess_time = new_start_time;
        let max_time = new_start_time + new_duration;
        while searching{
            let next_pass = SatPass::new(self.satellite, self.ground_station, guess_time,max_time);
            match next_pass{
                Some(pass) => {
                    guess_time = pass.aos_time;
                    passes.push(pass.clone())
                },
                None => searching = false,
            }
        }
        self.passes = passes;
        self.start_time = new_start_time;
        self.duration = new_duration;
    }
    pub fn get_pass_list(&self)->Vec<SatPass>{
        self.passes.clone()
    }
}

#[derive(Clone)]
pub struct SatPass{
    pub aos_time:i64,
    pub los_time:i64,
    pub max_elevation:f64
    // look_angles:Vec<SatAngle>//needed? Since its only going to exist as part of PassesList so maybe this can just exist to mark AOS/LOS times
}
impl SatPass{
    ///Returns the next pass after predicted_time. 
    fn new(sat:&Satellite,station:&GroundStation, predicted_time:i64,max_search:i64)->Option<SatPass>{
        let mut aos_time:i64 = 0;
        let mut found_pass:bool = false;
        for i in predicted_time..max_search{
            let look_angle = sat.get_look_angle(station, i);
            if look_angle.elevation > 0.{
                found_pass = true;
                aos_time = i;
                break
            }
        }
        if !found_pass{
            return None
        } else {
            let mut max_elevation:f64 = 0.;
            let mut los_time:i64 = 0;
            for i in aos_time..{
                let look_angle = sat.get_look_angle(station, i);
                if look_angle.elevation > max_elevation{
                    max_elevation = look_angle.elevation;
                }
                if look_angle.elevation < 0.{
                    los_time = i;
                    break
                }
            }
            Some(SatPass { aos_time, los_time, max_elevation })
        }
    }
    pub fn interpolate_pass(&self,sat:&Satellite,gs:&GroundStation,padding_time:i64)->Vec<SatAngle>{//Make the range an argument?
        let mut passes: Vec<SatAngle> = vec![];
        for i in self.aos_time-padding_time..self.los_time+padding_time{
            passes.push(sat.get_look_angle(gs, i));
        }
        passes
    }
}