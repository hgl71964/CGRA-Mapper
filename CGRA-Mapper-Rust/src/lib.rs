use cost::*;
use egg::*;
use rules::*;
use std::collections::{HashMap, HashSet};
use std::mem::size_of;
use std::os::raw::c_char;

mod cost;
mod rules;

// gh512
mod eg_env;
mod env;
mod node;
mod pool_manager;
mod run;
mod tree;
mod workers;
use crate::run::{run_mcts, MCTSArgs};
// use rmcts::run::{run_mcts, MCTSArgs};

#[repr(C)]
pub struct CppNode {
    op: *const c_char,

    num_children: u16,
    child_ids: *const u32,
}

#[repr(C)]
pub struct CppDFG {
    nodes: *const CppNode,
    count: u32,
}

#[repr(C)]
pub struct CppDFGs {
    dfgs: *const CppDFG,
    count: u32,
}

#[repr(C)]
pub struct Rulesets {
    names: *const *const c_char, // Name of each ruleset to use.
    num_rulesets: u32,           // How many rulesets?
}

fn dfg_nodes(dfg: &CppDFG) -> &[CppNode] {
    unsafe { std::slice::from_raw_parts(dfg.nodes, dfg.count as usize) }
}

fn node_op(node: &CppNode) -> &str {
    unsafe { std::ffi::CStr::from_ptr(node.op) }
        .to_str()
        .unwrap()
}

fn node_children(node: &CppNode) -> &[u32] {
    unsafe { std::slice::from_raw_parts(node.child_ids, node.num_children as usize) }
}

// NOTE: enables explanations
fn dfg_to_egraph(dfg: &CppDFG) -> (EGraph<SymbolLang, ()>, Vec<Id>) {
    let nodes = dfg_nodes(dfg);
    println!("DFG num_node {}", nodes.len());
    let mut egraph = EGraph::<SymbolLang, ()>::default().with_explanations_enabled();
    let mut eclasses = vec![];
    let mut is_root = (0..nodes.len()).map(|_| true).collect::<Vec<_>>(); // could use bit vector

    // DEBUG >>>
    // let test =
    // unsafe { std::slice::from_raw_parts(std::ptr::null_mut(), 0) }.iter()
    //	.map(|&c: &u32| eclasses[c as usize]).collect::<Vec<_>>();
    // println!("{:?}", test);
    // <<<
    for node in nodes {
        let op = Symbol::from(node_op(node));
        // DEBUG
        // println!("{}: {} with {} children", eclasses.len(), op, node.num_children);
        let children = node_children(node)
            .iter()
            .map(|&c| {
                // DEBUG
                // println!("access: {} / {}", c, eclasses.len());
                is_root[c as usize] = false;
                eclasses[c as usize]
            })
            .collect::<Vec<_>>();
        eclasses.push(egraph.add(SymbolLang::new(op, children)));
    }

    let mut roots = eclasses
        .iter()
        .cloned()
        .zip(is_root.iter().cloned())
        .filter_map(|(e, is_r)| if is_r { Some(e) } else { None })
        .collect::<Vec<_>>();
    // we remove duplicates, why is this necessary?
    roots.sort();
    roots.dedup();
    (egraph, roots)
}

fn dfg_to_egraph_single_root(dfg: &CppDFG) -> (EGraph<SymbolLang, ()>, Vec<Id>) {
    let (mut egraph, roots) = dfg_to_egraph(dfg);
    // ES formulation becomes easier if the graph is single-rooted,
    // thus add a Noop dummy sink node
    let sink = egraph.add(SymbolLang::new("__root", roots));
    (egraph, vec![sink])
}

