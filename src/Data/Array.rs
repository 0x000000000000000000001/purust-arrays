use purust_core::*;
use std::rc::Rc;

pub fn Data_Array_length() -> UnknownType {
    perceus_ptr::PerceusPtr::new(Record_a {
        call: Some(Rc::new(move |mut xs: UnknownType| -> UnknownType {
            let arr = xs.init_array.as_ref().unwrap();
            mk_int(arr.len() as i64)
        })),
        ..Default::default()
    })
}

pub fn Data_Array_unconsImpl() -> UnknownType {
    perceus_ptr::PerceusPtr::new(Record_a {
        call: Some(Rc::new(move |mut empty: UnknownType| -> UnknownType {
            perceus_ptr::PerceusPtr::new(Record_a {
                call: Some(Rc::new(move |mut next: UnknownType| -> UnknownType {
                    let empty = empty.clone();
                    perceus_ptr::PerceusPtr::new(Record_a {
                        call: Some(Rc::new(move |mut xs: UnknownType| -> UnknownType {
                            let mut empty = empty.clone();
                            let mut next = next.clone();
                            let arr = xs.init_array.as_ref().unwrap();
                            if arr.is_empty() {
                                empty.call.clone().unwrap()(UnknownType::new(Record_a { ..Default::default() }))
                            } else {
                                let head = arr[0].clone();
                                let tail = arr[1..].to_vec();
                                next.call.clone().unwrap()(head).call.clone().unwrap()(crate::mk_array(tail))
                            }
                        })),
                        ..Default::default()
                    })
                })),
                ..Default::default()
            })
        })),
        ..Default::default()
    })
}

pub fn Data_Array_indexImpl() -> UnknownType {
    perceus_ptr::PerceusPtr::new(Record_a {
        call: Some(Rc::new(move |mut just: UnknownType| -> UnknownType {
            perceus_ptr::PerceusPtr::new(Record_a {
                call: Some(Rc::new(move |mut nothing: UnknownType| -> UnknownType {
                    let just = just.clone();
                    perceus_ptr::PerceusPtr::new(Record_a {
                        call: Some(Rc::new(move |mut xs: UnknownType| -> UnknownType {
                            let just = just.clone();
                            let nothing = nothing.clone();
                            perceus_ptr::PerceusPtr::new(Record_a {
                                call: Some(Rc::new(move |mut i: UnknownType| -> UnknownType {
                                    let mut just = just.clone();
                                    let nothing = nothing.clone();
                                    let arr = xs.init_array.as_ref().unwrap();
                                    let idx_raw = i.init_int.unwrap();
                                    if idx_raw < 0 || idx_raw >= arr.len() as i64 {
                                        nothing
                                    } else {
                                        just.call.clone().unwrap()(arr[idx_raw as usize].clone())
                                    }
                                })),
                                ..Default::default()
                            })
                        })),
                        ..Default::default()
                    })
                })),
                ..Default::default()
            })
        })),
        ..Default::default()
    })
}

pub fn Data_Array_sliceImpl() -> UnknownType {
    perceus_ptr::PerceusPtr::new(Record_a {
        call: Some(Rc::new(move |mut s: UnknownType| -> UnknownType {
            perceus_ptr::PerceusPtr::new(Record_a {
                call: Some(Rc::new(move |mut e: UnknownType| -> UnknownType {
                    let s = s.clone();
                    perceus_ptr::PerceusPtr::new(Record_a {
                        call: Some(Rc::new(move |mut l: UnknownType| -> UnknownType {
                            let arr = l.init_array.as_ref().unwrap();
                            let start = std::cmp::max(0, s.init_int.unwrap()) as usize;
                            let end = std::cmp::min(arr.len() as i64, e.init_int.unwrap()) as usize;
                            let start = std::cmp::min(start, arr.len());
                            let end = std::cmp::max(start, end);
                            let sliced = arr[start..end].to_vec();
                            crate::mk_array(sliced)
                        })),
                        ..Default::default()
                    })
                })),
                ..Default::default()
            })
        })),
        ..Default::default()
    })
}

pub fn Data_Array_findIndexImpl() -> UnknownType {
    perceus_ptr::PerceusPtr::new(Record_a {
        call: Some(Rc::new(move |mut just: UnknownType| -> UnknownType {
            perceus_ptr::PerceusPtr::new(Record_a {
                call: Some(Rc::new(move |mut nothing: UnknownType| -> UnknownType {
                    let just = just.clone();
                    perceus_ptr::PerceusPtr::new(Record_a {
                        call: Some(Rc::new(move |mut f: UnknownType| -> UnknownType {
                            let just = just.clone();
                            let nothing = nothing.clone();
                            perceus_ptr::PerceusPtr::new(Record_a {
                                call: Some(Rc::new(move |mut xs: UnknownType| -> UnknownType {
                                    let mut just = just.clone();
                                    let nothing = nothing.clone();
                                    let mut f = f.clone();
                                    let arr = xs.init_array.as_ref().unwrap();
                                    for (i, x) in arr.iter().enumerate() {
                                        let res = f.call.clone().unwrap()(x.clone());
                                        if res.init_bool.unwrap() {
                                            return just.call.clone().unwrap()(mk_int(i as i64));
                                        }
                                    }
                                    nothing
                                })),
                                ..Default::default()
                            })
                        })),
                        ..Default::default()
                    })
                })),
                ..Default::default()
            })
        })),
        ..Default::default()
    })
}

pub fn Data_Array_filterImpl() -> UnknownType {
    perceus_ptr::PerceusPtr::new(Record_a {
        call: Some(Rc::new(move |mut f: UnknownType| -> UnknownType {
            perceus_ptr::PerceusPtr::new(Record_a {
                call: Some(Rc::new(move |mut xs: UnknownType| -> UnknownType {
                    let mut f = f.clone();
                    let arr = xs.init_array.as_ref().unwrap();
                    let mut res = Vec::new();
                    for x in arr.iter() {
                        let ok = f.call.clone().unwrap()(x.clone());
                        if ok.init_bool.unwrap() {
                            res.push(x.clone());
                        }
                    }
                    crate::mk_array(res)
                })),
                ..Default::default()
            })
        })),
        ..Default::default()
    })
}

pub fn Data_Array_rangeImpl() -> UnknownType {
    perceus_ptr::PerceusPtr::new(Record_a {
        call: Some(Rc::new(move |mut start: UnknownType| -> UnknownType {
            perceus_ptr::PerceusPtr::new(Record_a {
                call: Some(Rc::new(move |mut end: UnknownType| -> UnknownType {
                    let s = start.init_int.unwrap();
                    let e = end.init_int.unwrap();
                    let mut res = Vec::new();
                    if s <= e {
                        for i in s..=e {
                            res.push(crate::mk_int(i));
                        }
                    } else {
                        let mut i = s;
                        while i >= e {
                            res.push(crate::mk_int(i));
                            i -= 1;
                        }
                    }
                    crate::mk_array(res)
                })),
                ..Default::default()
            })
        })),
        ..Default::default()
    })
}
