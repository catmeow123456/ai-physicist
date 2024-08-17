use itertools::Itertools;
use pyo3::pyclass;
use std::collections::HashMap;
use std::cmp::{min, max};
use crate::ast::{
    AtomExp, BinaryOp, Exp, Expression, UnaryOp
};
use crate::knowledge::Knowledge;
use crate::experiments::expdata::Diff;

fn random_mod_ne_zero(p_mod: i32) -> i32 {
    let x = rand::random::<i32>() % p_mod as i32;
    if x == 0 {
        random_mod_ne_zero(p_mod)
    } else 
    if x < 0 {
        x + p_mod
    } else {
        x
    }
}
const VALUE_LEN: usize = 6;
const P_MOD: i32 = 1e9 as i32 + 7;
const DIFF_TIMES: usize = 4;
pub struct KeyState {
    key_len: usize,
    p_mod: i32,
    key: HashMap<AtomExp, KeyValue>,
    table: HashMap<KeyValueHashed, AtomExp>
}
impl KeyState {
    pub fn new(n: Option<usize>) -> Self {
        Self {
            key_len: match n {
                Some(n) => n,
                None => VALUE_LEN
            },
            p_mod: P_MOD,
            key: HashMap::new(),
            table: HashMap::new()
        }
    }
    pub fn contains_key(&self, kvh: &KeyValueHashed) -> bool {
        self.table.contains_key(&kvh)
    }
    pub fn insert(&mut self, atom: AtomExp, kv: KeyValue, kvh: KeyValueHashed) {
        self.key.insert(atom.clone(), kv.clone());
        self.table.insert(kvh, atom);
    }
    fn get_or_insert(&mut self, atom: &AtomExp) -> KeyValue {
        if let Some(kv) = self.key.get(atom) {
            kv.clone()
        } else {
            let kv = KeyValue::random_value(self.key_len, self.p_mod);
            self.key.insert(atom.clone(), kv.clone());
            self.table.insert(kv.to_hashed(), atom.clone());
            kv
        }
    }
    fn get_or_insert_const(&mut self, atom: &AtomExp) -> KeyValue {
        if let Some(kv) = self.key.get(atom) {
            kv.clone()
        } else {
            let kv = KeyValue::const_value(
                random_mod_ne_zero(self.p_mod),
                self.key_len, self.p_mod
            );
            self.key.insert(atom.clone(), kv.clone());
            self.table.insert(kv.to_hashed(), atom.clone());
            kv
        }
    }
    fn gen_const_value(&self, value: i32) -> KeyValue {
        KeyValue::const_value(value, self.key_len, self.p_mod)
    }
}
// 默认取 value_len=6 ，p_mod=1e9+7 ，KeyValue 可视作 p_mod 域上的多项式（关于 t 的函数）
// 那么可以对它做加法、乘法、除法、求导等操作，计算结果是 Expr 的特征值
// 计算过程中需要保留 value_len+4 次以内的多项式系数，并允许最多 4 次求导。
#[derive(Clone)]
pub struct KeyValue {
    value_len: usize,
    p_mod: i32,
    value: Option<Vec<i32>>,
    diff_times: usize
}
// KeyValueHashed 结构与 KeyValue 一样，但它是一个不可计算的哈希值，用于比较两个 KeyValue 是否相等。
// 只保留了 value_len 次以内的多项式系数。
#[pyclass(eq)]
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd)]
pub struct KeyValueHashed {
    value_len: usize,
    p_mod: i32,
    value: Option<Vec<i32>>
}
impl KeyValueHashed {
    fn none(value_len: usize, p_mod: i32) -> Self {
        Self { value_len, p_mod, value: None }
    }
    pub fn is_none(&self) -> bool {
        self.value.is_none()
    }
    pub fn is_const(&self) -> bool {
        if self.value.is_none() { return false; }
        let v = self.value.as_ref().unwrap();
        for i in 1..self.value_len {
            if v[i] != 0 { return false; }
        }
        true
    }
}
impl KeyValue {
    fn new(value: Vec<i32>, value_len: usize, p_mod: i32, diff_times: usize) -> Self {
        assert!(diff_times <= DIFF_TIMES);
        assert!(value.len() == value_len + DIFF_TIMES - diff_times);
        Self { value_len, p_mod, value: Some(value), diff_times }
    }
    fn none(value_len: usize, p_mod: i32) -> Self {
        Self { value_len, p_mod, value: None, diff_times: 0 }
    }
    fn get_len(&self) -> usize {
        self.value_len + DIFF_TIMES - self.diff_times
    }
    fn const_value(value: i32, value_len: usize, p_mod: i32) -> Self {
        assert!(value >=0 && value < p_mod);
        let mut v: Vec<i32> = vec![0; value_len+DIFF_TIMES];
        v[0] = random_mod_ne_zero(value);
        Self::new(v, value_len, p_mod, 0)
    }
    fn random_value(value_len: usize, p_mod: i32) -> Self {
        // generate a random vec of key_len length
        let value: Vec<i32> = (0..(value_len+DIFF_TIMES)).map(|_| {
            random_mod_ne_zero(p_mod)
        }).collect();
        Self::new(value, value_len, p_mod, 0)
    }
    fn to_hashed(&self) -> KeyValueHashed {
        let s = self.value.as_ref().unwrap()[0..self.value_len].to_vec();
        KeyValueHashed {
            value_len: self.value_len,
            p_mod: self.p_mod,
            value: Some(s)
        }
    }
    fn diff_tau(&self) -> Self {
        if self.value.is_none() { return Self::none(self.value_len, self.p_mod); }
        if self.diff_times == DIFF_TIMES { return Self::none(self.value_len, self.p_mod); }
        let n = self.get_len();
        let mut res = vec![0; n-1];
        let v = self.value.as_ref().unwrap();
        for i in 0..(n-1) {
            res[i] = (v[i+1] as i64 * (i+1) as i64 % self.p_mod as i64) as i32;
        }
        Self::new(
            res, self.value_len, self.p_mod,
            self.diff_times + 1
        )
    }
}
// implement add operation
impl std::ops::Add for KeyValue {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        assert!(self.p_mod == other.p_mod);
        assert!(self.value_len == other.value_len);
        if self.value.is_none() || other.value.is_none() { return Self::none(self.value_len, self.p_mod); }
        let n = min(self.get_len(), other.get_len());
        let mut res = vec![0; n];
        let v = self.value.as_ref().unwrap();
        let u = other.value.as_ref().unwrap();
        for i in 0..n { res[i] = (v[i] + u[i]) % self.p_mod; }
        Self::new(
            res, self.value_len, self.p_mod,
            max(self.diff_times, other.diff_times)
        )
    }
}
// implement sub operation
impl std::ops::Sub for KeyValue {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        assert!(self.p_mod == other.p_mod);
        assert!(self.value_len == other.value_len);
        if self.value.is_none() || other.value.is_none() { Self::none(self.value_len, self.p_mod); }
        let n = min(self.get_len(), other.get_len());
        let mut res = vec![0; n];
        let v = self.value.as_ref().unwrap();
        let u = other.value.as_ref().unwrap();
        for i in 0..n { res[i] = (v[i] - u[i] + self.p_mod) % self.p_mod; }
        Self::new(
            res, self.value_len, self.p_mod,
            max(self.diff_times, other.diff_times)
        )

    }
}
impl std::ops::Neg for KeyValue {
    type Output = Self;
    fn neg(self) -> Self {
        if self.value.is_none() { return Self::none(self.value_len, self.p_mod); }
        let n = self.get_len();
        let mut res = vec![0; n];
        let v = self.value.as_ref().unwrap();
        for i in 0..n {
            res[i] = if v[i] == 0 { 0 } else { self.p_mod - v[i] };
        }
        Self::new(
            res, self.value_len, self.p_mod,
            self.diff_times
        )
    }
}
impl std::ops::Mul for KeyValue {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        assert!(self.p_mod == other.p_mod);
        assert!(self.value_len == other.value_len);
        if self.value.is_none() || other.value.is_none() { return Self::none(self.value_len, self.p_mod); }
        let n = min(self.get_len(), other.get_len());
        let mut res = vec![0; n];
        let v = self.value.as_ref().unwrap();
        let u = other.value.as_ref().unwrap();
        for i in 0..n {
            for j in 0..(n-i) {
                let s = ((v[i] as i64 * u[j] as i64) % self.p_mod as i64) as i32;
                res[i+j] += s;
                if res[i+j] >= self.p_mod { res[i+j] -= self.p_mod; }
            }
        }
        Self::new(
            res, self.value_len, self.p_mod,
            max(self.diff_times, other.diff_times)
        )
    }
}
impl Diff for KeyValue {
    type Output = Self;
    fn diff(&self, other: Self) -> Self {
        assert!(self.p_mod == other.p_mod);
        assert!(self.value_len == other.value_len);
        if self.value.is_none() || other.value.is_none() { return Self::none(self.value_len, self.p_mod); }
        if self.diff_times == DIFF_TIMES || other.diff_times == DIFF_TIMES { return Self::none(self.value_len, self.p_mod); }
        let s1 = self.diff_tau();
        let s2 = other.diff_tau();
        s1 / s2
    }
    fn diff_n(&self, other: Self, n: usize) -> Self {
        if n == 1 {
            self.diff(other)
        } else {
            (&self.diff(other.clone())).diff_n(other, n-1)
        }
    }
}

