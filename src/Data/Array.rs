use purust_core::*;
use std::rc::Rc;
use std::sync::Mutex;

pub fn Data_Array_length(xs: UnknownType) -> i64 {
    // A length read must not clone the backing buffer reference.
    xs.array_len() as i64
}

#[inline]
pub fn Data_Array_rangeImpl() -> UnknownType {
    Value::Func2(Func2::Static(|start, end| {
        let start = start.unwrap_int();
        let end = end.unwrap_int();
        // Ranges are monomorphic Int arrays: keep the elements unboxed.
        mk_int_array(if start <= end {
            (start..=end).collect()
        } else {
            (end..=start).rev().collect()
        })
    }))
}

pub fn Data_Array_replicateImpl() -> UnknownType {
    Value::Func2(Func2::Static(|count, value| {
        let count = count.unwrap_int().max(0) as usize;
        // Replicating an Int keeps the elements unboxed.
        match value {
            Value::Int(v) => mk_int_array(vec![v; count]),
            other => mk_array(vec![other; count]),
        }
    }))
}

pub fn Data_Array_fromFoldableImpl() -> UnknownType {
    Value::Func2(Func2::Static(|foldr, xs| {
        // The universally quantified accumulator is an index into this private
        // persistent list arena. Construction and release both use bounded stack.
        let nodes = Rc::new(Mutex::new(Vec::<(Value, i64)>::new()));
        let captured = nodes.clone();
        let cons = Value::Func2(Func2::Shared(Rc::new(move |head, tail| {
            let mut nodes = captured.lock().unwrap();
            let index = nodes.len() as i64;
            nodes.push((head, tail.unwrap_int()));
            Value::Int(index)
        })));
        let mut index = foldr.unwrap_func3()(cons, Value::Int(-1), xs).unwrap_int();
        let nodes = nodes.lock().unwrap();
        let mut result = Vec::new();
        while index >= 0 {
            let (value, next) = &nodes[index as usize];
            result.push(value.clone());
            index = *next;
        }
        mk_array(result)
    }))
}

pub fn Data_Array_unconsImpl() -> UnknownType {
    Value::Func3(Func3::Static(|empty, next, xs| {
        let xs = xs.unwrap_array();
        match xs.split_first() {
            None => empty.unwrap_func1()(Value::Unit),
            Some((head, tail)) => next.unwrap_func2()(head.clone(), mk_array(tail.to_vec())),
        }
    }))
}

pub fn Data_Array_indexImpl() -> UnknownType {
    Value::Func4(Func4::Static(|just, nothing, xs, index| {
        let just = just.unwrap_func1();
        match usize::try_from(index.unwrap_int()).ok().filter(|i| *i < xs.array_len()) {
            Some(i) => just(xs.array_get(i)),
            None => nothing,
        }
    }))
}

pub fn Data_Array_findMapImpl() -> UnknownType {
    Value::Func4(Func4::Static(|nothing, is_just, f, xs| {
        let f = f.unwrap_func1();
        let is_just = is_just.unwrap_func1();
        for value in xs.unwrap_array().iter() {
            let result = f(value.clone());
            if is_just(result.clone()).unwrap_bool() {
                return result;
            }
        }
        nothing
    }))
}

pub fn Data_Array_findIndexImpl() -> UnknownType {
    Value::Func4(Func4::Static(|just, nothing, f, xs| {
        let f = f.unwrap_func1();
        let just = just.unwrap_func1();
        for (i, value) in xs.unwrap_array().iter().enumerate() {
            if f(value.clone()).unwrap_bool() {
                return just(Value::Int(i as i64));
            }
        }
        nothing
    }))
}

pub fn Data_Array_findLastIndexImpl() -> UnknownType {
    Value::Func4(Func4::Static(|just, nothing, f, xs| {
        let f = f.unwrap_func1();
        let just = just.unwrap_func1();
        for (i, value) in xs.unwrap_array().iter().enumerate().rev() {
            if f(value.clone()).unwrap_bool() {
                return just(Value::Int(i as i64));
            }
        }
        nothing
    }))
}

pub fn Data_Array__insertAt() -> UnknownType {
    Value::Func5(Func5::Static(|just, nothing, index, value, xs| {
        let index = index.unwrap_int();
        let xs = xs.unwrap_array();
        if index < 0 || index as usize > xs.len() {
            return nothing;
        }
        let mut result = xs.as_ref().clone();
        result.insert(index as usize, value);
        just.unwrap_func1()(mk_array(result))
    }))
}

pub fn Data_Array__deleteAt() -> UnknownType {
    Value::Func4(Func4::Static(|just, nothing, index, xs| {
        let index = index.unwrap_int();
        let xs = xs.unwrap_array();
        if index < 0 || index as usize >= xs.len() {
            return nothing;
        }
        let mut result = xs.as_ref().clone();
        result.remove(index as usize);
        just.unwrap_func1()(mk_array(result))
    }))
}

pub fn Data_Array__updateAt() -> UnknownType {
    Value::Func5(Func5::Static(|just, nothing, index, value, xs| {
        let index = index.unwrap_int();
        let xs = xs.unwrap_array();
        if index < 0 || index as usize >= xs.len() {
            return nothing;
        }
        let mut result = xs.as_ref().clone();
        result[index as usize] = value;
        just.unwrap_func1()(mk_array(result))
    }))
}

pub fn Data_Array_reverse(xs: UnknownType) -> UnknownType {
    match purust_core::IntItems::from(&xs) {
        purust_core::IntItems::Ints(values) => mk_int_array(values.iter().rev().copied().collect()),
        purust_core::IntItems::Boxed(values) => mk_array(values.iter().rev().cloned().collect()),
    }
}

