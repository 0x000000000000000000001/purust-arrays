pub fn Data_Array_NonEmpty_Internal_foldl1Impl() -> crate::UnknownType {
    crate::Value::Func1(purust_core::Func1::Shared(std::rc::Rc::new(move |mut f: crate::UnknownType| -> crate::UnknownType {
        crate::Value::Func1(purust_core::Func1::Shared(std::rc::Rc::new(move |mut xs: crate::UnknownType| -> crate::UnknownType {
                    let arr = xs.unwrap_array();
                    let mut acc = arr[0].clone();
                    for i in 1..arr.len() {
                        acc = f.unwrap_func1()(acc).unwrap_func1()(arr[i].clone());
                    }
                    acc
                })))
    })))
}

pub fn Data_Array_NonEmpty_Internal_foldr1Impl() -> crate::UnknownType {
    crate::Value::Func1(purust_core::Func1::Shared(std::rc::Rc::new(move |mut f: crate::UnknownType| -> crate::UnknownType {
        crate::Value::Func1(purust_core::Func1::Shared(std::rc::Rc::new(move |mut xs: crate::UnknownType| -> crate::UnknownType {
                    let arr = xs.unwrap_array();
                    let mut acc = arr[arr.len() - 1].clone();
                    for i in (0..arr.len() - 1).rev() {
                        acc = f.unwrap_func1()(arr[i].clone()).unwrap_func1()(acc);
                    }
                    acc
                })))
    })))
}

fn nonempty_traversal_singleton(value: crate::UnknownType) -> crate::UnknownType {
    crate::mk_array(vec![value])
}

fn nonempty_traversal_concat(left: crate::UnknownType) -> crate::UnknownType {
    let left = left.unwrap_array();
    crate::Value::Func1(purust_core::Func1::Shared(std::rc::Rc::new(move |right: crate::UnknownType| {
        let right = right.unwrap_array();
        let mut result = Vec::with_capacity(left.len() + right.len());
        result.extend(left.iter().cloned());
        result.extend(right.iter().cloned());
        crate::mk_array(result)
    })))
}

fn nonempty_traversal_combine(
    apply: &purust_core::Func2<crate::UnknownType, crate::UnknownType, crate::UnknownType>,
    map: &purust_core::Func2<crate::UnknownType, crate::UnknownType, crate::UnknownType>,
    effects: &[crate::UnknownType],
) -> crate::UnknownType {
    if effects.len() == 1 {
        return map(
            crate::Value::Func1(purust_core::Func1::Static(nonempty_traversal_singleton)),
            effects[0].clone(),
        );
    }
    // A balanced applicative expression keeps construction, execution and drop
    // depth logarithmic while applying the left effects before the right ones.
    let pivot = effects.len() / 2;
    let left = nonempty_traversal_combine(apply, map, &effects[..pivot]);
    let left = map(
        crate::Value::Func1(purust_core::Func1::Static(nonempty_traversal_concat)),
        left,
    );
    let right = nonempty_traversal_combine(apply, map, &effects[pivot..]);
    apply(left, right)
}

pub fn Data_Array_NonEmpty_Internal_traverse1Impl() -> crate::UnknownType {
    crate::Value::Func3(purust_core::Func3::Static(|apply, map, function| {
        let apply = apply.unwrap_func2();
        let map = map.unwrap_func2();
        let function = function.unwrap_func1();
        crate::Value::Func1(purust_core::Func1::Shared(std::rc::Rc::new(move |array: crate::UnknownType| {
            let array = array.unwrap_array();
            assert!(!array.is_empty(), "traverse1Impl requires a nonempty array");
            // Upstream constructs the element actions from right to left.
            let mut effects: Vec<_> = array.iter().rev().map(|value| function(value.clone())).collect();
            effects.reverse();
            nonempty_traversal_combine(&apply, &map, &effects)
        })))
    }))
}