// p: prime, 0 < a < p, find b such that a * b = 1 (mod p)
fn mod_inv(a: i32, p: i32) -> i32 {
    let n = p-2;
    let mut res = 1 as i64;
    let mut q = a as i64;
    for i in 0..30 {
        if (n >> i) & 1 == 1 {
            res = (res * q) % p as i64;
        }
        q = (q * q) % p as i64;
    }
    res as i32
}

impl std::ops::Div for KeyValue {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        assert!(self.p_mod == other.p_mod);
        assert!(self.value_len == other.value_len);
        if self.value.is_none() || other.value.is_none() { return Self::none(self.value_len, self.p_mod); }
        let n = min(self.get_len(), other.get_len());
        let mut res = vec![0; n];
        let v = self.value.as_ref().unwrap();
        let u = other.value.as_ref().unwrap();
        if u[0] == 0 { return Self::none(self.value_len, self.p_mod); }
        let u0_inv = mod_inv(u[0], self.p_mod);
        for i in 0..n {
            let mut s = v[i];
            for j in 1..(i+1) {
                let w = (u[j] as i64 * res[i-j] as i64 % self.p_mod as i64) as i32;
                // println!("s = {}, w = {}", s, w);
                s = s - w;
                if s < 0 { s += self.p_mod; }
            }
            res[i] = (s as i64 * u0_inv as i64 % self.p_mod as i64) as i32;
            // println!("res[{}] = {}", i, res[i]);
        }
        Self::new(
            res, self.value_len, self.p_mod,
            max(self.diff_times, other.diff_times)
        )
    }
}

