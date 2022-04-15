pub enum TrafficLight{
    Red,
    Yellow,
    Green,
}


pub trait Time{
    fn time(&self) -> u8;
}

impl Time for TrafficLight{
    fn time(&self) -> u8{
        match self {
            TrafficLight::Red => 80,
            TrafficLight::Yellow => 2,
            TrafficLight::Green => 20,
        }
    }
}