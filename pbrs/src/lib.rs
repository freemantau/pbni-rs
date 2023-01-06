#![allow(unused_variables)]
#![allow(dead_code)]

use pbni::{pbx::*, prelude::*, syslib};

struct RustObject {}

#[nonvisualobject(name = "n_cst_test")]
impl RustObject {
    #[constructor]
    fn new(session: Session, object: Object) -> RustObject { RustObject {} }

    #[method]
    fn of_array(&mut self, mut arg: Array, session: Session) -> Result<String> {
        arg.set_item_long(10, 12333223);
        let mut s = String::new();
        for item in arg.iter::<pblong>() {
            s += &format!("item: {:?}\n", item);
        }

        let mut new_arr = session.new_array(ValueType::String)?;
        for i in 1..1024 * 1024 {
            new_arr.set_item_str(i as pblong, "abcd");
        }

        Ok(s)
    }
    #[method]
    fn of_invoke(&mut self, mut obj: Object) -> Result<String> {
        let rv = obj.invoke_method("of_Test", pbx_args!["abcd", 123])?;
        Ok(rv)
    }
}

struct ParentObject {
    foo: Option<PBString>
}

#[nonvisualobject(name = "n_cst_parent")]
impl ParentObject {
    #[constructor]
    fn new_pbobject(session: Session, object: Object) -> ParentObject {
        ParentObject {
            foo: None
        }
    }
    #[method(overload = 1)]
    fn of_test<'a>(&mut self, session: Session, a: &'a PBStr, b: Option<&'a PBStr>) -> Result<&'a PBStr> {
        let invoker = session.begin_invoke_function(("MessageBox", "ISS"))?;
        invoker.arg(0).set_str("title");
        invoker.arg(1).set_str("content");
        invoker.invoke()?;
        Ok(if let Some(b) = b {
            b
        } else {
            a
        })
    }
    #[method(name = "of_hello", overload = 1)]
    fn hello(&self, arg: String, b: Option<String>) -> String { format!("hello {},{:?}", arg, b) }
    #[method]
    fn of_setfoo(&mut self, arg: &PBStr) -> bool {
        self.foo = Some(arg.to_owned());
        true
    }
    #[method(name = "of_trigger")]
    fn trigger(&mut self, arg: &PBStr) -> Result<String> {
        self.ontest(arg)?;
        let object = ParentObject::get_object(self);
        let eid = object.get_event_id(("ontest", "LS"));
        let mid = object.get_method_id("of_test");
        Ok(format!("eid: {:?}, mid: {:?}", eid, mid))
    }
    #[event(name = "ontest")]
    fn ontest(&mut self, arg: &PBStr) -> Result<pblong> {}
}

struct ChildObject {
    parent: ParentObject
}

#[nonvisualobject(name = "n_cst_child", inherit = "parent")]
impl ChildObject {
    #[constructor]
    fn new_pbobject(session: Session, object: Object) -> ChildObject {
        ChildObject {
            parent: ParentObject {
                foo: None
            }
        }
    }
    #[method]
    fn of_hello(&self, arg: String) -> Result<String> { Ok(format!("child hello {}", arg)) }
}

#[global_function(name = "gf_test_inherit")]
fn test_inherit(parent: &ParentObject) -> Result<String> { Ok(format!("parent foo {:?}", parent.foo)) }

#[global_function(name = "gf_bitor")]
fn bit_or(session: Session, a: pblong, b: pblong) -> pblong { a | b }

#[global_function(name = "gf_Test")]
fn global_function_test(
    session: Session,
    a: &PBStr,
    b: NaiveDate,
    c: NaiveTime,
    d: NaiveDateTime,
    e: Decimal,
    f: &[u8]
) -> Result<()> {
    let a = a.to_string_lossy();
    let b = b.to_string();
    let c = c.to_string();
    let d = d.to_string();
    let e = e.to_string();
    let f = String::from_utf8_lossy(f).into_owned();

    let mut obj = session.new_object("n_cst_pbtest")?;
    obj.set_field_str("is_test", "我爱RUST");
    let is_test = obj.get_field_string("is_test");
    let invoker = obj.begin_invoke_method("of_test")?;
    //invoker.arg(0).set_str("call from rust to");
    let mut arg = session.new_array(ValueType::String)?;
    arg.set_item_str(1, "a");
    arg.set_item_str(2, "b");
    invoker.arg(0).set_array(&arg);
    let rv = invoker.invoke()?.get_string();

    Ok(())
}
