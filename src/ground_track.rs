use sky_track::{Satellite,SubPoint};
pub struct GroundTrack<'a>{
    satellite:&'a Satellite,
    start_time:i64,
    points:Vec<SubPoint>,
    counter:usize,
}
impl GroundTrack<'_>{
    pub fn new(sat:&Satellite,start_time:i64,duration:i64)->GroundTrack{
        let mut carry:Vec<SubPoint> = vec![];
        for i in 0..duration{
            carry.push(sat.get_sub_point((start_time+i).try_into().expect("Timestamp lead to integer overflow")));
        }
        GroundTrack { satellite:sat, start_time: start_time, points: carry,counter:0}
    }
    pub fn get_point(&self, point:usize)->Option<SubPoint>{
        self.points.get(point).copied()
    }
    pub fn get_start_time(&self)->i64{
        self.start_time
    }
    pub fn get_satellite(&self)->String{
        self.satellite.get_name()
    }
}
impl Iterator for GroundTrack<'_>{
    type Item = SubPoint;

    fn next(&mut self) -> Option<Self::Item> {
        let output = self.points.get(self.counter)?;
        self.counter+=1;
        Some(output.clone())
    }
}
