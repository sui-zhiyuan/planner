use super::*;

#[test]
fn test_list_from_reader() {
    let input = "智能护板 = 加强铁板 + 转子 30s
转子 = 5*铁棒 + 25*螺丝 15s
加强铁板 = 6*铁板 + 12*螺丝 12s";
    // let output = FormulaList

    let items: Vec<Rc<Item>> = vec![
        Rc::new(Item {
            name: "智能护板".to_string(),
        }),
        Rc::new(Item {
            name: "加强铁板".to_string(),
        }),
        Rc::new(Item {
            name: "转子".to_string(),
        }),
        Rc::new(Item {
            name: "铁棒".to_string(),
        }),
        Rc::new(Item {
            name: "螺丝".to_string(),
        }),
        Rc::new(Item {
            name: "铁板".to_string(),
        })
    ];

    let formula_list = FormulaList {
        items: items.clone(),
        formulas: vec![
            Rc::new(Formula {
                target: FormulaItem(get_item(&items, "智能护板"), 1),
                source: vec![
                    FormulaItem(get_item(&items, "加强铁板"), 1),
                    FormulaItem(get_item(&items, "转子"), 1),
                ],
                speed: 2.into(),
            }),
            Rc::new(Formula {
                target: FormulaItem(get_item(&items, "转子"), 1),
                source: vec![
                    FormulaItem(get_item(&items, "铁棒"), 5),
                    FormulaItem(get_item(&items, "螺丝"), 25),
                ],
                speed: 4.into(),
            }),
            Rc::new(Formula {
                target: FormulaItem(get_item(&items, "加强铁板"), 1),
                source: vec![
                    FormulaItem(get_item(&items, "铁板"), 6),
                    FormulaItem(get_item(&items, "螺丝"), 12),
                ],
                speed: 5.into(),
            }),
        ],
    };

    let output = FormulaList::from_reader(input.as_bytes()).unwrap();
    assert_eq!(formula_list, output);
}

fn get_item(items: &Vec<Rc<Item>>, name: &str) -> Rc<Item> {
    for v in items {
        if v.name == name {
            return v.clone();
        }
    }
    panic!("not found item {}", name)
}
