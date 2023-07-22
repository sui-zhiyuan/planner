use std::{
    fmt::{Debug, Display, Formatter},
    rc::Rc,
};

use crate::Fraction;

pub struct Metadata {
    items: Vec<Rc<Item>>,
    formulas: Vec<Rc<Formula>>,
}

impl Metadata {
    pub fn items(&self) -> &[Rc<Item>] {
        &self.items
    }

    pub fn formulas<'a>(&'a self) -> &[Rc<Formula>] {
        &self.formulas
    }

    pub fn from<'a, T>(source: T) -> Self
    where
        T: Iterator<Item = ((&'a str, u32), Vec<(&'a str, u32)>, u32)>,
    {
        let mut meta = Self::new();
        let formulas: Vec<Rc<Formula>> = source
            .into_iter()
            .map(|(target, source, round)| Formula {
                target: FormulaItem(meta.get_or_create_item(target.0), target.1),
                source: source
                    .iter()
                    .map(|x| FormulaItem(meta.get_or_create_item(x.0), x.1))
                    .collect(),
                speed: Fraction::new(60, round),
            })
            .map(Rc::new)
            .collect();
        meta.formulas = formulas;
        meta
    }

    fn new() -> Self {
        Self {
            items: Vec::new(),
            formulas: Vec::new(),
        }
    }

    fn get_or_create_item(&mut self, name: &str) -> Rc<Item> {
        for v in self.items.iter() {
            if v.name == name {
                return v.clone();
            }
        }

        let v = Rc::new(Item {
            name: name.to_string(),
        });
        self.items.push(v.clone());
        v
    }

    pub fn get_item(&self, name: &str) -> Rc<Item> {
        for v in self.items.iter() {
            if v.name == name {
                return v.clone();
            }
        }
        todo!("not found")
    }
}

#[derive(PartialEq, Eq, Hash)]
pub struct Item {
    name: String,
}

impl Item {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let v = &self.name;
        write!(f, "{v}")?;
        Ok(())
    }
}

impl Debug for Item {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Item::fmt(&self, f)
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Item::fmt(&self, f)
    }
}

#[derive(Clone)]
pub struct Formula {
    target: FormulaItem,
    source: Vec<FormulaItem>,
    /// round time by second
    speed: Fraction,
}

impl Formula {
    pub fn target(&self) -> &FormulaItem {
        &self.target
    }

    pub fn source(&self) -> &[FormulaItem] {
        &self.source
    }

    pub fn speed(&self) -> Fraction {
        self.speed
    }

    fn fmt(&self, f: &mut Formatter<'_>, times: Fraction) -> std::fmt::Result {
        let FormulaItem(target, amount) = &self.target;
        let amount = times * amount.clone().into();
        write!(f, "{target:?}*{amount:?} =")?;
        for (i, v) in self.source.iter().enumerate() {
            if i != 0 {
                write!(f, " +")?;
            }
            let FormulaItem(source, amount) = v;
            let amount = times * amount.clone().into();
            write!(f, " {source:?}*{amount:?}")?;
        }
        if times == 1.into() {
            let speed = self.speed;
            write!(f, " {speed:?}/min")?;
        }
        Ok(())
    }
}

impl Debug for Formula {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Formula::fmt(self, f, 1.into())
    }
}

impl Display for Formula {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Formula::fmt(self, f, 1.into())
    }
}

#[derive(Debug, Clone)]
pub struct FormulaItem(Rc<Item>, u32);

impl<'a> FormulaItem {
    pub fn item(&self) -> Rc<Item> {
        self.0.clone()
    }
    pub fn amount(&self) -> u32 {
        self.1
    }
}

#[derive(Debug)]
pub struct Result {
    pub formula: Rc<Formula>,
    pub speed: Fraction,
    pub group: Fraction,
    pub rate: Fraction,
}

impl Display for Result {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Result {
            formula,
            speed,
            group,
            rate,
        } = self;
        let rate = *rate * 100.into();
        write!(
            f,
            "group={group:2} speed={speed:6.2}/min rate={rate:6.2}% ==> "
        )?;
        Formula::fmt(formula, f, *speed / formula.target.amount().into())
    }
}
