use std::collections::HashMap;

use crate::{sound, value};

use std::cell::{Cell, RefCell};
use std::rc::Rc;
type RcCell<T> = Rc<Cell<T>>;
type RcRefCell<T> = Rc<RefCell<T>>;

pub struct Function {
    pub body: Body,
    pub arguments: Vec<value::Value>,
    pub named_arguments: HashMap<String, value::Value>,
}

impl Function {
    pub fn primitive_real_1(fnc: fn(f64) -> f64) -> Function {
        let x = Rc::new(Cell::new(0.));
        Function {
            arguments: vec![value::Value::Real(x.clone())],
            named_arguments: HashMap::new(),
            body: Body::Real(Rc::new(RealFunction::Primitive1(fnc, x))),
        }
    }
    pub fn primitive_real_2(fnc: fn(f64, f64) -> f64) -> Function {
        let x = Rc::new(Cell::new(0.));
        let y = Rc::new(Cell::new(0.));
        Function {
            arguments: vec![value::Value::Real(x.clone()), value::Value::Real(y.clone())],
            named_arguments: HashMap::new(),
            body: Body::Real(Rc::new(RealFunction::Primitive2(fnc, x, y))),
        }
    }
    pub fn sin() -> Function {
        let x = Rc::new(Cell::new(0.));
        Function {
            arguments: vec![value::Value::Real(x.clone())],
            named_arguments: HashMap::new(),
            body: Body::Sound(Rc::new(SoundFunction::Sin(x))),
        }
    }
    pub fn exp() -> Function {
        let x = Rc::new(Cell::new(0.));
        Function {
            arguments: vec![value::Value::Real(x.clone())],
            named_arguments: HashMap::new(),
            body: Body::Sound(Rc::new(SoundFunction::Exp(x))),
        }
    }
    pub fn linear() -> Function {
        let x0 = Rc::new(Cell::new(0.));
        let x1 = Rc::new(Cell::new(0.));
        let t1 = Rc::new(Cell::new(0.));
        Function {
            arguments: vec![
                value::Value::Real(x0.clone()),
                value::Value::Real(x1.clone()),
                value::Value::Real(t1.clone()),
            ],
            named_arguments: vec![("t".to_string(), value::Value::Real(t1.clone()))]
                .into_iter()
                .collect(),
            body: Body::Sound(Rc::new(SoundFunction::Linear(x0, x1, t1))),
        }
    }
    pub fn write() -> Function {
        let sound = Rc::new(RefCell::new(sound::Sound::Const(0.)));
        let time = Rc::new(Cell::new(0.));
        let filename = Rc::new(RefCell::new("".to_string()));
        Function {
            arguments: vec![
                value::Value::Sound(sound.clone()),
                value::Value::Real(time.clone()),
                value::Value::String(filename.clone()),
            ],
            named_arguments: HashMap::new(),
            body: Body::Void(Rc::new(VoidFunction::Write(sound, time, filename))),
        }
    }
}

pub enum Body {
    Real(Rc<RealFunction>),
    Boolean(Rc<BooleanFunction>),
    Sound(Rc<SoundFunction>),
    String(Rc<StringFunction>),
    Void(Rc<VoidFunction>),
}

pub enum RealFunction {
    Primitive1(fn(f64) -> f64, RcCell<f64>),
    Primitive2(fn(f64, f64) -> f64, RcCell<f64>, RcCell<f64>),
}

impl RealFunction {
    pub fn evaluate(&self) -> f64 {
        match self {
            RealFunction::Primitive1(fnc, x) => fnc(x.get()),
            RealFunction::Primitive2(fnc, x, y) => fnc(x.get(), y.get()),
        }
    }
}

pub enum BooleanFunction {}

pub enum SoundFunction {
    Sin(RcCell<f64>),
    Linear(RcCell<f64>, RcCell<f64>, RcCell<f64>),
    Exp(RcCell<f64>),
}

impl SoundFunction {
    pub fn evaluate(&self) -> sound::Sound {
        match self {
            SoundFunction::Sin(frequency) => sound::Sound::Sin {
                frequency: frequency.get(),
                phase: 0.,
            },
            SoundFunction::Linear(x0, x1, t1) => {
                let x0 = x0.get();
                let x1 = x1.get();
                let t1 = t1.get();
                sound::Sound::Linear {
                    slope: (x1 - x0) / t1,
                    intercept: x0,
                }
            }
            SoundFunction::Exp(time) => sound::Sound::Exp {
                coefficient: 1. / time.get(),
                intercept: 1.,
            },
        }
    }
}

pub enum StringFunction {}

pub enum VoidFunction {
    Write(RcRefCell<sound::Sound>, RcCell<f64>, RcRefCell<String>),
}
impl VoidFunction {
    pub fn evaluate(&self) {
        match self {
            VoidFunction::Write(sound, time, filename) => {
                let samplerate = 44100;
                let mut iter = sound.borrow().iter(samplerate as f64);
                let spec = hound::WavSpec {
                    channels: 1,
                    sample_rate: samplerate,
                    bits_per_sample: 32,
                    sample_format: hound::SampleFormat::Int,
                };
                let mut writer = hound::WavWriter::create(&*filename.borrow(), spec).unwrap();
                let amplitude = std::i32::MAX as f64;
                for _ in 0..(time.get() * samplerate as f64) as i64 {
                    writer
                        .write_sample((amplitude * iter.next()) as i32)
                        .unwrap();
                }
                writer.finalize().unwrap();
            }
        }
    }
}
