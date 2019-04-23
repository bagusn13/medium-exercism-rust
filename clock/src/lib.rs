use std::fmt;

pub struct Clock{
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        //convert ke menit semua
        //untuk yang minus 
        //-45 -> 23:15
        //waktu dalam sehari + (-45)
        //1440 - 45 = 1395
        //23 : 15 
        
        //versi 1 untuk format normal
        //let to_minutes = (hours * 60) + minutes;

        //versi 2 untuk over sehari
        //let to_minutes = ((hours * 60) + minutes) % 1440;
        
        //versi 3 untuk yang minus normal
        //let to_minutes = (((hours * 60) + minutes) + 1440 ) % 1440;

        //versi 4 untuk yang minus over sehari
        //let to_minutes = ((((hours * 60) + minutes) % 1440) + 1440) % 1440;

        let to_minutes = ((((hours * 60) + minutes) % 1440) + 1440) % 1440;
        let hours = to_minutes / 60;
        let minutes = to_minutes % 60;
        Clock {hours,minutes}

    }

    pub fn add_minutes(&mut self, minutes: i32) -> Self {
        //let m = self.minutes + minutes;
        //self.minutes = m;
        Clock::new(self.hours, self.minutes + minutes)
    }

}

impl fmt::Display for Clock{
    fn fmt(& self, f: &mut fmt::Formatter) -> fmt::Result {
        //(1)  write!(f, "0{}:0{}", self.hours,self.minutes)
        //let h = self.minutes / 60;
        //let m = self.minutes % 60;
        write!(
            f,"{}:{}",
            //pengecualian paling pertama klo jam 24 jadi 00
            //hours > 10 langsung
            //hours < 10 tambahin 0 di depan
            //minutes > 10 langsung
            //minutes < 10 tambahin 0 di depan
            (if self.hours < 10 {
                format!("0{}", self.hours)
            }
            else if self.hours == 24 {
                format!("00")
            }
            else{
                format!("{}", self.hours)
            })
            ,
            (if self.minutes < 10{
                format!("0{}", self.minutes)
            }
            else{
                format!("{}", self.minutes)
            })
        )
    }
    
}

impl fmt::Debug for Clock{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"Clock {{ hours: {}, minutes: {} }}",self.hours,self.minutes)

    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Clock) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}
