#[derive(Copy)]
pub struct Koa {
    pub hunger_level: uint,
}



impl Koa {
    fn demand(&self) -> &str {
        // let demand = "feed me!";
        // demand
        // return demand;
        "feed me!"
    }
    fn get_robot_reply(robot_reply: &str) -> &str {
        robot_reply
    }

    fn reply_to_robot(&self, robot_reply: &str) -> &str {
        match robot_reply {
            "yes" => "It's about time!",
            "no"  => "Hiss!",
            _     => "I want food NOW, you useless robot!"
        }
    }
}

pub struct Robot {
    total_food_capacity: uint,
    dispense_amount: uint,
    current_food_level: uint,
    name: String,
}

impl Robot {
    fn dispense(&mut self) -> Result<(), &str> {
        // Dispense food if Koa demands and current food level > 0 && > = dispense amount
        // () = Ok()
        if self.current_food_level >= self.dispense_amount {
            self.current_food_level = self.current_food_level - self.dispense_amount;
            // self.current_food_level -= self.dispense_amount;
            return Ok(());
        }
        return Err("Sorry, no food left!");
    }

    fn reply_to_koa(&self, koa_demand: &str) -> &str {
        // When Koa says "feed me!" Robot will reply yes or no. Yes if robot can dispense. No if not.

    }
}

#[allow(dead_code)]
fn main() {
    // Add code here
}

#[cfg(test)]
mod test {
    use super::Koa;
    use super::Robot;
  #[test]
    fn test_demand() {
        let koa: Koa = Koa{hunger_level: 10};
        // koa.hunger_level => 10
        // koa.demand => error no attribute named demand
        // koa.demand() => calls the fn demand
        assert_eq!("feed me!", koa.demand());
    }

    #[test]
    fn test_koa_reply_robot_yes(){
        let koa = Koa{hunger_level: 10};
        let robot_reply = Koa::get_robot_reply("yes");
        let reply_to_robot = koa.reply_to_robot(robot_reply);
        assert_eq!("It's about time!", reply_to_robot);
        // reply_to_robot("yes")
        // reply_to_robot("no")

    }

    #[test]
    fn test_koa_reply_robot_no(){
        let koa = Koa{hunger_level: 10};
        // let robot_reply = Koa::get_robot_reply("no");
        // let reply_to_robot = koa.reply_to_robot(robot_reply);
        assert_eq!("Hiss!", koa.reply_to_robot("no"));
    }

    #[test]
    fn test_robot_dispense_ok(){
        let mut robot: Robot = Robot{total_food_capacity: 100u, dispense_amount: 10u, current_food_level: 100u, name: "Butler".to_string()};
        //"string" is &str so you have to convert it to string by using .to_string() function.
        assert_eq!(Ok(()), robot.dispense());
        // assert_eq!((), robot.dispense().unwrap())
    }

    #[test]
    fn test_robot_dispense_err(){
        let mut robot: Robot = Robot{total_food_capacity: 100u, dispense_amount: 10u, current_food_level: 0u, name: "Butler".to_string()};
        assert_eq!(Err("Sorry, no food left!"), robot.dispense())
    }

}
