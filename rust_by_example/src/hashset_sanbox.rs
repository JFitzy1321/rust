use uuid::Uuid;

#[derive(Hash, Debug, PartialEq, Eq)]
pub struct Org {
    id: Uuid,
    hierarchy: String,
}

// impl PartialEq for Org {
//     // captial Self means same type as little self
//     fn eq(&self, other: &Self) -> bool {
//         self.id.eq(&other.id) && self.hierarchy.eq(&other.hierarchy)
//     }
// }

pub fn main() {
    use std::collections::HashSet;

    let same_org = Org {
        id: Uuid::new_v4(),
        hierarchy: "1/2/3".to_string(),
    };

    let a_set: HashSet<Org> = vec![
        Org {
            id: same_org.id,
            hierarchy: same_org.hierarchy.clone(),
        },
        Org {
            id: Uuid::new_v4(),
            hierarchy: same_org.hierarchy.clone(),
        },
    ]
    .into_iter()
    .collect();

    let b_set: HashSet<Org> = vec![
        Org {
            id: Uuid::new_v4(),
            hierarchy: "something different".to_string(),
        },
        Org {
            id: same_org.id,
            hierarchy: same_org.hierarchy.clone(),
        },
    ]
    .into_iter()
    .collect();

    let union_set: HashSet<_> = a_set.union(&b_set).collect();

    println!(
        "Union of Org HashSets -> Count: {:?}, Items: {:#?}",
        union_set.len(),
        &union_set
    );
}
