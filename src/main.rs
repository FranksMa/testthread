struct Duck;
struct Pig;

// 特征
trait Fly {
    // 具备该特征的行为
    fn fly(&self) -> bool;
    fn is_wings(&self);
}

impl Fly for Duck {
    fn fly(&self) -> bool {
        return true;
    }

    fn is_wings(&self) {
        println!("I have a wing");
    }
}

impl Fly for Pig {
    fn fly(&self) -> bool {
        return false;
    }

    fn is_wings(&self) {
        println!("I don't have a wing");
    }
}

// 静态分发
fn fly_static<T: Fly>(s: T) -> bool {
    s.fly()
}

// 动态分发
fn fly_dyn(s: &dyn Fly) -> bool {
    s.fly()
}

fn main() {
    let pig = Pig;
    pig.is_wings();
    assert_eq!(fly_static::<Pig>(pig), false);

    let duck = Duck;
    duck.is_wings();
    assert_eq!(fly_static::<Duck>(duck), true);
    assert_eq!(fly_dyn(&Pig), false);
    assert_eq!(fly_dyn(&Duck), true);
}
