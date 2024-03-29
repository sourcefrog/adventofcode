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

/// A discovered shortest path in a space.
///
/// Type `P` identifies a Point or state in the graph.
///
/// Type `D` is the distance between points: it may be an `isize`,
/// `usize`, `f64`, etc.
pub struct ShortestPath<P, D>
where
    P: Eq + Ord + Clone + Hash + Debug,
    D: Ord + Add<Output = D> + Clone + Default + Debug,
{
    distance: D,
    path: Vec<P>,
    stats: Stats,
}

/// Statistics about the work done to find the path.
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Stats {
    pub search_cycles: usize,
}

impl<P, D> ShortestPath<P, D>
where
    P: Eq + Ord + Clone + Hash + Debug,
    D: Ord + Add<Output = D> + Clone + Default + Debug,
{
    /// Find the shortest path using
    /// [A* search](https://en.wikipedia.org/wiki/A*_search_algorithm).
    ///
    /// `origin` is the starting point.
    ///
    /// `estimate` returns a *lower bound* on the distance between a candidate
    /// point `p` and the destination. This must be `0` (more precisely, `D::default()`)
    /// for the destination, and greater than `0` for every other point.
    pub fn find_astar<NbrFn, Nbrs, EstFn>(
        origin: &P,
        estimate: EstFn,
        neighbors: NbrFn,
    ) -> Option<Self>
    where
        NbrFn: Fn(&P) -> Nbrs,
        Nbrs: IntoIterator<Item = (P, D)>,
        EstFn: Fn(&P) -> D,
    {
        // Next points to visit, indexed by distance so far.
        let mut queue = MinHeap::<(D, P)>::new();
        // Shortest known distance to reach any point.
        let mut best = HashMap::<P, D>::new();
        // The previous state that leads, on the best path, to this state.
        let mut predecessor = HashMap::<P, P>::new();
        let mut search_cycles = 0;
        queue.push((Default::default(), origin.clone()));
        best.insert(origin.clone(), Default::default());
        loop {
            let (_priority, p) = queue
                .pop()
                .expect("heap is empty without reaching destination");
            search_cycles += 1;
            if search_cycles % 100000 == 0 {
                dbg!(&p);
                queue.assert_valid();
            }
            let est = estimate(&p);
            if est == D::default() {
                // Reassemble (a) shortest path to the destination by looking backwards
                // at the step that led to each point.
                let distance = best[&p].clone();
                let mut path = vec![p];
                while let Some(next) = predecessor.get(path.last().unwrap()) {
                    path.push(next.clone());
                }
                path.reverse();
                return Some(ShortestPath {
                    distance,
                    path,
                    stats: Stats { search_cycles },
                });
            }
            debug_assert!(
                est > D::default(),
                "estimate {:?} of {:?} is negative",
                est,
                &p
            );
            for (np, step) in neighbors(&p) {
                let nd = step + best[&p].clone();
                if let Some(prev_d) = best.get(&np) {
                    if nd >= *prev_d {
                        continue; // Already found a shorter path; don't revisit.
                    }
                }
                best.insert(np.clone(), nd.clone());
                queue.push((nd + est.clone(), np.clone()));
                predecessor.insert(np, p.clone());
            }
        }
    }

    /// Find the shortest path in a graph, using Djikstra's method.
    ///
    /// `origin` is the starting point.
    ///
    /// `is_destination` returns true for the destination point.
    ///
    /// `neighbors` returns a `Vec` of neighbors for a given point, and the
    /// incremental distance to them.
    pub fn find<NbrFn, Nbrs, DestFn>(
        origin: &P,
        is_destination: DestFn,
        neighbors: NbrFn,
    ) -> Option<Self>
    where
        NbrFn: Fn(&P) -> Nbrs,
        Nbrs: IntoIterator<Item = (P, D)>,
        DestFn: Fn(&P) -> bool,
    {
        // Next points to visit, indexed by distance so far.
        let mut queue = MinHeap::<(D, P)>::new();
        // Shortest known distance to reach any point.
        let mut best = HashMap::<P, D>::new();
        // The previous state that leads, on the best path, to this state.
        let mut predecessor = HashMap::<P, P>::new();
        queue.push((Default::default(), origin.clone()));
        best.insert(origin.clone(), Default::default());
        let mut search_cycles = 0;
        loop {
            search_cycles += 1;
            if let Some((d, p)) = queue.pop() {
                if search_cycles % 100000 == 0 {
                    println!("best distance={d:?} p={p:?}");
                    // dbg!(&d, &p);
                    queue.assert_valid();
                }
                if is_destination(&p) {
                    // Reassemble (a) shortest path to the destination by looking backwards
                    // at the step that led to each point.
                    let mut path = vec![p];
                    while let Some(next) = predecessor.get(path.last().unwrap()) {
                        path.push(next.clone());
                    }
                    path.reverse();
                    println!(
                        "shortest_path: destination found at distance {d:?} {search_cycles} search cycles, {} states examined",
                        best.len()
                    );
                    return Some(ShortestPath {
                        distance: d,
                        path,
                        stats: Stats { search_cycles },
                    });
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
                    predecessor.insert(np, p.clone());
                }
            } else {
                return None;
            }
        }
    }

    /// Return the total distance along the shortest path.
    pub fn distance(&self) -> D {
        self.distance.clone()
    }

    /// Return the points visited in order on the shortest path from the origin to
    /// the destination.
    pub fn path(&self) -> impl Iterator<Item = &P> {
        self.path.iter()
    }

    /// Return the final end point.
    ///
    /// This might be useful if there are multiple possible end points, or if the state includes
    /// other information such as a clock.
    pub fn final_point(&self) -> &P {
        self.path.last().expect("path is not empty")
    }

    pub fn stats(&self) -> &Stats {
        &self.stats
    }
}
