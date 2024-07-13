use std::{env::args, fs::write, iter::IntoIterator};

use image::{imageops::FilterType, io::Reader, GenericImageView, Rgba};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Expression {
    r#type: String,
    id: String,
    color: Option<String>,
    latex: Option<String>,
    color_latex: Option<String>,
    fill_opacity: Option<String>,
    columns: Option<Vec<Column>>,
    point_size: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Column {
    values: Vec<String>,
    hidden: Option<bool>,
    id: String,
    color: String,
    latex: String,
}

impl Expression {
    fn new_table(id: usize, columns: Vec<(Vec<String>, usize, String)>) -> Self {
        Self {
            r#type: "table".to_string(),
            id: id.to_string(),
            color: None,
            latex: None,
            color_latex: None,
            fill_opacity: None,
            columns: Some(
                columns
                    .into_iter()
                    .map(|(values, id, latex)| Column {
                        values,
                        hidden: Some(true),
                        id: id.to_string(),
                        color: "#000000".to_string(),
                        latex,
                    })
                    .collect_vec(),
            ),
            point_size: None,
        }
    }
}

fn main() {
    let template = include_str!("template.js");
    let image = Reader::open(args().nth(1).unwrap())
        .unwrap()
        .decode()
        .unwrap()
        .resize(
            args().nth(2).unwrap().parse().unwrap(),
            args().nth(2).unwrap().parse().unwrap(),
            FilterType::Triangle,
        );
    let tables = image
        .pixels()
        .chunks(10_000)
        .into_iter()
        .map(|chunk| {
            let columns = chunk
                .map(|(x, y, Rgba([r, g, b, _]))| {
                    [
                        x.to_string(),
                        y.to_string(),
                        (u32::from(r) * 0x10000 + u32::from(g) * 0x100 + u32::from(b)).to_string(),
                    ]
                })
                .collect_vec();
            let len = columns.first().unwrap().len();
            let mut iters = columns
                .into_iter()
                .map(IntoIterator::into_iter)
                .collect_vec();
            (0..len)
                .map(move |_| {
                    iters
                        .iter_mut()
                        .map(|n| n.next().unwrap())
                        .collect::<Vec<String>>()
                })
                .collect_vec()
        })
        .collect_vec();
    let mut next_index = 2;
    let mut id = || {
        let next = next_index;
        next_index += 1;
        next
    };
    let mut expressions = Vec::new();
    for (index, table) in tables.into_iter().enumerate() {
        let expression = Expression::new_table(
            id(),
            table
                .into_iter()
                .zip_eq([
                    format!("x_{{{index}}}"),
                    format!("y_{{{index}}}"),
                    format!("h_{{{index}}}"),
                ])
                .map(|(column, header)| (column, id(), header))
                .collect_vec(),
        );
        expressions.push(expression);
        expressions.push(Expression {
            r#type: "expression".to_string(),
            id: id().to_string(),
            color: Some("#000000".to_string()),
            latex: Some(format!("\\left(x_{{{index}}},-y_{{{index}}}\\right)")),
            color_latex: Some(format!("c_{{{index}}}")),
            point_size: Some("4".to_string()),
            fill_opacity: None,
            columns: None,
        });
        expressions.push(Expression {
            r#type: "expression".to_string(),
            id: id().to_string(),
            color: Some("#000000".to_string()),
            latex: Some(format!("c_{{{index}}}=\\operatorname{{rgb}}\\left(\\operatorname{{mod}}\\left(\\operatorname{{floor}}\\left(\\frac{{h_{{{index}}}}}{{65536}}\\right),256\\right),\\operatorname{{mod}}\\left(\\operatorname{{floor}}\\left(\\frac{{{{h_{{{index}}}}}}}{{256}}\\right),256\\right),\\operatorname{{mod}}\\left({{h_{{{index}}}}},256\\right)\\right)")),
            color_latex: None,
            point_size: None,
            fill_opacity: None,
            columns: None,
        });
    }

    write(
        "out",
        template.replace("INSERT", &to_string(&expressions).unwrap()),
    )
    .unwrap();
}
