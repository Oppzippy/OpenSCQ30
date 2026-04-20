pub trait Migrate<const SIZE: usize>
where
    Self: Sized,
{
    type T;

    fn migrate(
        migration_planner: &MigrationPlanner<Self::T, SIZE>,
        from: &Self,
        to: &Self,
    ) -> Vec<Self>;
    fn migration_planner() -> MigrationPlanner<Self::T, SIZE>;
}

pub trait ToPacketBody {
    fn bytes(&self) -> Vec<u8>;
}

pub struct MigrationPlanner<T, const SIZE: usize> {
    tree: MigrationTree<T>,
}

#[derive(Debug, Clone)]
pub struct Requirement<T> {
    pub index: usize,
    pub values: Vec<T>,
}

#[derive(Debug, Clone)]
pub struct MigrationTree<T> {
    nodes: Vec<MigrationNode<T>>,
}

#[derive(Debug, Clone)]
struct MigrationNode<T> {
    pub index: usize,
    pub required_parent_value: Vec<T>,
    pub children_indices: Vec<usize>,
}

impl<T> MigrationTree<T>
where
    T: PartialEq + Clone,
{
    /// Returns reachable nodes that are either the target state or a dependency of another field
    fn interesting_reachable_nodes<const SIZE: usize>(
        &self,
        current: &[T; SIZE],
        target: &[T; SIZE],
    ) -> Vec<(usize, T)> {
        let mut reachable = Vec::new();
        for node in &self.nodes {
            // root nodes
            if node.required_parent_value.is_empty() {
                if current[node.index] != target[node.index] {
                    reachable.push((node.index, target[node.index].clone()));
                }
                self.interesting_reachable_nodes_inner(node, current, target, &mut reachable);
            }
        }
        reachable
    }

    fn interesting_reachable_nodes_inner<const SIZE: usize>(
        &self,
        parent: &MigrationNode<T>,
        current: &[T; SIZE],
        target: &[T; SIZE],
        reachable: &mut Vec<(usize, T)>,
    ) {
        let children = &self.nodes[parent.index];
        for child_index in &children.children_indices {
            let child = &self.nodes[*child_index];
            debug_assert!(
                !child.required_parent_value.is_empty(),
                "child node should have parent requirements"
            );

            if child.required_parent_value.contains(&current[parent.index]) {
                // requirement met to change this field's value, so add the target value plus any values that enable
                // changing another field that depends on this one
                if current[child.index] != target[child.index] {
                    reachable.push((child.index, target[child.index].clone()));
                }
                self.interesting_reachable_nodes_inner(child, current, target, reachable);
            } else {
                // The requirement isn't met, so add moving into any required state as a reachable node so that we can
                // get there in the future.
                for required_value in &child.required_parent_value {
                    // This may be a duplicate, and it's much better time complexity to check here
                    // than to explore the same path multiple times.
                    if !reachable
                        .iter()
                        .any(|(index, value)| *index == parent.index && value == required_value)
                    {
                        reachable.push((parent.index, required_value.clone()))
                    }
                }
            }
        }
    }
}

