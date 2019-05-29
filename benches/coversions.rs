
extern crate mrusty;
use mrusty::{Mruby, MrubyImpl};

#[macro_use]
extern crate bencher;
use bencher::Bencher;


fn convert_fixnum(b: &mut Bencher) {
    let mruby = Mruby::new();

    b.iter(|| {
        let one = mruby.fixnum(1);

        one.to_i32().unwrap()
    });
}

fn convert_string(b: &mut Bencher) {
    let mruby = Mruby::new();

    b.iter(|| {
        let string = mruby.string("hi");

        string.to_str().unwrap()
    });
}

fn convert_obj(b: &mut Bencher) {
    struct Cont;

    let mruby = Mruby::new();

    mruby.def_class_for::<Cont>("Container");

    b.iter(|| {
        let obj = mruby.obj(Cont);

        obj.to_obj::<Cont>().unwrap()
    });
}

benchmark_group!(benches, convert_obj, convert_string, convert_fixnum);
benchmark_main!(benches);