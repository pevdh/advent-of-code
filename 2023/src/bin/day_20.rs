use core::panic;
use std::collections::{HashMap, HashSet, BTreeMap};

use aoc2023::*;

aoc_main!(
    day: 20,
    test_input: r#"
    broadcaster -> a, b, c
    %a -> b
    %b -> c
    %c -> inv
    &inv -> a"#,
    task_1: task_1,
    expected_1: 32000000,
    task_2: task_2,
    expected_2: 167409079868000,
);

fn task_1(input: &str) -> Result<u64> {
    let config = ModuleConfiguration::from_input(input);
    let mut state = ModuleState::default();

    for _ in 0..1000 {
        config.simulate_button_press(&mut state);
    }

    Ok(state.low_pulses * state.high_pulses)
}

fn task_2(_input: &str) -> Result<u64> {
    let input: &str = include_str!(concat!("../../input/day_20.txt"));

    let config = ModuleConfiguration::from_input(input);

    config.button_presses_until_rx_receives_low_pulse()
}

struct ModuleConfiguration<'a> {
    output_modules: HashMap<&'a str, Vec<&'a str>>,
    input_modules: HashMap<&'a str, Vec<&'a str>>,
    module_types: HashMap<&'a str, ModuleType>,
}

#[derive(Default, Clone, PartialEq, Eq, Hash)]
struct ModuleState<'a> {
    flipflops: BTreeMap<&'a str, bool>,
    conjunctions: BTreeMap<&'a str, BTreeMap<&'a str, Pulse>>,
    low_pulses: u64,
    high_pulses: u64,
}

