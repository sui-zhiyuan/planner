use crate::{model::Metadata, calculate};

/*
1*智能护板 = 1*加强铁板 + 1*转子 30s
1*转子 = 5*铁棒 + 25*螺丝 15s
1*加强铁板 = 6*铁板 + 12*螺丝 12s
4*螺丝 = 1*铁棒 6s
1*铁棒 = 1*铁锭 4s
2*铁板 = 3*铁锭 6s
1*铁锭 = 1*铁矿石 2s
 */
#[test]
fn test1() {
    let value: Vec<((&str, u32), Vec<(&str, u32)>, u32)> = vec![
        (("智能护板", 1), w2("加强铁板", 1, "转子", 1), 30),
        (("转子", 1), w2("铁棒", 5, "螺丝", 25), 15),
        (("加强铁板", 1), w2("铁板", 6, "螺丝", 12), 12),
        (("螺丝", 4), w1("铁棒", 1), 6),
        (("铁棒", 1), w1("铁锭", 1), 4),
        (("铁板", 2), w1("铁锭", 3), 6),
        (("铁锭", 1), w1("铁矿石", 1), 2),
    ];

    let meta = Metadata::from(value.into_iter());
    let source = vec![meta.get_item("铁锭")];
    let target= meta.get_item("智能护板");

    let r = calculate(&meta, &source, target, 8);
    for v in r{
        println!("{v}")
    }
}

fn w2(v1: &'static str, n1: u32, v2: &'static str, n2: u32) -> Vec<(&'static str, u32)> {
    vec![(v1, n1), (v2, n2)]
}

fn w1(v1: &'static str, n1: u32) -> Vec<(&'static str, u32)> {
    vec![(v1, n1)]
}