use std::{collections::HashMap, mem};

pub trait Migrate<const SIZE: usize>
where
    Self: Sized,
{
    fn migrate(migration_planner: &MigrationPlanner<SIZE>, from: &Self, to: &Self) -> Vec<Self>;
    fn migration_planner() -> MigrationPlanner<SIZE>;
}

pub trait ToPacketBody {
    fn bytes(&self) -> Vec<u8>;
}

pub struct MigrationPlanner<const SIZE: usize> {
    // invariant: parents are ordered before children
    field_transitive_requirements: [Vec<Requirement>; SIZE],
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct Requirement {
    pub index: usize,
    pub value: u8,
}

#[derive(Debug, Clone)]
struct MigrationNode {
    index: usize,
    value: u8,
    children: Vec<MigrationNode>,
}

impl<const SIZE: usize> MigrationPlanner<SIZE> {
    pub fn new(requirements: [Option<Requirement>; SIZE]) -> Self {
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

        let field_transitive_requirements = requirements.map(|maybe_requirement| {
            maybe_requirement
                .map(|requirement| {
                    let mut transitive_requirements = Vec::new();
                    Self::collect_requirements(
                        &mut transitive_requirements,
                        &requirements,
                        &requirement,
                    );
                    transitive_requirements
                })
                .unwrap_or_default()
        });

        Self {
            field_transitive_requirements,
        }
    }

    fn collect_requirements(
        collection: &mut Vec<Requirement>,
        requirements: &[Option<Requirement>; SIZE],
        requirement: &Requirement,
    ) {
        if let Some(requirement) = &requirements[requirement.index] {
            Self::collect_requirements(collection, requirements, requirement);
        }
        collection.push(*requirement);
    }

    pub fn migrate(&self, from: [u8; SIZE], to: &[u8; SIZE]) -> Vec<[u8; SIZE]> {
        let mut current = from;
        if current == *to {
            return Vec::new();
        }
        // Start by building a dependency tree
        let mut tree = Vec::new();
        for (index, transitive_requirements) in
            self.field_transitive_requirements.iter().enumerate().rev()
        {
            let node = MigrationNode {
                index,
                value: to[index],
                children: Vec::new(),
            };
            if let Some(first_requirement) = transitive_requirements.first() {
                tree.push(MigrationNode {
                    index: first_requirement.index,
                    value: first_requirement.value,
                    children: Vec::new(),
                });
                let mut current = tree.last_mut().unwrap();
                for requirement in transitive_requirements.iter().skip(1) {
                    current.children.push(MigrationNode {
                        index: requirement.index,
                        value: requirement.value,
                        children: Vec::new(),
                    });
                    current = current.children.first_mut().unwrap();
                }
                current.children.push(node);
            } else {
                tree.push(node);
            }
        }

        Self::squish_tree(&mut tree, &from, to);
        Self::reorder_tree(&mut tree, &from, to);

        // Convert tree to list of states
        let mut path = Vec::new();
        Self::serialize_tree(&tree, &mut current, &mut path);

        path
    }

    /// squish identical earlier nodes into the last identical node
    fn squish_tree(tree: &mut Vec<MigrationNode>, from: &[u8; SIZE], to: &[u8; SIZE]) {
        let mut identical_node_indices = HashMap::<(usize, u8), Vec<usize>>::new();
        for (i, node) in tree.iter().enumerate() {
            if let Some(indices) = identical_node_indices.get_mut(&(node.index, node.value)) {
                indices.push(i);
            } else {
                identical_node_indices.insert((node.index, node.value), vec![i]);
            }
        }

        let mut new_tree = Vec::new();
        // We need to not hold a reference to tree inside of the loop so that we can mem::take from arbitrary indices
        for i in 0..tree.len() {
            let node = &tree[i];
            let indices = identical_node_indices
                .get(&(node.index, node.value))
                .unwrap();

            if i == *indices.last().unwrap() {
                new_tree.push(MigrationNode {
                    index: node.index,
                    value: node.value,
                    children: indices
                        .iter()
                        .flat_map(|index| mem::take(&mut tree[*index].children))
                        .collect(),
                });
            }
        }
        *tree = new_tree;

        for node in tree {
            Self::squish_tree(&mut node.children, from, to);
        }
    }

    /// prefer assigning values with their dependencies already in the desired state first
    fn reorder_tree(tree: &mut Vec<MigrationNode>, from: &[u8; SIZE], to: &[u8; SIZE]) {
        let tree_len = tree.len();
        if tree_len > 2 {
            let nodes_except_last = &mut tree[0..tree_len - 1];
            nodes_except_last.sort_by_key(|n| from[n.index] != n.value);
        }

        for node in tree {
            Self::reorder_tree(&mut node.children, from, to);
        }
    }

    fn serialize_tree(
        tree: &[MigrationNode],
        current: &mut [u8; SIZE],
        dest: &mut Vec<[u8; SIZE]>,
    ) {
        for node in tree {
            if current[node.index] != node.value {
                current[node.index] = node.value;
                dest.push(*current);
            }
            Self::serialize_tree(&node.children, current, dest);
        }
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
        #[migration(to = |mode| mode as u8, from = |b| AmbientSoundMode::from_repr(b).unwrap())]
        ambient_sound_mode: AmbientSoundMode,
        #[migration(
            to = |mode| mode as u8,
            from = |b| NoiseCancelingMode::from_repr(b).unwrap(),
            required_field = ambient_sound_mode,
            required_value = AmbientSoundMode::NoiseCanceling,
        )]
        noise_canceling_mode: NoiseCancelingMode,
        #[migration(
            to = |value: ManualNoiseCanceling| value.0,
            from = ManualNoiseCanceling,
            required_field = noise_canceling_mode,
            required_value = NoiseCancelingMode::Manual,
        )]
        manual_noise_canceling: ManualNoiseCanceling,
        #[migration(
            to = |value: AdaptiveNoiseCanceling| value.0,
            from = AdaptiveNoiseCanceling,
            required_field = noise_canceling_mode,
            required_value = NoiseCancelingMode::Adaptive,
        )]
        adaptive_noise_canceling: AdaptiveNoiseCanceling,
        #[migration(
            to = |mode| mode as u8,
            from = |b| TransparencyMode::from_repr(b).unwrap(),
            required_field = ambient_sound_mode,
            required_value = AmbientSoundMode::Transparency,
        )]
        transparency_mode: TransparencyMode,
        #[migration(
            to = |value: ManualTransparency| value.0,
            from = ManualTransparency,
            required_field = transparency_mode,
            required_value = TransparencyMode::Manual,
        )]
        manual_transparency: ManualTransparency,
        #[migration(
            to = |is_enabled| is_enabled as u8,
            from = |b| b != 0,
            required_field = ambient_sound_mode,
            required_value = AmbientSoundMode::NoiseCanceling,
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
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default, FromRepr, Arbitrary)]
    pub enum TransparencyMode {
        #[default]
        TalkMode = 0,
        Manual = 1,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
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
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default, FromRepr, Arbitrary)]
    pub enum NoiseCancelingMode {
        #[default]
        Manual = 0,
        Adaptive = 1,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub struct ManualNoiseCanceling(u8);

    impl proptest::arbitrary::Arbitrary for ManualNoiseCanceling {
        type Parameters = ();

        fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
            use proptest::prelude::Strategy;

            (1u8..=5u8).prop_map(Self)
        }

        type Strategy = proptest::strategy::Map<std::ops::RangeInclusive<u8>, fn(u8) -> Self>;
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
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

            assert_eq!(from == to, plan.is_empty());

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
        assert!(average <= 7.4, "average case: {average} steps",);
    }
}