impl<'a> ModuleConfiguration<'a> {
    fn from_input(input: &'a str) -> ModuleConfiguration<'a> {
        let mut output_modules: HashMap<&str, Vec<&str>> = HashMap::default();
        let mut input_modules: HashMap<&str, Vec<&str>> = HashMap::default();

        let mut module_types: HashMap<&str, ModuleType> = HashMap::default();

        for line in input.lines() {
            let (mut label, rest) = line.split_once(" -> ").unwrap();

            if label == "broadcaster" {
                module_types.insert("broadcaster", ModuleType::Broadcaster);
            }

            if label.starts_with("%") {
                label = &label[1..];
                module_types.insert(label, ModuleType::FlipFlop);
            }

            if label.starts_with("&") {
                label = &label[1..];
                module_types.insert(label, ModuleType::Conjunction);
            }

            let out: Vec<&str> = rest.split(", ").collect();

            for output_module in &out {
                input_modules
                    .entry(output_module)
                    .or_insert(vec![])
                    .push(label);
            }

            output_modules.insert(label, out);
        }

        ModuleConfiguration {
            output_modules,
            input_modules,
            module_types,
        }
    }

    fn simulate_button_press(&'a self, module_state: &mut ModuleState<'a>) -> Result<()> {
        let mut pulses: VecDeque<(&str, Pulse, &str)> = VecDeque::new();
        pulses.push_back(("button", Pulse::Low, "broadcaster"));

        while let Some((source, pulse, current)) = pulses.pop_front() {
            match pulse {
                Pulse::Low => module_state.low_pulses += 1,
                Pulse::High => module_state.high_pulses += 1,
            }

            let current_ty = match self.module_types.get(current) {
                Some(current_ty) => current_ty,
                None => continue,
            };

            match current_ty {
                ModuleType::Broadcaster => {
                    let targets = self
                        .output_modules
                        .get(current)
                        .ok_or(eyre!("module {} not found", current))?;

                    pulses.extend(targets.iter().map(|&t| (current, pulse, t)));
                }
                ModuleType::FlipFlop => {
                    let flipflop_state = module_state.flipflops.entry(current).or_insert(false);

                    if pulse == Pulse::High {
                        continue;
                    }

                    if pulse == Pulse::Low {
                        *flipflop_state = !*flipflop_state;
                    }

                    let output = if *flipflop_state { Pulse::High} else {Pulse::Low };

                    let targets = self
                        .output_modules
                        .get(current)
                        .ok_or(eyre!("module {} not found", current))?;

                    pulses.extend(targets.iter().map(|&t| (current, output, t)));
                }
                ModuleType::Conjunction => {
                    let conjunction_state = module_state
                        .conjunctions
                        .entry(current)
                        .or_insert_with(|| BTreeMap::default());

                    conjunction_state.insert(source, pulse);

                    let all_high = self
                        .input_modules
                        .get(current)
                        .ok_or(eyre!("module {} not found", current))?
                        .iter()
                        .all(|&input_module| {
                            *conjunction_state.get(input_module).unwrap_or(&Pulse::Low)
                                == Pulse::High
                        });

                    let output = if all_high { Pulse::Low } else { Pulse::High };

                    let targets = self
                        .output_modules
                        .get(current)
                        .ok_or(eyre!("module {} not found", current))?;

                    pulses.extend(targets.iter().map(|&t| (current, output, t)));
                }
            }
        }
        Ok(())
    }

    fn count_cycles_until_input(&self, module: &str, pulse: Pulse) -> Result<u64> {
        // count number of cycles required for module to receive pulse
        let module_ty = self.module_types.get(module);

        match module_ty {
            Some(module_ty) => todo!(),
            None => {
                let input = self.input_modules.get(module).unwrap();
                assert!(input.len() == 1);

                let input = input[0];

                self.count_cycles_until_input(input, pulse)
            }
        }
    }

    fn count_cycles_until_output(&self, module: &str, pulse: Pulse) -> Result<u64> {
        // count number of cycles required for module to emit pulse

        let module_ty = self.module_types.get(module).ok_or(eyre!(
            "module {} has no type and therefore does not output a pulse",
            module
        ))?;

        match module_ty {
            ModuleType::Broadcaster => {
                let input = self.input_modules.get(module).unwrap();
                assert!(input.len() == 1);

                let input = input[0];

                self.count_cycles_until_output(input, pulse)
            }
            ModuleType::FlipFlop => {
                // Flip-flop modules (prefix %) are either on or off; they are initially off.
                // If a flip-flop module receives a high pulse, it is ignored and nothing happens. However,
                // if a flip-flop module receives a low pulse, it flips between on and off. If it was off,
                // it turns on and sends a high pulse. If it was on, it turns off and sends a low pulse.

                let inputs = self.input_modules.get(module).unwrap();

                todo!()
            }
            ModuleType::Conjunction => todo!(),
        }
    }

    fn button_presses_until_rx_receives_low_pulse(&self) -> Result<u64> {
        let inputs = self.input_modules.get("rx").unwrap();

        let mut button_presses = 1;
        for input in inputs {
            let cycle = self.count_conjunction_cycle(input)?;

            button_presses *= cycle;
        }

        Ok(button_presses)
    }

    fn count_conjunction_cycle(&'a self, conjuction: &'a str) -> Result<u64> {
        let mut visited_states : HashSet<ModuleState> = HashSet::default();

        let mut state = ModuleState::default();

        let inputs = self.find_all_direct_and_indirect_inputs(conjuction);

        for button_press in 1.. {
            dbg!(button_press);
            self.simulate_button_press(&mut state)?;

            let mut substate = ModuleState::default();
            
            for inp in inputs.iter() {
                let ty = self.module_types.get(inp).unwrap();

                match ty {
                    ModuleType::Broadcaster => continue,
                    ModuleType::FlipFlop => {
                        let state = state.flipflops.get(inp).unwrap_or(&false);

                        substate.flipflops.insert(inp, *state);
                    },
                    ModuleType::Conjunction => {
                        let new_bmap = BTreeMap::default();
                        let state = state.conjunctions.get(inp).unwrap_or(&new_bmap);

                        let conj_state: BTreeMap<&str, Pulse> = self.input_modules.get(inp).unwrap()
                            .iter()
                            .map(|&i| (i, *state.get(i).unwrap_or(&Pulse::Low)))
                            .collect();

                        substate.conjunctions.insert(inp, conj_state);
                    },
                }
            }

            if visited_states.contains(&substate) {
                eprintln!("found cycle for {}: {}", conjuction, button_press);
                return Ok(button_press);
            }

            visited_states.insert(substate.clone());
        }

        unreachable!()
    }

    fn find_all_direct_and_indirect_inputs(&self, module: &str) -> HashSet<&str> {
        let mut inp: HashSet<&str> = HashSet::default();
        let mut to_visit: VecDeque<&str> = VecDeque::new();

        to_visit.extend(self.input_modules.get(module).unwrap());

        while let Some(current) = to_visit.pop_front() {
            inp.insert(current);


            for direct_input in self.input_modules.get(current).unwrap_or(&vec![]) {
                if !inp.contains(direct_input) {
                    to_visit.push_back(direct_input);
                }
            }
        }

        inp
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ModuleType {
    Broadcaster,
    FlipFlop,
    Conjunction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Pulse {
    Low,
    High,
}

impl Pulse {
    fn invert(self) -> Self {
        match self {
            Pulse::Low => Pulse::High,
            Pulse::High => Pulse::Low,
        }
    }
}
