use purust_core::{Func2, Value};
use std::rc::Rc;
use Purs_Data_Ordering::Ordering;

pub struct UndefinedOr(Option<Value>);

pub fn Test_Data_UndefinedOr_undefined() -> Rc<UndefinedOr> {
    Rc::new(UndefinedOr(None))
}
pub fn Test_Data_UndefinedOr_defined(value: Value) -> Rc<UndefinedOr> {
    Rc::new(UndefinedOr(Some(value)))
}
pub fn Test_Data_UndefinedOr_eqUndefinedOrImpl(
    eq: Func2<Value, Value, bool>,
    a: Rc<UndefinedOr>,
    b: Rc<UndefinedOr>,
) -> bool {
    match (&a.0, &b.0) {
        (None, None) => true,
        (Some(a), Some(b)) => eq(a.clone(), b.clone()),
        _ => false,
    }
}
pub fn Test_Data_UndefinedOr_compareUndefinedOrImpl(
    lt: Ordering,
    eq: Ordering,
    gt: Ordering,
    compare: Func2<Value, Value, Ordering>,
    a: Rc<UndefinedOr>,
    b: Rc<UndefinedOr>,
) -> Ordering {
    match (&a.0, &b.0) {
        (None, None) => eq,
        (None, _) => lt,
        (_, None) => gt,
        (Some(a), Some(b)) => compare(a.clone(), b.clone()),
    }
}
