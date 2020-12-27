// todo: finish tests

use crate::helpers::{graph::Graph, read};

fn into_graph(input: &str) -> Graph {
    let mut ret = Graph::new();
    let lines = input.lines();
    for line in lines {
        let mut iter = line.split_ascii_whitespace();
        let vertex0: String = iter.by_ref().take(2).collect();
        let mut inner = iter.skip(2).peekable();
        let mut vertices = Vec::<(u32, String)>::new();
        while inner.peek().is_some() {
            let weight = inner.next().unwrap();
            if weight == "no" {
                break;
            }
            let edge: (u32, String) = (
                weight.parse().expect("Error parsing an edge weight."),
                inner.by_ref().take(2).collect(),
            );
            vertices.push(edge);
            inner.next();
        }
        ret.add_edges(&vertex0, &vertices);
    }
    ret
}

fn count_bags_holding(graph: &Graph, holding: &str) -> usize {
    graph.list_ancestors(holding).iter().count()
}

fn count_bags_inside(graph: &Graph, inside_of: &str) -> u32 {
    graph.weigh_successors(inside_of)
}

//--------------------------------------------------------------------
// Solution
//--------------------------------------------------------------------

pub fn run() {
    println!("Day 07");
    let input = read::to_str("day07").expect("Error reading day07 input file");
    let graph = into_graph(&input);
    let ch = count_bags_holding(&graph, "shinygold");
    let ci = count_bags_inside(&graph, "shinygold");
    println!(
        "Number of bags containing at least one Shiny Gold bag: {}",
        ch
    );
    println!("Number of bags inside a Shiny Gold bag: {}", ci);
}

//--------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------

#[cfg(test)]
mod test {
    use crate::helpers::graph::Graph;

    use super::*;

    const BRIGHT_WHITE: &str = "brightwhite";
    const DARK_OLIVE: &str = "darkolive";
    const DARK_ORANGE: &str = "darkorange";
    const DOTTED_BLACK: &str = "dottedblack";
    const FADED_BLUE: &str = "fadedblue";
    const LIGHT_RED: &str = "lightred";
    const MUTED_YELLOW: &str = "mutedyellow";
    const SHINY_GOLD: &str = "shinygold";
    const VIBRANT_PLUM: &str = "vibrantplum";

    const INPUT_STR1: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    const INPUT_STR2: &str = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

    #[test]
    fn tests() {
        // Buildig and testing a graph from the instructions
        let mut graph = Graph::new();
        graph.add_edge(LIGHT_RED, BRIGHT_WHITE, 1);
        graph.add_edge(LIGHT_RED, MUTED_YELLOW, 2);
        graph.add_edge(DARK_ORANGE, BRIGHT_WHITE, 3);
        graph.add_edge(DARK_ORANGE, MUTED_YELLOW, 4);
        graph.add_edge(BRIGHT_WHITE, SHINY_GOLD, 1);
        graph.add_edge(MUTED_YELLOW, SHINY_GOLD, 2);
        graph.add_edge(MUTED_YELLOW, FADED_BLUE, 9);
        graph.add_edge(SHINY_GOLD, DARK_OLIVE, 1);
        graph.add_edge(SHINY_GOLD, VIBRANT_PLUM, 2);
        graph.add_edge(DARK_OLIVE, FADED_BLUE, 3);
        graph.add_edge(DARK_OLIVE, DOTTED_BLACK, 4);
        graph.add_edge(VIBRANT_PLUM, FADED_BLUE, 5);
        graph.add_edge(VIBRANT_PLUM, DOTTED_BLACK, 6);

        assert_eq!(count_bags_holding(&graph, SHINY_GOLD), 4);

        // Parsing a graph and testing it
        let parsed_graph1 = into_graph(INPUT_STR1);
        assert_eq!(count_bags_holding(&parsed_graph1, SHINY_GOLD), 4);

        let parsed_graph2 = into_graph(INPUT_STR2);
        assert_eq!(count_bags_inside(&parsed_graph2, SHINY_GOLD), 126);
    }
}
