/*
Умная розетка умеет:
    Предоставлять текстовое описание.
    Включаться и выключаться.
    Предоставлять данные о текущей потребляемой мощности.
Термометр умеет:
    Выдавать данные о текущей температуре.
*/

struct SmartSocket {
    description: String,
    power_consumption: String,
    status: String,
}
struct SmartThermometer {
    current_temp: String,
} 

impl SmartSocket{
    fn description(&self) {
        println!("SmartSocket description {}", self.description)
    }
    fn power_consumption(&self) {
        println!("SmartSocket power consumption is {}", self.power_consumption)
    }
    fn status(&self) {
        println!("SmartSocket power is {}", self.status)
    }
}

impl SmartThermometer {
    fn current_temp(&self) {
        println!("Current temperature is {}", self.current_temp)
    }
}


fn main() {
    let socket = SmartSocket { description: String::from(""), power_consumption: String::from("1W"), status: String::from("on")};
    let thermometer = SmartThermometer { current_temp: String::from("24 celcius")};

    socket.description();
    socket.power_consumption();
    socket.status();

    thermometer.current_temp();
}
