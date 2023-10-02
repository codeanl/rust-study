fn main() {
    // 3
    let creature = SeaCreature {
        noise: String::from("blub"),
    };
    println!("{}", creature.get_sound());

    // 4
    let creature = SeaCreature {
        name: String::from("Ferris"),
        noise: String::from("blub"),
    };
    println!("{}", creature.get_sound());

    // 5
    let creature = SeaCreature {
        name: String::from("Ferris"),
        noise: String::from("blub"),
    };
    creature.make_noise();

    // 6
    let creature = SeaCreature {
        name: String::from("Ferris"),
        noise: String::from("blub"),
    };
    creature.make_alot_of_noise();

    // 7
    let creature = SeaCreature {
        name: String::from("Ferris"),
        noise: String::from("blub"),
    };
    creature.make_alot_of_noise();

    // 8
    let creature = SeaCreature {
        name: String::from("Ferris"),
        noise: String::from("咕噜"),
    };
    static_make_noise(&creature);
    dynamic_make_noise(&creature);

    // 11
    let creature = SeaCreature {
        name: String::from("Ferris"),
        noise: String::from("咕噜"),
    };
    generic_make_noise(&creature);

    // 12
    let creature = SeaCreature {
        name: String::from("Ferris"),
        noise: String::from("咕噜"),
    };
    generic_make_noise(&creature);

    // 13
    let ferris = SeaCreature {
        name: String::from("Ferris"),
        noise: String::from("咕噜"),
    };
    let sarah = SeaCreature {
        name: String::from("Sarah"),
        noise: String::from("哧溜"),
    };
    let ocean = Ocean {
        animals: vec![Box::new(ferris), Box::new(sarah)],
    };
    for a in ocean.animals.iter() {
        a.make_noise();
    }
}

// 3
struct SeaCreature {
    noise: String,
}
impl SeaCreature {
    fn get_sound(&self) -> &str {
        &self.noise
    }
}

// 4
struct SeaCreature {
    pub name: String,
    noise: String,
}
impl SeaCreature {
    pub fn get_sound(&self) -> &str {
        &self.noise
    }
}

// 5
struct SeaCreature {
    pub name: String,
    noise: String,
}

impl SeaCreature {
    pub fn get_sound(&self) -> &str {
        &self.noise
    }
}

trait NoiseMaker {
    fn make_noise(&self);
}

impl NoiseMaker for SeaCreature {
    fn make_noise(&self) {
        println!("{}", &self.get_sound());
    }
}

// 6
struct SeaCreature {
    pub name: String,
    noise: String,
}

impl SeaCreature {
    pub fn get_sound(&self) -> &str {
        &self.noise
    }
}

trait NoiseMaker {
    fn make_noise(&self);

    fn make_alot_of_noise(&self) {
        self.make_noise();
        self.make_noise();
        self.make_noise();
    }
}

impl NoiseMaker for SeaCreature {
    fn make_noise(&self) {
        println!("{}", &self.get_sound());
    }
}

// 7
struct SeaCreature {
    pub name: String,
    noise: String,
}

impl SeaCreature {
    pub fn get_sound(&self) -> &str {
        &self.noise
    }
}

trait NoiseMaker {
    fn make_noise(&self);
}

trait LoudNoiseMaker: NoiseMaker {
    fn make_alot_of_noise(&self) {
        self.make_noise();
        self.make_noise();
        self.make_noise();
    }
}

impl NoiseMaker for SeaCreature {
    fn make_noise(&self) {
        println!("{}", &self.get_sound());
    }
}

impl LoudNoiseMaker for SeaCreature {}

// 8
struct SeaCreature {
    pub name: String,
    noise: String,
}

impl SeaCreature {
    pub fn get_sound(&self) -> &str {
        &self.noise
    }
}

trait NoiseMaker {
    fn make_noise(&self);
}

impl NoiseMaker for SeaCreature {
    fn make_noise(&self) {
        println!("{}", &self.get_sound());
    }
}

fn static_make_noise(creature: &SeaCreature) {
    // 我们知道真实类型
    creature.make_noise();
}

fn dynamic_make_noise(noise_maker: &dyn NoiseMaker) {
    // 我们不知道真实类型
    noise_maker.make_noise();
}

// 11
struct SeaCreature {
    pub name: String,
    noise: String,
}

impl SeaCreature {
    pub fn get_sound(&self) -> &str {
        &self.noise
    }
}

trait NoiseMaker {
    fn make_noise(&self);
}

impl NoiseMaker for SeaCreature {
    fn make_noise(&self) {
        println!("{}", &self.get_sound());
    }
}

fn generic_make_noise<T>(creature: &T)
where
    T: NoiseMaker,
{
    // 我们在编译期就已经知道其真实类型
    creature.make_noise();
}

// 12
struct SeaCreature {
    pub name: String,
    noise: String,
}

impl SeaCreature {
    pub fn get_sound(&self) -> &str {
        &self.noise
    }
}

trait NoiseMaker {
    fn make_noise(&self);
}

impl NoiseMaker for SeaCreature {
    fn make_noise(&self) {
        println!("{}", &self.get_sound());
    }
}

fn generic_make_noise(creature: &impl NoiseMaker) {
    // 我们在编译期就已经知道其真实类型
    creature.make_noise();
}

// 13
struct SeaCreature {
    pub name: String,
    noise: String,
}

impl SeaCreature {
    pub fn get_sound(&self) -> &str {
        &self.noise
    }
}

trait NoiseMaker {
    fn make_noise(&self);
}

impl NoiseMaker for SeaCreature {
    fn make_noise(&self) {
        println!("{}", &self.get_sound());
    }
}

struct Ocean {
    animals: Vec<Box<dyn NoiseMaker>>,
}