fn dfg_to_graph(dfg: CppDFG) -> Graph<SymbolLang> {
    let nodes = dfg_nodes(&dfg);
    let mut graph: Graph<SymbolLang> = Default::default();
    let mut ids = Vec::with_capacity(nodes.len());
    let mut is_root = (0..nodes.len()).map(|_| true).collect::<Vec<_>>(); // could use bit vector
                                                                          //
    for node in nodes {
        let op = Symbol::from(node_op(node));
        // DEBUG
        // println!("{}: {} with {} children", ids.len(), op, node.num_children);
        let children = node_children(node)
            .iter()
            .map(|&c| {
                // DEBUG
                // println!("access: {} / {}", c, ids.len());
                is_root[c as usize] = false;
                ids[c as usize]
            })
            .collect::<Vec<_>>();
        ids.push(graph.add(SymbolLang::new(op, children)));
    }

    let mut roots = ids
        .iter()
        .cloned()
        .zip(is_root.iter().cloned())
        .filter_map(|(e, is_r)| if is_r { Some(e) } else { None })
        .collect::<Vec<_>>();
    // we remove duplicates, why is this necessary?
    roots.sort();
    roots.dedup();

    for r in roots {
        graph.add_root(r);
    }

    graph
}

fn load_rulesets(rsets: Rulesets) -> Vec<Rewrite<SymbolLang, ()>> {
    let mut result = vec![];
    let names = unsafe { std::slice::from_raw_parts(rsets.names, rsets.num_rulesets as usize) };
    for name in names {
        let rset_name = unsafe { std::ffi::CStr::from_ptr(*name) }.to_str().unwrap();
        println!("Loading ruleset {}", rset_name);
        result.extend(load_ruleset(rset_name).clone());
    }

    result
}

fn load_ruleset(nm: &str) -> Vec<Rewrite<SymbolLang, ()>> {
    match nm {
        "int" => rules(),             // These are the 'normal' rewrite rules
        "fp" => fp_rules(),           // These are rewrite rules for -ffast-math style rewrites
        "boolean" => boolean_logic(), // These are rewrite rules that assume ^&| are boolean rather than logical
        "stochastic" => stochastic(),
        // "cannonicalization" => gcc_style_rules(),  // Load rules as they are used in LLVM or GCC: to cannonicalize and simplify.
        _ => panic!("unknown ruleset"),
    }
}

fn add_dfg<L: Language, N: Analysis<L>>(dfg: &RecExpr<L>, egraph: &mut EGraph<L, N>) -> Vec<Id> {
    let nodes = dfg.as_ref();
    let mut eclasses = Vec::with_capacity(nodes.len());
    let mut is_root = (0..nodes.len()).map(|_| true).collect::<Vec<_>>(); // could use bit vector
    for node in nodes {
        eclasses.push(egraph.add(node.clone().map_children(|i| {
            is_root[usize::from(i)] = false;
            eclasses[usize::from(i)]
        })));
    }
    eclasses
        .iter()
        .cloned()
        .zip(is_root.iter().cloned())
        .filter_map(|(e, is_r)| if is_r { Some(e) } else { None })
        .collect::<Vec<_>>()
}

fn expr_to_dfg(expr: RecExpr<SymbolLang>) -> CppDFG {
    let enodes = expr.as_ref();
    let nodes_ptr = unsafe { libc::malloc(enodes.len() * size_of::<CppNode>()) } as *mut CppNode;
    let nodes = unsafe { std::slice::from_raw_parts_mut(nodes_ptr, enodes.len()) };
    for (en, n) in enodes.iter().zip(nodes.iter_mut()) {
        n.op = std::ffi::CString::new(en.op.as_str()).unwrap().into_raw();
        let child_ids = unsafe { libc::malloc(en.children.len() * size_of::<u32>()) } as *mut u32;
        for (child_id, &c) in
            unsafe { std::slice::from_raw_parts_mut(child_ids, en.children.len()) }
                .iter_mut()
                .zip(en.children.iter())
        {
            *child_id = usize::from(c).try_into().unwrap();
        }
        n.num_children = en.children.len().try_into().unwrap();
        n.child_ids = child_ids;
    }

    CppDFG {
        nodes: nodes_ptr,
        count: enodes.len().try_into().unwrap(),
    }
}

