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
/// `is_destination` returns true for the destination point.
///
/// `neighbors` returns a `Vec` of neighbors for a given point, and the
/// incremental distance to them.
pub fn shortest_distance<P, N, DF, D>(origin: &P, is_destination: DF, neighbors: N) -> D
where
    P: Eq + Ord + Clone + Hash + Debug,
    D: Ord + Add<Output = D> + Clone + Default + Debug,
    N: Fn(&P) -> Vec<(P, D)>,
    DF: Fn(&P) -> bool,
{
    // Next points to visit, indexed by distance so far.
    let mut queue = MinHeap::<(D, P)>::new();
    // Shortest known distance to reach any point.
    let mut best = HashMap::<P, D>::new();
    // The previous state that leads, on the best path, to this state.
    let mut path = std::collections::BTreeMap::<P, P>::new();
    queue.push((Default::default(), origin.clone()));
    best.insert(origin.clone(), Default::default());
    loop {
        let (d, p) = queue
            .pop()
            .expect("heap is empty without reaching destination");
        if is_destination(&p) {
            // Found a shortest path to the end
            let mut forward_path = vec![p];
            while let Some(next) = path.get(forward_path.last().unwrap()) {
                forward_path.push(next.clone());
            }
            forward_path.reverse();
            return d;
        }
        for (np, step) in neighbors(&p) {
            let nd = step + d.clone();
            if let Some(prev_d) = best.get(&np) {
                if nd >= *prev_d {
                    continue; // Already found a shorter path; don't revisit.
                }
            }
            best.insert(np.clone(), nd.clone());
            queue.push((nd, np.clone()));
            path.insert(np, p.clone());
        }
    }
}
