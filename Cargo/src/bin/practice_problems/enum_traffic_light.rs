use crate::TrafficLight::GREEN;

enum TrafficLight {
    RED(u32),
    YELLOW(u32),
    GREEN(u32),
}

impl TrafficLight {
    fn driver_action(&self) -> &str {
        match self {
            TrafficLight::GREEN(_) => "GO!",
            TrafficLight::YELLOW(_) => "SLOW DOWN!",
            TrafficLight::RED(_) => "STOP!",
        }
    }

    fn wait_time(&self) -> u32 {
        match self {
            TrafficLight::RED(secs) => *secs,
            TrafficLight::YELLOW(secs) => *secs,
            TrafficLight::GREEN(secs) => *secs,
        }
    }
}

fn main() {
    let lights = [
        TrafficLight::GREEN(30),
        TrafficLight::YELLOW(5),
        TrafficLight::RED(20),
    ];

    for light in lights {
        let action = light.driver_action();
        let time = light.wait_time();

        println!("The action is: {}, and the time to wait is: {}", action, time);
    }
}

// This one was lowkey kinda hard for me