fn expr_to_dfg_single_root(expr: RecExpr<SymbolLang>) -> CppDFG {
    let enodes = expr.as_ref();
    let num_valid_enodes = enodes.len() - 1; // exclude the added root
    let nodes_ptr = unsafe { libc::malloc(num_valid_enodes * size_of::<CppNode>()) } as *mut CppNode;
    let nodes = unsafe { std::slice::from_raw_parts_mut(nodes_ptr, num_valid_enodes) };

    // ensure only 1 root
    let mut find = false;
    for i in 0..enodes.len() {
        let en = &enodes[i];
        if en.op.as_str() == "__root" {
            if find {
                panic!("2 roots?");
            }
            find = true;
            continue;
        }
    }
    assert!(find, "cannot find added root, forget to add root?");

    let mut index = 0;
    for i in 0..enodes.len() {
        let en = &enodes[i];
        if en.op.as_str() == "__root" {
            continue;
        }

        if index == enodes.len() - 1 {
            break;
        }
        let n = &mut nodes[index];
        index += 1;

        n.op = std::ffi::CString::new(en.op.as_str()).unwrap().into_raw();
        let child_ids = unsafe { libc::malloc(en.children.len() * size_of::<u32>()) } as *mut u32;
        for (child_id, &c) in
            unsafe { std::slice::from_raw_parts_mut(child_ids, en.children.len()) }
                .iter_mut()
                .zip(en.children.iter())
        {
            *child_id = usize::from(c).try_into().unwrap();
        }
        n.num_children = en.children.len().try_into().unwrap();
        n.child_ids = child_ids;
    }

    CppDFG {
        nodes: nodes_ptr,
        count: num_valid_enodes.try_into().unwrap(),
    }
}

fn dfg_to_rooted_expr(dfg: &CppDFG) -> RecExpr<SymbolLang> {
    let nodes = dfg_nodes(dfg);
    let mut expr: RecExpr<SymbolLang> = Default::default();
    let mut ids = vec![];
    let mut is_root = (0..nodes.len()).map(|_| true).collect::<Vec<_>>(); // could use bit vector

    for node in nodes {
        let op = Symbol::from(node_op(node));
        let children = node_children(node)
            .iter()
            .map(|&c| {
                is_root[c as usize] = false;
                ids[c as usize]
            })
            .collect::<Vec<_>>();
        ids.push(expr.add(SymbolLang::new(op, children)));
    }

    let mut roots = ids
        .iter()
        .cloned()
        .zip(is_root.iter().cloned())
        .filter_map(|(e, is_r)| if is_r { Some(e) } else { None })
        .collect::<Vec<_>>();

    // we remove duplicates, why is this necessary?
    roots.sort();
    roots.dedup();

    expr.add(SymbolLang::new("__root", roots));
    expr
}

fn rooted_expr(e: &RecExpr<SymbolLang>, roots: Vec<Id>) -> RecExpr<SymbolLang> {
    let mut rooted = e.clone();
    rooted.add(SymbolLang::new("__root", roots));
    rooted
}

fn explanation_statistics(e: &Explanation<SymbolLang>) {
    fn rec(
        t: &TreeExplanation<SymbolLang>,
        visited: &mut HashSet<*const TreeTerm<SymbolLang>>,
        applied_rules: &mut HashMap<Symbol, u16>,
    ) {
        for tt in t {
            let ttp = std::rc::Rc::as_ptr(tt);
            if visited.contains(&ttp) {
                continue;
            }
            visited.insert(ttp);

            let ropts = &[tt.backward_rule, tt.forward_rule];
            for &r in ropts.iter().flatten() {
                *(applied_rules.entry(r).or_insert(0)) += 1;
            }

            for cp in &tt.child_proofs {
                rec(cp, visited, applied_rules);
            }
        }
    }

    // println!("-- explanation:");
    // println!("{}", e.get_string_with_let());
    // println!("--");

    let mut applied_rules = HashMap::new();
    rec(
        &e.explanation_trees,
        &mut HashSet::new(),
        &mut applied_rules,
    );
    println!("applied {} rules: {:?}", applied_rules.len(), applied_rules);
    println!("done listing rules");
}

