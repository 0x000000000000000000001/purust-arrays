use purust_core::*;
use std::cell::RefCell;
use std::rc::Rc;

fn integers(value: Value) -> Vec<i64> {
    value.unwrap_array().iter().map(Value::unwrap_int).collect()
}

#[test]
fn callbacks_capture_values_and_short_circuit_left_to_right() {
    use Purs_Data_Array::*;
    let input = mk_array(vec![mk_int(1), mk_int(2), mk_int(3)]);
    let seen = Rc::new(RefCell::new(Vec::new()));
    let trace = seen.clone();
    let predicate = Value::Func1(Func1::Shared(Rc::new(move |value| {
        let n = value.unwrap_int();
        trace.borrow_mut().push(n);
        mk_bool(n == 2)
    })));
    assert!(Data_Array_anyImpl().unwrap_func2()(predicate.clone(), input.clone()).unwrap_bool());
    assert_eq!(*seen.borrow(), vec![1, 2]);
    seen.borrow_mut().clear();
    assert_eq!(
        integers(Data_Array_filterImpl().unwrap_func2()(
            predicate,
            input.clone()
        )),
        vec![2]
    );
    assert_eq!(*seen.borrow(), vec![1, 2, 3]);
    assert_eq!(integers(input), vec![1, 2, 3]);
}

#[test]
fn sorting_is_stable_and_checked_indexing_preserves_input() {
    use Purs_Data_Array::*;
    let input = mk_array(vec![mk_int(21), mk_int(11), mk_int(22), mk_int(12)]);
    let compare = Value::Func2(Func2::Static(|a, b| {
        mk_int(match (a.unwrap_int() / 10).cmp(&(b.unwrap_int() / 10)) {
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => 1,
        })
    }));
    let identity = Value::Func1(Func1::Static(|value| value));
    let sorted = Data_Array_sortByImpl().unwrap_func3()(compare, identity.clone(), input.clone());
    assert_eq!(integers(sorted), vec![11, 12, 21, 22]);
    assert_eq!(integers(input.clone()), vec![21, 11, 22, 12]);
    for index in [-1, 4, i64::MAX] {
        assert!(matches!(
            Data_Array_indexImpl().unwrap_func4()(
                identity.clone(),
                Value::Unit,
                input.clone(),
                mk_int(index)
            ),
            Value::Unit
        ));
    }
    assert_eq!(
        Data_Array_indexImpl().unwrap_func4()(identity, Value::Unit, input, mk_int(2)).unwrap_int(),
        22
    );
}

#[test]
fn st_allocations_snapshots_replay_and_bounds_are_independent() {
    use Purs_Data_Array_ST::*;
    let allocate = Data_Array_ST_new();
    let first = allocate.unwrap_func1()(Value::Unit);
    let second = allocate.unwrap_func1()(Value::Unit);
    let push = Data_Array_ST_pushImpl().unwrap_func2();
    assert_eq!(push(mk_int(1), first.clone()).unwrap_int(), 1);
    let frozen = Data_Array_ST_freezeImpl().unwrap_func1()(first.clone());
    let cloned = Data_Array_ST_cloneImpl().unwrap_func1()(first.clone());
    assert_eq!(push(mk_int(2), first.clone()).unwrap_int(), 2);
    assert_eq!(push(mk_int(2), first.clone()).unwrap_int(), 3);
    assert!(integers(Data_Array_ST_freezeImpl().unwrap_func1()(second)).is_empty());
    assert_eq!(integers(frozen), vec![1]);
    assert_eq!(
        integers(Data_Array_ST_freezeImpl().unwrap_func1()(cloned)),
        vec![1]
    );
    let poke = Data_Array_ST_pokeImpl().unwrap_func3();
    for index in [-1, 3, i64::MAX] {
        assert!(!poke(mk_int(index), mk_int(99), first.clone()).unwrap_bool());
    }
    assert_eq!(
        integers(Data_Array_ST_freezeImpl().unwrap_func1()(first)),
        vec![1, 2, 2]
    );
}
