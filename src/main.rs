
fn main(){
    let spec_thing = MoreSpecificThing{
        base: BaseThing { data: 10 },
        extra_data: 69
    };

    let s = does_something_with_base(&spec_thing);

    println!("{:#?}", s);
    let f = does_something_with_boxed_base(Box::new(spec_thing));

    println!("{:#?}", f);
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

fn does_something_with_base(x: &dyn Base) -> usize{
    x.base().data * 2
}

fn does_something_with_boxed_base(x: Box<dyn Base>) -> usize{
    x.base().data * 4
}