// #[no_mangle]
// pub extern "C" fn optimize_with_egraphs(
//     dfg: CppDFG,
//     rulesets: Rulesets,
//     cgra_params: *const c_char,
//     print_used_rules: bool,
//     cost_mode: *const c_char,
// ) -> CppDFGs {
//     let rules = load_rulesets(rulesets);
//     let expr = dfg_to_rooted_expr(&dfg);  // single rooted
//     let runner = Runner::default()
//         .with_iter_limit(15)
//         .with_node_limit(100_000)
//         .with_time_limit(std::time::Duration::from_secs(40))
//         .with_expr(&expr)
//         .with_scheduler(SimpleScheduler);
//     let root = runner.roots[0];
//     let cgrafilename = unsafe { std::ffi::CStr::from_ptr(cgra_params) }
//         .to_str()
//         .unwrap();
//     let cost_fn = GreedyBanCost::from_operations_file(cgrafilename);
//
//     // ILP
//     // let cost = BanCost::from_operations_file(cgrafilename);
//     // let mut extractor = LpExtractor::new(&runner.egraph, cost);
//     // extractor.timeout(30.0);
//     // let best = extractor.solve(root);
//     //
//     // GREEDY
//     let (base_cost, _) = Extractor::new(&runner.egraph, cost_fn.clone()).find_best(root);
//     let mut runner = runner.run(&rules);
//     let (best_cost, best) = Extractor::new(&runner.egraph, cost_fn).find_best(root);
//     println!("[EGG] base_cost {} -> best_cost {}", base_cost, best_cost);
//
//     let dfgs_ptr = unsafe { libc::malloc(size_of::<CppDFG>()) } as *mut CppDFG;
//     assert!(dfgs_ptr != std::ptr::null_mut());
//     unsafe { *dfgs_ptr = expr_to_dfg_single_root(best) };
//     // unsafe { *dfgs_ptr = expr_to_dfg(best) };
//     CppDFGs {
//         dfgs: dfgs_ptr,
//         count: 1,
//     }
// }

