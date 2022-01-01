// Copyright 2020 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Find the shortest path in a graph, using Djikstra's method.

use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::Add;

use crate::MinHeap;

/// Find the shortest path in a graph, using Djikstra's method.
///
/// Type `P` identifies a Point or state in the graph.
///
/// Type `D` is the distance between points: it may be an `isize`,
/// `usize`, `f64`, etc.
///
/// `origin` is the starting point.
///
/// `dest_fn` returns true for the destination point.
///
/// `nbr_fn` returns a `Vec` of neighbors for a given point, and the
/// incremental distance to them.
pub fn shortest_distance<P, N, DF, D>(origin: P, dest_fn: DF, nbr_fn: N) -> D
where
    P: Eq + Ord + Copy + Hash + Debug,
    D: Ord + Add<Output = D> + Clone + Default + Debug,
    N: Fn(P) -> Vec<(P, D)>,
    DF: Fn(&P) -> bool,
{
    // Next points to visit, indexed by distance so far.
    let mut queue = MinHeap::<(D, P)>::new();
    // Shortest known distance to reach any point.
    let mut best = HashMap::<P, D>::new();
    // The previous state that leads, on the best path, to this state.
    let mut path = std::collections::BTreeMap::<P, P>::new();
    queue.push((Default::default(), origin));
    best.insert(origin, Default::default());
    loop {
        let (d, p) = queue
            .pop()
            .expect("heap is empty without reaching destination");
        if dest_fn(&p) {
            // Found a shortest path to the end
            let mut fwd = vec![p];
            let mut pred = p;
            while let Some(next) = path.get(&pred) {
                fwd.push(*next);
                pred = *next;
            }
            fwd.reverse();
            // for i in &fwd {
            //     println!("cost={:?}\n{}", best[i], i);
            // }
            return d;
        }
        for (np, step) in nbr_fn(p) {
            let nd = step + d.clone();
            if let Some(prev_d) = best.get(&np) {
                if nd >= *prev_d {
                    continue; // Already found a shorter path; don't revisit.
                }
            }
            best.insert(np, nd.clone());
            queue.push((nd, np));
            path.insert(np, p);
        }
    }
}
