use std::cmp::min;
const DATA:  &str = "\
More generally, permissions tuples are defined on paths and not just variables. 
A path is anything you can put on the left-hand side of an assignment. 
Paths include: Variables, like a.
Dereferences of paths, like *a.
Array accesses of paths, like a[0].
Fields of paths, like a.0 for tuples or a.field for structs (discussed next chapter).
Any combination of the above, like *((*a)[0].1).
tuples ";

const NEEDLE: &str = "tuples";
const CTX_LINES: usize = 2;

fn main() {
    let search_space: String = DATA.into();
    let mut ctx: Vec<&[(usize, &str)]> = vec!{};
    let search_vec = search_space.lines().enumerate().collect::<Vec<(usize, &str)>>();
    let num_lines = search_vec.len();

    for (i, line) in search_vec.iter() {
        if line.contains(NEEDLE) {
            ctx.push(
                &search_vec[i.saturating_sub(CTX_LINES)..min(i.saturating_add(CTX_LINES)+1, num_lines)]
            );
        }
    }
    for ls in ctx.iter() {
        for (i, l) in ls.iter() {
            println!("{i}: {l}");
        }
    }
}