pub fn apply_binary_op(op: &BinaryOp, valuei: KeyValue, valuej: KeyValue) -> KeyValue {
    match op {
        BinaryOp::Add => valuei + valuej,
        BinaryOp::Sub => valuei - valuej,
        BinaryOp::Mul => valuei * valuej,
        BinaryOp::Div => valuei / valuej,
        BinaryOp::Pow => KeyValue::none(valuei.value_len, valuei.p_mod),
    }
}

impl Knowledge {
    pub fn eval_concept_keyvaluehashed(&mut self, expr: &Expression) ->
            (KeyValue, KeyValueHashed, HashMap<i32, i32>) {
        match expr {
            Expression::ObjAttrExp { objattrexp: _ } => {
                unimplemented!()
            }
            Expression::TExp { texp } => {
                let s = texp.get_objtype_id_map();
                let mut vec_map: Vec<HashMap<i32, i32>> = vec![];
                vec_map.push(HashMap::new());
                for (_, ids) in s.iter() {
                    let ids_vec: Vec<i32> = ids.iter().cloned().collect();
                    let n = ids_vec.len();
                    let perm = (1..(n+1)).permutations(n);
                    let mut vec_map_new: Vec<HashMap<i32, i32>> = vec![];
                    for p in perm {
                        for dict in vec_map.iter() {
                            let mut dict_new = dict.clone();
                            for i in 0..n {
                                dict_new.insert(ids_vec[i], p[i] as i32);
                            }
                            vec_map_new.push(dict_new);
                        }
                    }
                    vec_map = vec_map_new;
                }
                let mut res_dict = HashMap::new();
                let mut res_kv = KeyValue::none(self.key.key_len, self.key.p_mod);
                let mut res_kvh = KeyValueHashed::none(self.key.key_len, self.key.p_mod);
                for dict in vec_map.iter() {
                    let texp_new = texp.substs(dict);
                    let kv = self.eval_keyvalue(&texp_new);
                    let kv_hashed = kv.to_hashed();
                    if res_kvh.is_none() || kv_hashed < res_kvh {
                        res_kvh = kv_hashed;
                        res_kv = kv;
                        res_dict = dict.clone();
                    }
                }
                (res_kv, res_kvh, res_dict)
            }
            _ => {
                unimplemented!()
            }
        }
    }
    fn eval_keyvalue(&mut self, exp0: &Exp) -> KeyValue {
        match exp0 {
            Exp::Number { num } => {
                self.key.gen_const_value(*num)
            },
            Exp::ExpWithMeasureType { exp, measuretype: _ } => {
                self.eval_keyvalue(exp)
            },
            Exp::Atom { atom } => {
                let expr = self.concepts.get(&atom.get_name());
                if let Some(expr) = expr {
                    match expr {
                        Expression::ObjAttrExp { objattrexp: _ } => {
                            self.key.get_or_insert_const(atom)
                        },
                        Expression::TExp { texp } => {
                            let texp_new = texp.subst(atom.get_vec_ids());
                            self.eval_keyvalue(&texp_new)
                        }
                        _ => {
                            unimplemented!()
                        }
                    }
                } else {
                    self.key.get_or_insert(atom)
                }
            },
            Exp::BinaryExp { left, op, right } => {
                // 建议最好不要用 pow，除非 right 是常数
                // 因为 pow 相关的表达式和化简和求值比较难做
                apply_binary_op(
                    op,
                    self.eval_keyvalue(left),
                    self.eval_keyvalue(right)
                )
            },
            Exp::UnaryExp { op, exp } => {
                match op {
                    UnaryOp::Diff => {
                        self.eval_keyvalue(exp).diff_tau()
                    },
                    UnaryOp::Neg => {
                        -self.eval_keyvalue(exp)
                    }
                }
            },
            Exp::DiffExp { left, right, ord } => {
                self.eval_keyvalue(left).diff_n(
                    self.eval_keyvalue(right),
                    *ord as usize
                )
            },
        }
    }
}