pub fn Data_Array_concat(arrays: UnknownType) -> UnknownType {
    let mut result = Vec::new();
    for xs in arrays.unwrap_array().iter() {
        result.extend(xs.unwrap_array().iter().cloned());
    }
    mk_array(result)
}

// Call sites pass callbacks whose concrete representation is often known
// where the call is emitted; cross-crate inlining exposes it to the optimizer.
#[inline]
pub fn Data_Array_filterImpl() -> UnknownType {
    Value::Func2(Func2::Static(|f, xs| {
        // Resolve the callback once: the Value-level wrapper is rebuilt per
        // element otherwise, and the result is known to be at most as long as
        // the input, so a single exact allocation replaces the growth steps.
        let f = f.unwrap_func1();
        match purust_core::IntItems::from(&xs) {
            // Filtering an unboxed Int array keeps it unboxed.
            purust_core::IntItems::Ints(values) => {
                let mut result = Vec::with_capacity(values.len());
                for value in values.iter() {
                    if f(Value::Int(*value)).unwrap_bool() {
                        result.push(*value);
                    }
                }
                mk_int_array(result)
            }
            purust_core::IntItems::Boxed(values) => {
                let mut result = Vec::with_capacity(values.len());
                for value in values.iter() {
                    if f(value.clone()).unwrap_bool() {
                        result.push(value.clone());
                    }
                }
                mk_array(result)
            }
        }
    }))
}

pub fn Data_Array_partitionImpl() -> UnknownType {
    Value::Func2(Func2::Static(|f, xs| {
        let f = f.unwrap_func1();
        let mut yes = Vec::new();
        let mut no = Vec::new();
        for value in xs.unwrap_array().iter() {
            if f(value.clone()).unwrap_bool() {
                yes.push(value.clone());
            } else {
                no.push(value.clone());
            }
        }
        let mut record = Record_a::default();
        record.set_field("yes", mk_array(yes));
        record.set_field("no", mk_array(no));
        Value::Record_a(PerceusPtr::new(record))
    }))
}

pub fn Data_Array_scanlImpl() -> UnknownType {
    Value::Func3(Func3::Static(|f, mut acc, xs| {
        let f = f.unwrap_func2();
        let xs = xs.unwrap_array();
        let mut result = Vec::with_capacity(xs.len() + 1);
        for value in xs.iter() {
            acc = f(acc, value.clone());
            result.push(acc.clone());
        }
        mk_array(result)
    }))
}

pub fn Data_Array_scanrImpl() -> UnknownType {
    Value::Func3(Func3::Static(|f, mut acc, xs| {
        let f = f.unwrap_func2();
        let xs = xs.unwrap_array();
        let mut result = Vec::with_capacity(xs.len() + 1);
        for value in xs.iter().rev() {
            acc = f(value.clone(), acc);
            result.push(acc.clone());
        }
        result.reverse();
        mk_array(result)
    }))
}

pub fn Data_Array_sortByImpl() -> UnknownType {
    Value::Func3(Func3::Static(|compare, from_ordering, xs| {
        // The comparator runs O(n log n) times; resolve both callbacks once.
        let compare = compare.unwrap_func2();
        let from_ordering = from_ordering.unwrap_func1();
        let mut result = xs.unwrap_array().as_ref().clone();
        result.sort_by(|left, right| {
            from_ordering(compare(left.clone(), right.clone()))
                .unwrap_int()
                .cmp(&0)
        });
        mk_array(result)
    }))
}

pub fn Data_Array_sliceImpl() -> UnknownType {
    Value::Func3(Func3::Static(|start, end, xs| {
        let len = match purust_core::IntItems::from(&xs) {
            purust_core::IntItems::Ints(ref values) => values.len() as i64,
            purust_core::IntItems::Boxed(ref values) => values.len() as i64,
        };
        let normalize = |i: i64| if i < 0 { (len + i).max(0) } else { i.min(len) } as usize;
        let start = normalize(start.unwrap_int());
        let end = normalize(end.unwrap_int()).max(start);
        // Slicing an unboxed Int array keeps it unboxed.
        match purust_core::IntItems::from(&xs) {
            purust_core::IntItems::Ints(values) => mk_int_array(values[start..end].to_vec()),
            purust_core::IntItems::Boxed(values) => mk_array(values[start..end].to_vec()),
        }
    }))
}

pub fn Data_Array_zipWithImpl() -> UnknownType {
    Value::Func3(Func3::Static(|f, xs, ys| {
        let f = f.unwrap_func2();
        let xs = xs.unwrap_array();
        let ys = ys.unwrap_array();
        let mut result = Vec::with_capacity(xs.len().min(ys.len()));
        for (x, y) in xs.iter().zip(ys.iter()) {
            result.push(f(x.clone(), y.clone()));
        }
        mk_array(result)
    }))
}

pub fn Data_Array_anyImpl() -> UnknownType {
    Value::Func2(Func2::Static(|f, xs| {
        let f = f.unwrap_func1();
        Value::Bool(xs.unwrap_array().iter().any(|x| f(x.clone()).unwrap_bool()))
    }))
}

pub fn Data_Array_allImpl() -> UnknownType {
    Value::Func2(Func2::Static(|f, xs| {
        let f = f.unwrap_func1();
        Value::Bool(xs.unwrap_array().iter().all(|x| f(x.clone()).unwrap_bool()))
    }))
}

pub fn Data_Array_unsafeIndexImpl() -> UnknownType {
    Value::Func2(Func2::Static(|xs, index| {
        // Borrow the buffer and clone only the read element.
        xs.array_get(index.unwrap_int() as usize)
    }))
}
