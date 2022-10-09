
fn main(){

    let spec_thing = MoreSpecificThing{
        base: BaseThing { data: 10 },
        extra_data: 69
    };

    let s = does_something_with_base(&spec_thing);
    println!("{:#?}", s);

    let f = does_something_with_boxed_base(Box::new(spec_thing));
    println!("{:#?}", f);

    let game_specific_thing = GameSpecificThing{
        base: MoreSpecificThing { 
            base: BaseThing{
                data: 1
            }, 
            extra_data: 2 
        },
        game_data: 3
    };

    let g = does_something_with_more_specific(&game_specific_thing);
    println!("{}", g);
}

struct BaseThing{
    pub data: usize
}

trait Base{
    fn base(&self) -> &BaseThing;
}

struct MoreSpecificThing{
    pub base: BaseThing,
    pub extra_data: usize
}

impl Base for MoreSpecificThing{
    fn base(&self) -> &BaseThing{
        &self.base
    }
}

impl Base for BaseThing{
    fn base(&self) -> &BaseThing{
        &self
    }
}

trait MoreSpecific{
    fn base(&self) -> &MoreSpecificThing;
}

impl MoreSpecific for MoreSpecificThing{
    fn base(&self) -> &MoreSpecificThing{
        &self
    }
}

struct GameSpecificThing{
    base: MoreSpecificThing,
    game_data: usize
}

impl MoreSpecific for GameSpecificThing{
    fn base(&self) -> &MoreSpecificThing{
        &self.base
    }
}

fn does_something_with_base(x: &dyn Base) -> usize{
    x.base().data * 2
}

fn does_something_with_boxed_base(x: Box<dyn Base>) -> usize{
    x.base().data * 4
}

fn does_something_with_more_specific(x: &dyn MoreSpecific) -> usize{
    x.base().extra_data * 69
}