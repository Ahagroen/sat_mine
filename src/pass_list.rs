use sky_track::{Satellite,GroundStation};
struct UpcomingPasses<'b>{
    satellite:&'b Satellite,
    start_time:i64,
    passes:Vec<SatPass<'b>>,
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
        UpcomingPasses { satellite: sat, start_time, passes }
    }
    pub fn get_next_pass(&self,timestamp:i64)->SatPass{
        todo!()
    }
}

#[derive(Clone)]
struct SatPass<'c>{
    ground_station:&'c GroundStation,
    aos_time:i64,
    los_time:i64,
    max_elevation:f64
    // look_angles:Vec<SatAngle>//needed? Since its only going to exist as part of PassesList so maybe this can just exist to mark AOS/LOS times
}
impl SatPass<'_>{
    ///Returns the next pass after predicted_time. 
    fn new<'c>(sat:&Satellite,station:&'c GroundStation, predicted_time:i64,max_search:i64)->Option<SatPass<'c>>{
        let pass_found:bool;
        let aos_time:i64;
        let found_pass:bool;
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
        }
        let mut max_elevation:f64 = 0.;
        let los_time:i64;
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
        Some(SatPass { ground_station: station, aos_time, los_time, max_elevation })
    }
}