use crate::{formula::FormulaList, calculate};

#[test]
fn test1() {
    let formula_list = "铁矿石 1s
铁锭 = 铁矿石 2s
2*铁板 = 3*铁锭 6s
铁棒 = 铁锭 4s
4*螺丝 = 铁棒 6s
铁制钢筋 = 铁棒 4s
加强铁板 = 6*铁板 + 12*螺丝 12s
转子 = 5*铁棒 + 25*螺丝 15s
2*模块化框架 = 3*加强铁板 + 12*铁棒 60s
铜矿石 1s
铜锭 = 铜矿石 2s
2*电线 = 铜矿石 4s
电缆 = 2*电线 2s
铜板 = 2*铜锭 6s
石灰石 1s
混凝土 = 3*石灰石 4s
煤 1s
3*钢锭 = 3*铁矿石 + 3*煤 4s
钢梁 = 4*钢锭 4s
2*钢管 = 3*钢锭 6s
钢筋混凝土梁 = 4*钢梁 + 5*混凝土 10s
重型模块化框架 = 5*模块化框架 + 15*钢管 + 5*钢筋混凝土梁 + 100*螺丝 30s
定子 = 3*钢管 + 8*电线 12s
电机 = 2*转子 + 2*定子 12s
20*生物质 = 4*木头 4s
4*固体生物燃料 = 8*生物质 4s
智能护板 = 加强铁板 + 转子 30s
2*多功能框架 = 模块化框架 + 12*钢梁 24s
自动线路 = 定子 + 20*电缆 24s";
    let formula_list = FormulaList::from_reader(formula_list.as_bytes()).unwrap();
    let source: Vec<std::rc::Rc<crate::Item>> = vec![formula_list.get_item("铁锭")];
    let target= formula_list.get_item("重型模块化框架");

    let r = calculate(&formula_list, &source, target, 4);
    for v in r{
        println!("{v}")
    }
}
