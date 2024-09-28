use super::{change_filename::change_filename, download::SpeedUnit};

pub fn rate_limit(arg : &str)->Option<(usize, SpeedUnit)>{
    let mut res: (usize, SpeedUnit) = (0, SpeedUnit::K);
    let limit:String = match change_filename(arg.to_owned()){
        Some(value) => value,
        None => " ".to_owned()
    };
    
    for v in limit.chars(){
        if v.is_digit(10) {
            res.0 = (res.0* 10) + v.to_digit(10).unwrap() as usize
        }else{
            match v{
                'B' | 'b' => {res.1 = SpeedUnit::B},
                'K' | 'k' => {res.1 = SpeedUnit::K},
                'M' | 'm' => {res.1 = SpeedUnit::M},
                'G' | 'g' => {res.1 = SpeedUnit::G},
                _ => {return None}
            }
        }
    }
    Some(res)
}
