use crate::{formula::FormulaList, calculate};

#[test]
fn test1() {
    let formula_list = "铁矿石 1s
铁锭 = 铁矿石 2s
2*铁板 = 3*铁锭 6s
铁棒 = 铁锭 4s
4*螺丝 = 铁棒 6s
加强铁板 = 6*铁板 + 12*螺丝 12s
转子 = 5*铁棒 + 25*螺丝 15s
智能护板 = 加强铁板 + 转子 30s";
    let formula_list = FormulaList::from_reader(formula_list.as_bytes()).unwrap();
    let source: Vec<std::rc::Rc<crate::Item>> = vec![formula_list.get_item("铁锭")];
    let target= formula_list.get_item("智能护板");

    let r = calculate(&formula_list, &source, target, 8);
    for v in r{
        println!("{v}")
    }
}