#[no_mangle]
pub extern "C" fn optimize_with_egraphs(
    dfg: CppDFG,
    rulesets: Rulesets,
    cgra_params: *const c_char,
    print_used_rules: bool,
    cost_mode: *const c_char,
) -> CppDFGs {
    println!("entering Rust");
    // env_logger::init();

    let rules = load_rulesets(rulesets);

    let (egraph, mut roots) = dfg_to_egraph(&dfg);
    println!("identified {} roots", roots.len());
    // egraph.dot().to_svg("/tmp/initial.svg").unwrap();

    let runner = Runner::default()
        .with_iter_limit(15)
        .with_node_limit(100_000)
        .with_time_limit(std::time::Duration::from_secs(40))
        .with_egraph(egraph)
        // .with_explanations_enabled()
        .with_scheduler(SimpleScheduler);
    let runner = if print_used_rules {
        runner.with_explanations_enabled()
    } else {
        runner
    };
    let mut runner = runner.run(&rules);
    runner.print_report();
    // runner.egraph.dot().to_svg("/tmp/egraph.svg").unwrap();

    for r in &mut roots[..] {
        *r = runner.egraph.find(*r);
    }

    let cgrafilename = unsafe { std::ffi::CStr::from_ptr(cgra_params) }
        .to_str()
        .unwrap();
    let cost_mode_string = unsafe { std::ffi::CStr::from_ptr(cost_mode) }
        .to_str()
        .unwrap();
    let start_extraction = std::time::Instant::now();
    let (best, best_roots) = if cost_mode_string == "frequency" {
        println!("Running egraphs with frequency cost");
        let cost = LookupCost::from_operations_frequencies(cgrafilename);
        LpExtractor::new(&runner.egraph, cost).solve_multiple(&roots[..])
    } else {
        println!("Running egraphs with ban cost");
        let cost = BanCost::from_operations_file(cgrafilename);
        let mut extractor = LpExtractor::new(&runner.egraph, cost);
        extractor.timeout(30.0);
        let (exp, ids) = extractor.solve_multiple(&roots[..]);
        (exp, ids)
    };
    let extraction_time = start_extraction.elapsed();
    println!("extraction took {:?}", extraction_time);

    println!("Explanation required: {:?}", print_used_rules);
    if print_used_rules {
        explanation_statistics(
            &runner.explain_equivalence(&dfg_to_rooted_expr(&dfg), &rooted_expr(&best, best_roots)),
        );
    };

    /* {
        let mut g: EGraph<SymbolLang, ()> = Default::default();
        g.add_expr(&best);
        g.dot().to_svg("/tmp/final.svg").unwrap();
    } */

    let dfgs_ptr = unsafe { libc::malloc(size_of::<CppDFG>()) } as *mut CppDFG;
    assert!(dfgs_ptr != std::ptr::null_mut());
    unsafe { *dfgs_ptr = expr_to_dfg(best) };

    CppDFGs {
        dfgs: dfgs_ptr,
        count: 1,
    }
}

#[no_mangle]
pub extern "C" fn optimize_with_mcts(
    dfg: CppDFG,
    rulesets: Rulesets,
    cgra_params: *const c_char,
    print_used_rules: bool,
    _cost_mode: *const c_char,
) -> CppDFGs {
    // build rules, egraph
    assert!(!print_used_rules);
    let rules = load_rulesets(rulesets);
    let n_threads = std::thread::available_parallelism().unwrap().get();
    println!("[RMCTS] Number of rules: {} - n_threads {}", rules.len(), n_threads);

    let expr = dfg_to_rooted_expr(&dfg);  // single rooted
    let runner = Runner::default().with_expr(&expr);
    let root = runner.roots[0];

    // build CostFn
    let cgrafilename = unsafe { std::ffi::CStr::from_ptr(cgra_params) }
        .to_str()
        .unwrap();
    let cost_fn = GreedyBanCost::from_operations_file(cgrafilename);

    // mcts-geb
    let cost_threshold = 10_000;
    let (mut best_cost, mut best) = Extractor::new(&runner.egraph, cost_fn.clone()).find_best(root);
    if best_cost < cost_threshold {
        println!("[RMCTS] expr OK without mcts -> cost {}", best_cost);
        // Use ILP to extract anyways?
        //
        // let (egraph, mut roots) = dfg_to_egraph(&dfg);
        // let runner = Runner::default()
        //     .with_iter_limit(15)
        //     .with_node_limit(100_000)
        //     .with_time_limit(std::time::Duration::from_secs(40))
        //     .with_egraph(egraph)
        //     .with_scheduler(SimpleScheduler)
        //     .run(&rules);
        // for r in &mut roots[..] {
        //     *r = runner.egraph.find(*r);
        // }
        // let cost = BanCost::from_operations_file(cgrafilename);
        // let mut extractor = LpExtractor::new(&runner.egraph, cost);
        // extractor.timeout(30.0);
        // let (best_expr, best_roots) = extractor.solve_multiple(&roots[..]);
        // best = best_expr;
    } else {
        let args = MCTSArgs {
            budget: 384,
            max_sim_step: 5,
            gamma: 0.90,
            expansion_worker_num: 1,
            cost_threshold: cost_threshold,
            iter_limit: 10,

            // simulation_worker_num: n_threads - 1,
            simulation_worker_num: 1,
            lp_extract: false,
            node_limit: 10_000,
            time_limit: 10,
        };
        let egraph = run_mcts(runner.egraph, root, rules, cost_fn.clone(), Some(args));

        // GREEDY
        (best_cost, best) = Extractor::new(&egraph, cost_fn).find_best(root);

        // ILP
        // let cost = BanCost::from_operations_file(cgrafilename);
        // let mut extractor = LpExtractor::new(&egraph, cost);
        // extractor.timeout(30.0);
        // best = extractor.solve(root);
    }

    // to cpp
    let dfgs_ptr = unsafe { libc::malloc(size_of::<CppDFG>()) } as *mut CppDFG;
    assert!(dfgs_ptr != std::ptr::null_mut());
    unsafe { *dfgs_ptr = expr_to_dfg_single_root(best) };  // will filter out the added root

    CppDFGs {
        dfgs: dfgs_ptr,
        count: 1,
    }
}

