#[cfg(test)]
mod test;

pub mod formula;
pub mod fraction;
pub mod error;

use std::collections::HashMap;
use std::rc::Rc;

pub use error::{Error, Result};
pub use formula::{Formula, FormulaItem, Item, FormulaList, Plan};
pub use fraction::Fraction;

pub fn calculate<'a>(
    meta: &'a FormulaList,
    source: &[Rc<Item>],
    target: Rc<Item>,
    amount: u32,
) -> Vec<Plan> {
    let formulas = meta.formulas();
    let mut data: HashMap<_, _> = formulas
        .iter()
        .map(|x| {
            (
                x.target().item(),
                Data {
                    formula: x.clone(),
                    is_source: false,
                    used: 0,
                    speed: 0.into(),
                },
            )
        })
        .collect();
    for v in source {
        data.get_mut(v)
            .unwrap_or_else(|| todo!("unknown item"))
            .is_source = true;
    }

    let mut required_item: Vec<Rc<Item>> = Vec::new();
    required_item.push(target.clone());
    while let Some(target_i) = required_item.pop() {
        let d = data.get(&target_i).unwrap_or_else(|| todo!("unknown item"));
        if d.is_source {
            continue;
        }
        let formula = d.formula.clone();

        for input_i in formula.source() {
            let input_d = data.get_mut(&input_i.item()).unwrap();
            input_d.used += 1;
            if input_d.used == 1 {
                required_item.push(input_i.item())
            }
        }
    }

    dbg!(&data);

    data.get_mut(&target)
        .unwrap_or_else(|| todo!("missing target "))
        .speed = amount.into();
    let mut calculated_item: Vec<Rc<Item>> = Vec::new();
    calculated_item.push(target);
    let mut results: Vec<Plan> = Vec::new();
    while let Some(item) = calculated_item.pop() {
        let d = data.get(&item).unwrap();
        let speed: Fraction = d.speed.into();
        let speed_f = d.formula.speed();
        let speed_t = speed_f * d.formula.target().amount().into();
        let group = speed / speed_t;

        results.push(Plan {
            formula: d.formula.clone(),
            speed,
            group: group.cell(),
            rate: speed / (group.cell() * speed_t),
        });
        if d.is_source {
            continue;
        }
        let formula = d.formula.clone();
        for v in formula.source() {
            let sd = data.get_mut(&v.item()).unwrap();
            sd.used -= 1;
            sd.speed += group * speed_f * v.amount().into();
            if sd.used == 0 {
                calculated_item.push(v.item())
            }
        }
    }

    let results: Vec<_> = results.into_iter().rev().collect();
    results
}

#[derive(Debug)]
struct Data {
    formula: Rc<Formula>,
    is_source: bool,
    used: u32,
    speed: Fraction,
}
