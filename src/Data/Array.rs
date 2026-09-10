use purust_core::*;
use std::rc::Rc;
use std::sync::Mutex;

pub fn Data_Array_length(xs: UnknownType) -> i64 {
    xs.unwrap_array().len() as i64
}

pub fn Data_Array_rangeImpl() -> UnknownType {
    Value::Func2(Func2::Static(|start, end| {
        let start = start.unwrap_int();
        let end = end.unwrap_int();
        mk_array(if start <= end {
            (start..=end).map(Value::Int).collect()
        } else {
            (end..=start).rev().map(Value::Int).collect()
        })
    }))
}

pub fn Data_Array_replicateImpl() -> UnknownType {
    Value::Func2(Func2::Static(|count, value| {
        mk_array(vec![value; count.unwrap_int().max(0) as usize])
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
        let xs = xs.unwrap_array();
        match usize::try_from(index.unwrap_int())
            .ok()
            .and_then(|i| xs.get(i))
        {
            Some(value) => just.unwrap_func1()(value.clone()),
            None => nothing,
        }
    }))
}

pub fn Data_Array_findMapImpl() -> UnknownType {
    Value::Func4(Func4::Static(|nothing, is_just, f, xs| {
        for value in xs.unwrap_array().iter() {
            let result = f.unwrap_func1()(value.clone());
            if is_just.unwrap_func1()(result.clone()).unwrap_bool() {
                return result;
            }
        }
        nothing
    }))
}

pub fn Data_Array_findIndexImpl() -> UnknownType {
    Value::Func4(Func4::Static(|just, nothing, f, xs| {
        for (i, value) in xs.unwrap_array().iter().enumerate() {
            if f.unwrap_func1()(value.clone()).unwrap_bool() {
                return just.unwrap_func1()(Value::Int(i as i64));
            }
        }
        nothing
    }))
}

pub fn Data_Array_findLastIndexImpl() -> UnknownType {
    Value::Func4(Func4::Static(|just, nothing, f, xs| {
        for (i, value) in xs.unwrap_array().iter().enumerate().rev() {
            if f.unwrap_func1()(value.clone()).unwrap_bool() {
                return just.unwrap_func1()(Value::Int(i as i64));
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
    mk_array(xs.unwrap_array().iter().rev().cloned().collect())
}

pub fn Data_Array_concat(arrays: UnknownType) -> UnknownType {
    let mut result = Vec::new();
    for xs in arrays.unwrap_array().iter() {
        result.extend(xs.unwrap_array().iter().cloned());
    }
    mk_array(result)
}

pub fn Data_Array_filterImpl() -> UnknownType {
    Value::Func2(Func2::Static(|f, xs| {
        mk_array(
            xs.unwrap_array()
                .iter()
                .filter(|x| f.unwrap_func1()((*x).clone()).unwrap_bool())
                .cloned()
                .collect(),
        )
    }))
}

pub fn Data_Array_partitionImpl() -> UnknownType {
    Value::Func2(Func2::Static(|f, xs| {
        let mut yes = Vec::new();
        let mut no = Vec::new();
        for value in xs.unwrap_array().iter() {
            if f.unwrap_func1()(value.clone()).unwrap_bool() {
                yes.push(value.clone());
            } else {
                no.push(value.clone());
            }
        }
        let mut record = Record_a::default();
        record.yes = Some(mk_array(yes));
        record.no = Some(mk_array(no));
        Value::Record_a(PerceusPtr::new(record))
    }))
}

pub fn Data_Array_scanlImpl() -> UnknownType {
    Value::Func3(Func3::Static(|f, mut acc, xs| {
        let mut result = Vec::new();
        for value in xs.unwrap_array().iter() {
            acc = f.unwrap_func2()(acc, value.clone());
            result.push(acc.clone());
        }
        mk_array(result)
    }))
}

pub fn Data_Array_scanrImpl() -> UnknownType {
    Value::Func3(Func3::Static(|f, mut acc, xs| {
        let mut result = Vec::new();
        for value in xs.unwrap_array().iter().rev() {
            acc = f.unwrap_func2()(value.clone(), acc);
            result.push(acc.clone());
        }
        result.reverse();
        mk_array(result)
    }))
}

pub fn Data_Array_sortByImpl() -> UnknownType {
    Value::Func3(Func3::Static(|compare, from_ordering, xs| {
        let mut result = xs.unwrap_array().as_ref().clone();
        result.sort_by(|left, right| {
            from_ordering.unwrap_func1()(compare.unwrap_func2()(left.clone(), right.clone()))
                .unwrap_int()
                .cmp(&0)
        });
        mk_array(result)
    }))
}

pub fn Data_Array_sliceImpl() -> UnknownType {
    Value::Func3(Func3::Static(|start, end, xs| {
        let xs = xs.unwrap_array();
        let len = xs.len() as i64;
        let normalize = |i: i64| if i < 0 { (len + i).max(0) } else { i.min(len) } as usize;
        let start = normalize(start.unwrap_int());
        let end = normalize(end.unwrap_int()).max(start);
        mk_array(xs[start..end].to_vec())
    }))
}

pub fn Data_Array_zipWithImpl() -> UnknownType {
    Value::Func3(Func3::Static(|f, xs, ys| {
        mk_array(
            xs.unwrap_array()
                .iter()
                .zip(ys.unwrap_array().iter())
                .map(|(x, y)| f.unwrap_func2()(x.clone(), y.clone()))
                .collect(),
        )
    }))
}

pub fn Data_Array_anyImpl() -> UnknownType {
    Value::Func2(Func2::Static(|f, xs| {
        Value::Bool(
            xs.unwrap_array()
                .iter()
                .any(|x| f.unwrap_func1()(x.clone()).unwrap_bool()),
        )
    }))
}

pub fn Data_Array_allImpl() -> UnknownType {
    Value::Func2(Func2::Static(|f, xs| {
        Value::Bool(
            xs.unwrap_array()
                .iter()
                .all(|x| f.unwrap_func1()(x.clone()).unwrap_bool()),
        )
    }))
}

pub fn Data_Array_unsafeIndexImpl() -> UnknownType {
    Value::Func2(Func2::Static(|xs, index| {
        xs.unwrap_array()[index.unwrap_int() as usize].clone()
    }))
}