#[no_mangle]
pub extern "C" fn optimize_with_graphs(
    dfg: CppDFG,
    rulesets: Rulesets,
    cgra_params: *const c_char,
    frequency_cost: bool,
    print_used_rules: bool,
) -> CppDFGs {
    println!("entering Rust, using standard rewriting");

    let rules = load_rulesets(rulesets);
    let mut graph = dfg_to_graph(dfg);
    println!("Number of rules: {}", rules.len());
    // TODO:
    // println!("identified {} roots", graph.roots.len());
    // graph.to_svg("/tmp/initial.svg").unwrap();

    let cgrafilename = unsafe { std::ffi::CStr::from_ptr(cgra_params) }
        .to_str()
        .unwrap();
    let (freq_cost, ban_cost);
    let cost: Box<dyn Fn(&Graph<SymbolLang>) -> f64> = if frequency_cost {
        println!("Running with frequency cost");
        freq_cost = LookupCost::from_operations_frequencies(cgrafilename);
        Box::new(|g| g.cost(&freq_cost))
    } else {
        println!("Running with ban cost");
        ban_cost = BanCost::from_operations_file(cgrafilename);
        Box::new(|g| g.cost(&ban_cost))
    };

    let start_rewriting = std::time::Instant::now();

    let mut local_minima = false;
    let mut applied = Vec::new();
    while !local_minima {
        local_minima = true;

        for r in &rules {
            // println!("trying {}", r.name);
            let lhs = r.searcher.get_pattern().unwrap();
            let rhs = r.applier.get_pattern_ast().unwrap();

            // while let Some((id, subst)) = lhs.search_graph(&graph)
            let current_cost = cost(&graph);
            if let Some(mut improved) = lhs.search_graph_until(&graph, |id, subst| {
                let mut candidate = graph.clone();
                candidate.replace(id, &rhs, &subst);

                // TODO: computing costs could be cheaper,
                // and maybe it can be predicted before actually replacing stuff,
                // to avoid copy
                let should_rewrite = cost(&candidate) < current_cost;
                if should_rewrite {
                    Some(candidate)
                } else {
                    None
                }
            }) {
                // println!("reducing cost to {}", improved.cost(&cost));
                std::mem::swap(&mut graph, &mut improved);
                applied.push(r.name);
                local_minima = false;
            }
        }
    }

    let rewriting_time = start_rewriting.elapsed();
    // graph.to_svg("/tmp/final.svg").unwrap();
    println!("[GREEDY]applied {} rules in {:?}:", applied.len(), rewriting_time);
    println!("[GREEDY]  {:?}", applied);

    let best = graph.as_dfg();

    let dfgs_ptr = unsafe { libc::malloc(size_of::<CppDFG>()) } as *mut CppDFG;
    assert!(dfgs_ptr != std::ptr::null_mut());
    unsafe { *dfgs_ptr = expr_to_dfg(best) };

    CppDFGs {
        dfgs: dfgs_ptr,
        count: 1,
    }
}