impl<T, const SIZE: usize> MigrationPlanner<T, SIZE>
where
    T: Clone + PartialEq + Eq + std::hash::Hash,
{
    pub fn new(requirements: [Option<Requirement<T>>; SIZE]) -> Self {
        for (index, maybe_requirement) in requirements.iter().enumerate() {
            if let Some(requirement) = maybe_requirement
                && requirement.index >= index
            {
                panic!(
                    "parents should be listed before children, but index {index} has parent {} which is listed after it",
                    requirement.index,
                );
            }
        }

        let mut nodes = Vec::with_capacity(SIZE);
        for (index, requirement) in requirements.into_iter().enumerate() {
            if let Some(requirement) = requirement {
                // child node
                let parent_node: &mut MigrationNode<T> = nodes
                    .get_mut(requirement.index)
                    .expect("requirements should be defined in order");
                parent_node.children_indices.push(index);
                nodes.push(MigrationNode {
                    index,
                    required_parent_value: requirement.values,
                    children_indices: Vec::new(),
                });
            } else {
                // root node
                nodes.push(MigrationNode {
                    index: index,
                    required_parent_value: Vec::new(),
                    children_indices: Vec::new(),
                });
            }
        }

        Self {
            tree: MigrationTree { nodes: nodes },
        }
    }

    pub fn migrate(&self, from: &[T; SIZE], to: &[T; SIZE]) -> Vec<[T; SIZE]> {
        if from == to {
            return Vec::new();
        }

        // needs to be its own function since bfs_bidirectional's successors_fn and predecessors_fn use the same
        // generic paramter, and different closuers would have different types. For a similar reason, this can't take
        // a reference to current.
        fn map_node_to_new_state<T, const SIZE: usize>(
            current: [T; SIZE],
        ) -> impl Fn((usize, T)) -> [T; SIZE]
        where
            T: PartialEq + Clone,
        {
            move |(index, value)| {
                // TODO use debug_assert_eq once T implements debug
                debug_assert!(
                    current[index] != value,
                    "duplicates should be filtered out before being added to the vec"
                );
                let mut new_state = current.clone();
                new_state[index] = value;
                new_state
            }
        }

        let mut path = pathfinding::directed::bfs::bfs_bidirectional(
            from,
            to,
            |current| {
                let reachable_nodes = self.tree.interesting_reachable_nodes(current, to);

                reachable_nodes
                    .into_iter()
                    .map(map_node_to_new_state(current.clone()))
            },
            |current| {
                let reachable_nodes = self.tree.interesting_reachable_nodes(current, &from);

                reachable_nodes
                    .into_iter()
                    .map(map_node_to_new_state(current.clone()))
            },
        )
        .expect("a path should always be available");

        // remove the initial state from the path
        path.remove(0);
        path
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{self, AtomicUsize};

    use openscq30_lib_macros::MigrationSteps;
    use proptest::{
        prelude::{Arbitrary, prop},
        test_runner::{Config, TestRunner},
    };
    use proptest_derive::Arbitrary;
    use strum::FromRepr;

    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Arbitrary, MigrationSteps)]
    struct SoundModes {
        ambient_sound_mode: AmbientSoundMode,
        #[migration_requirement(field = ambient_sound_mode, value = AmbientSoundMode::NoiseCanceling)]
        noise_canceling_mode: NoiseCancelingMode,
        #[migration_requirement(field = noise_canceling_mode, value = NoiseCancelingMode::Manual)]
        manual_noise_canceling: ManualNoiseCanceling,
        #[migration_requirement(field = noise_canceling_mode, value = NoiseCancelingMode::Adaptive)]
        adaptive_noise_canceling: AdaptiveNoiseCanceling,
        #[migration_requirement(field = ambient_sound_mode, value = AmbientSoundMode::Transparency)]
        transparency_mode: TransparencyMode,
        #[migration_requirement(field = transparency_mode, value = TransparencyMode::Manual)]
        manual_transparency: ManualTransparency,
        #[migration_requirement(
            field = ambient_sound_mode,
            value = AmbientSoundMode::NoiseCanceling,
            value2 = AmbientSoundMode::Transparency,
        )]
        wind_noise_reduction: bool,
    }

    #[repr(u8)]
    #[derive(FromRepr, Clone, Copy, Debug, PartialEq, Eq, Hash, Default, Arbitrary)]
    pub enum AmbientSoundMode {
        #[default]
        NoiseCanceling = 0,
        Transparency = 1,
        Normal = 2,
    }

    #[repr(u8)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default, FromRepr, Arbitrary, Hash)]
    pub enum TransparencyMode {
        #[default]
        TalkMode = 0,
        Manual = 1,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
    pub struct ManualTransparency(pub u8);

    impl proptest::arbitrary::Arbitrary for ManualTransparency {
        type Parameters = ();

        fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
            use proptest::prelude::Strategy;

            (1u8..=5u8).prop_map(Self)
        }

        type Strategy = proptest::strategy::Map<std::ops::RangeInclusive<u8>, fn(u8) -> Self>;
    }

    #[repr(u8)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash, FromRepr, Arbitrary)]
    pub enum NoiseCancelingMode {
        #[default]
        Manual = 0,
        Adaptive = 1,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
    pub struct ManualNoiseCanceling(u8);

    impl proptest::arbitrary::Arbitrary for ManualNoiseCanceling {
        type Parameters = ();

        fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
            use proptest::prelude::Strategy;

            (1u8..=5u8).prop_map(Self)
        }

        type Strategy = proptest::strategy::Map<std::ops::RangeInclusive<u8>, fn(u8) -> Self>;
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
    pub struct AdaptiveNoiseCanceling(u8);

    impl proptest::arbitrary::Arbitrary for AdaptiveNoiseCanceling {
        type Parameters = ();

        fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
            use proptest::prelude::Strategy;

            (0u8..=5u8).prop_map(Self)
        }

        type Strategy = proptest::strategy::Map<std::ops::RangeInclusive<u8>, fn(u8) -> Self>;
    }

    fn assert_valid_change(from: SoundModes, to: SoundModes, index: i32) {
        let error_message = || format!("index {index}: {from:?} -> {to:?}");
        if from.transparency_mode != to.transparency_mode {
            assert_eq!(
                from.ambient_sound_mode,
                AmbientSoundMode::Transparency,
                "{}",
                error_message(),
            );
            assert_eq!(
                from.ambient_sound_mode,
                to.ambient_sound_mode,
                "{}",
                error_message(),
            );
        }

        if from.manual_transparency != to.manual_transparency {
            assert_eq!(
                from.ambient_sound_mode,
                AmbientSoundMode::Transparency,
                "{}",
                error_message(),
            );
            assert_eq!(
                from.ambient_sound_mode,
                to.ambient_sound_mode,
                "{}",
                error_message(),
            );

            assert_eq!(
                from.transparency_mode,
                TransparencyMode::Manual,
                "{}",
                error_message(),
            );
            assert_eq!(
                from.transparency_mode,
                to.transparency_mode,
                "{}",
                error_message(),
            );
        }

        if from.noise_canceling_mode != to.noise_canceling_mode {
            assert_eq!(
                from.ambient_sound_mode,
                AmbientSoundMode::NoiseCanceling,
                "{}",
                error_message(),
            );
            assert_eq!(
                from.ambient_sound_mode,
                to.ambient_sound_mode,
                "{}",
                error_message(),
            );
        }

        if from.manual_noise_canceling != to.manual_noise_canceling {
            assert_eq!(
                from.ambient_sound_mode,
                AmbientSoundMode::NoiseCanceling,
                "{}",
                error_message(),
            );
            assert_eq!(
                from.ambient_sound_mode,
                to.ambient_sound_mode,
                "{}",
                error_message(),
            );

            assert_eq!(
                from.noise_canceling_mode,
                NoiseCancelingMode::Manual,
                "{}",
                error_message(),
            );
            assert_eq!(
                from.noise_canceling_mode,
                to.noise_canceling_mode,
                "{}",
                error_message(),
            );
        }

        if from.wind_noise_reduction != to.wind_noise_reduction {
            assert_ne!(
                from.ambient_sound_mode,
                AmbientSoundMode::Normal,
                "{}",
                error_message(),
            );
            assert_eq!(
                from.ambient_sound_mode,
                to.ambient_sound_mode,
                "{}",
                error_message(),
            );
        }
    }

    proptest::proptest! {
        #[test]
        fn test_valid_state_transitions(
            from: SoundModes,
            to: SoundModes,
        ) {
            let migration_planner = SoundModes::migration_planner();
            let plan = SoundModes::migrate(&migration_planner, &from, &to);

            assert_eq!(from == to, plan.is_empty(), "the plan should be empty if and only if the start and end are the same");

            if !plan.is_empty() {
                // the initial state is not part of the plan, so check that separately
                assert_valid_change(from, *plan.first().unwrap(), -1);
                // the final state is a part of the plan, though
                assert_eq!(*plan.last().unwrap(), to, "last plan step should equal target");

                plan.windows(2)
                    .enumerate()
                    .for_each(|(i, change)| assert_valid_change(change[0], change[1], i as i32))
            }
        }

        #[test]
        fn test_worst_case(
            from: SoundModes,
            to: SoundModes,
        ) {
            let migration_planner = SoundModes::migration_planner();
            let plan = SoundModes::migrate(&migration_planner, &from, &to);

            assert!(plan.len() <= 11, "{} steps:\n{plan:#?}", plan.len());
        }
    }

    #[test]
    fn average_case() {
        let mut runner = TestRunner::new(Config {
            cases: 10000,
            rng_algorithm: prop::test_runner::RngAlgorithm::ChaCha,
            rng_seed: prop::test_runner::RngSeed::Fixed(0),
            ..Config::default()
        });
        let migration_planner = SoundModes::migration_planner();
        let total = AtomicUsize::new(0);
        runner
            .run(
                &(SoundModes::arbitrary(), SoundModes::arbitrary()),
                |(from, to)| {
                    let plan = SoundModes::migrate(&migration_planner, &from, &to);
                    total.fetch_add(plan.len(), atomic::Ordering::Relaxed);
                    Ok(())
                },
            )
            .unwrap();
        let total = total.load(atomic::Ordering::Relaxed);
        let average = total as f64 / runner.config().cases as f64;

        // round up to nearest 10th for leeway
        assert!(average <= 7.0, "average case: {average} steps",);
    }

    #[test]
    fn test_change_ambient_sound_mode() {
        let migration_planner = SoundModes::migration_planner();
        let from = SoundModes {
            ambient_sound_mode: AmbientSoundMode::Transparency,
            ..Default::default()
        };
        let to = SoundModes {
            ambient_sound_mode: AmbientSoundMode::Normal,
            ..from
        };
        let path = SoundModes::migrate(&migration_planner, &from, &to);
        assert_eq!(path, vec![to]);
    }

    #[test]
    fn test_change_wind_noise_suppression_from_noise_canceling() {
        let migration_planner = SoundModes::migration_planner();
        let from = SoundModes {
            ambient_sound_mode: AmbientSoundMode::NoiseCanceling,
            wind_noise_reduction: false,
            ..Default::default()
        };
        let to = SoundModes {
            ambient_sound_mode: AmbientSoundMode::NoiseCanceling,
            wind_noise_reduction: true,
            ..from
        };
        let path = SoundModes::migrate(&migration_planner, &from, &to);
        assert_eq!(path, vec![to]);
    }

    #[test]
    fn test_change_wind_noise_suppression_from_transparency() {
        let migration_planner = SoundModes::migration_planner();
        let from = SoundModes {
            ambient_sound_mode: AmbientSoundMode::Transparency,
            wind_noise_reduction: false,
            ..Default::default()
        };
        let to = SoundModes {
            ambient_sound_mode: AmbientSoundMode::Transparency,
            wind_noise_reduction: true,
            ..from
        };
        let path = SoundModes::migrate(&migration_planner, &from, &to);
        assert_eq!(path, vec![to]);
    }
}
