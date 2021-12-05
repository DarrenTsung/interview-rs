use binary_heap_plus::*;

/*
At an airport you have a timetable for arrivals and departures.

You need to determine the minimum number of gates you'd need to provide
so that all the planes can be placed at a gate as per their schedule.
The arrival and departure times for each plane are presented in two arrays,
sorted by arrival time, and you're told the total number of flights for the
day. Assume that no planes remain overnight at the airport; all fly in and
back out on the same day. Assume that if a plane departs in the same minute
as another plane arrives, the arriving plane takes priority (i.e. you'll
still need the gate for the departing plane). Write a function that returns
the minimum number of gates needed for the schedules you're given.

Example:
arrQ = {900, 940, 950,1100,1500,1800}
depQ = {910,1200,1120,1130,1900,2000}
flights = 6
*/
#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
pub struct Schedule {
    start: u32,
    end: u32,
}

impl From<(u32, u32)> for Schedule {
    fn from(v: (u32, u32)) -> Self {
        Self {
            start: v.0,
            end: v.1,
        }
    }
}

pub trait AirportGatesSolution {
    fn airport_gates(&self, schedules_sorted_by_arrival: &[(u32, u32)]) -> u32;
}

/// O(N^2) runtime because search through gates is naive.
pub struct AirportGatesSolutionNaive;

impl AirportGatesSolution for AirportGatesSolutionNaive {
    fn airport_gates(&self, schedules_sorted_by_arrival: &[(u32, u32)]) -> u32 {
        let mut gates: Vec<Schedule> = vec![];
        for schedule in schedules_sorted_by_arrival {
            let schedule: Schedule = (*schedule).into();

            // See if any gate can accommodate the flight schedule.
            let mut found_gate = false;
            for gate in &mut gates {
                if gate.end < schedule.start {
                    gate.end = schedule.end;
                    found_gate = true;
                    break;
                }
            }
            if found_gate {
                continue;
            }

            // Otherwise, create a new gate for the schedule.
            gates.push(schedule);
        }

        gates.len() as u32
    }
}

/// O(NlogN) runtime because we store the available gates in a min-heap, which can be queried for the
/// min with O(1) runtime and replacing is O(logN).
pub struct AirportGatesSolutionMoreEfficient;

impl AirportGatesSolution for AirportGatesSolutionMoreEfficient {
    fn airport_gates(&self, schedules_sorted_by_arrival: &[(u32, u32)]) -> u32 {
        // Min-heap.
        let mut gate_end_times: BinaryHeap<u32, MinComparator> = BinaryHeap::from_vec(vec![]);

        for schedule in schedules_sorted_by_arrival {
            let schedule: Schedule = (*schedule).into();

            // See if any gate can accommodate the flight schedule.
            // We only need to check the min end time because if it can't
            // accommodate the schedule, then none of them can.
            if let Some(mut min_end_time) = gate_end_times.peek_mut() {
                if *min_end_time < schedule.start {
                    *min_end_time = schedule.end;
                    continue;
                }
            }

            // Otherwise, create a new gate for the schedule.
            gate_end_times.push(schedule.end);
        }

        gate_end_times.len() as u32
    }
}

/// O(NlogN) runtime because we sort O(2*N) items.
pub struct AirportGatesSolutionCounter;

impl AirportGatesSolution for AirportGatesSolutionCounter {
    fn airport_gates(&self, schedules_sorted_by_arrival: &[(u32, u32)]) -> u32 {
        #[derive(PartialEq, Eq, PartialOrd, Ord)]
        enum EventType {
            Arrival,
            Departure,
        }

        let sorted_events = {
            let mut events = vec![];
            for (arrival, departure) in schedules_sorted_by_arrival {
                events.push((arrival, EventType::Arrival));
                events.push((departure, EventType::Departure));
            }
            // Sort by time, then by EventType. Note that it's important
            // for Arrivals to come before Departures, otherwise the
            // same gate could be re-used for the same-time case.
            events.sort();
            events
        };

        let mut max_gate_count = 0;
        let mut gate_count = 0;
        for (_time, event_type) in sorted_events {
            match event_type {
                EventType::Arrival => gate_count += 1,
                EventType::Departure => gate_count -= 1,
            }

            max_gate_count = std::cmp::max(gate_count, max_gate_count);
        }

        max_gate_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_correctness_for_all_solutions(assertions: impl Fn(&dyn AirportGatesSolution)) {
        assertions(&AirportGatesSolutionNaive);
        assertions(&AirportGatesSolutionMoreEfficient);
        assertions(&AirportGatesSolutionCounter);
    }

    #[test]
    fn trivial() {
        check_correctness_for_all_solutions(|s| {
            assert_eq!(s.airport_gates(&[(0, 10)]), 1);
            assert_eq!(s.airport_gates(&[(0, 10), (11, 30)]), 1);
        })
    }

    #[test]
    fn arriving_at_same_time_as_departing() {
        check_correctness_for_all_solutions(|s| {
            // Need two gates because plane is leaving at same time plane is arriving.
            assert_eq!(s.airport_gates(&[(0, 10), (10, 20)]), 2);
        })
    }

    #[test]
    fn example() {
        check_correctness_for_all_solutions(|s| {
            assert_eq!(
                s.airport_gates(&[
                    (900, 910),
                    (940, 1200),
                    (950, 1120),
                    (1100, 1130),
                    (1500, 1900),
                    (1800, 2000),
                ]),
                3
            );
        })
    }
}
