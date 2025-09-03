use crate::routes::Route;

pub fn permutations(sequence: Route) -> Vec<Vec<Route>> {
    if sequence.is_empty() {
        let x: Vec<Vec<Route>> = vec![vec![]];
        return x;
    }

    let mut result: Vec<Vec<Route>> = Vec::new();

    for (i, item) in permutations(sequence[0..sequence.len() - 1].into())
        .iter()
        .enumerate()
    {
        let mut n = (0..item.len() + 1).collect::<Vec<usize>>();
        if i % 2 == 0 {
            n.reverse();
        }

        for k in n {
            let mut items = Vec::new();

            for x in &item[0..k] {
                items.push(*x);
            }

            items.push(item[item.len() - 1]);

            for x in &item[k..item.len()] {
                items.push(*x);
            }

            result.push(items);
        }
    }

    result
}
