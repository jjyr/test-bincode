use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Dep {
    #[serde(default)]
    cell_dep: Option<CellDep>,
    #[serde(default)]
    type_id: Option<TypeId>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct CellDep {
    out_point: String,
    dep_type: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct TypeId {
    id: String,
}

fn main() {
    let cell_dep = CellDep {
        out_point: "out_point1".to_string(),
        dep_type: "type".to_string(),
    };
    let dep = Dep {
        cell_dep: Some(cell_dep),
        type_id: None,
    };
    let r = bincode::serialize(&dep).unwrap();
    println!("{:#?}", r);
    let dep2: Dep = bincode::deserialize(&r).unwrap();
    println!("{dep2:?}");
}
