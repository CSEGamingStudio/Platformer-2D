use amethyst::{
    core::{math, SystemBundle, SystemDesc, Transform},
    derive::SystemDesc,
    ecs::prelude::*,
    Error,
};
use nphysics2d::{
    force_generator::DefaultForceGeneratorSet,
    joint::DefaultJointConstraintSet,
    object::*,
    world::{DefaultGeometricalWorld, DefaultMechanicalWorld},
};

pub use crate::components::RigidBodyComponent;

#[derive(Default)]
pub struct PhysicsBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for PhysicsBundle {
    fn build(
        self,
        _world: &mut World,
        dispatcher: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        dispatcher.add(super::player_input::PlayerInputSystem, "player_input", &[]);
        dispatcher.add(PhysicsStepSystem, "physics_step", &["player_input"]);
        dispatcher.add(CollisionSystem, "collisions", &["physics_step"]);

        Ok(())
    }
}

#[derive(Default, SystemDesc)]
struct PhysicsStepSystem;

impl<'s> System<'s> for PhysicsStepSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteExpect<'s, DefaultMechanicalWorld<f32>>,
        WriteExpect<'s, DefaultGeometricalWorld<f32>>,
        WriteExpect<'s, DefaultBodySet<f32>>,
        WriteExpect<'s, DefaultColliderSet<f32>>,
        WriteExpect<'s, DefaultJointConstraintSet<f32>>,
        WriteExpect<'s, DefaultForceGeneratorSet<f32>>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, RigidBodyComponent>,
    );

    fn run(
        &mut self,
        (
            mut mechanical_world,
            mut geometrical_world,
            mut body_set,
            mut collider_set,
            mut joint_constraint_set,
            mut force_generator_set,
            mut transforms,
            rigid_body_components,
        ): Self::SystemData,
    ) {
        for (transform, rigid_body_component) in (&mut transforms, &rigid_body_components).join() {
            let old_isometry = transform.isometry();
            let mut new_isometry = na::Isometry2::identity();
            new_isometry.append_translation_mut(&na::Translation::from(na::Vector2::new(
                old_isometry.translation.x,
                old_isometry.translation.y,
            )));
            new_isometry.append_rotation_mut(&na::UnitComplex::new_normalize(na::Complex::new(
                old_isometry.rotation.w,
                old_isometry.rotation.i,
            )));

            let rigid_body = body_set
                .rigid_body_mut(rigid_body_component.handle)
                .unwrap();
            rigid_body.set_position(new_isometry);
        }
        mechanical_world.step(
            &mut *geometrical_world,
            &mut *body_set,
            &mut *collider_set,
            &mut *joint_constraint_set,
            &mut *force_generator_set,
        );
        for (transform, rigid_body_component) in (&mut transforms, &rigid_body_components).join() {
            let rigid_body = body_set.rigid_body(rigid_body_component.handle).unwrap();
            let old_isometry = rigid_body.position();
            let mut new_isometry = math::Isometry3::identity();
            new_isometry.append_translation_mut(&math::Translation::from(math::Vector3::new(
                old_isometry.translation.x,
                old_isometry.translation.y,
                0.0,
            )));
            new_isometry.append_rotation_mut(&math::UnitQuaternion::from_quaternion(
                math::Quaternion::new(
                    old_isometry.rotation.complex().re,
                    old_isometry.rotation.complex().im,
                    0.0,
                    0.0,
                ),
            ));
            transform.set_isometry(new_isometry);
        }
    }
}

#[derive(Default, SystemDesc)]
struct CollisionSystem;

impl<'s> System<'s> for CollisionSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteExpect<'s, DefaultGeometricalWorld<f32>>,
        WriteExpect<'s, DefaultColliderSet<f32>>,
    );

    fn run(&mut self, (world, collider_set): Self::SystemData) {
        for collision in world.contact_pairs(&*collider_set, true) {
            let (_coll_handle_1, _collider_1, _coll_handle_2, _collider_2, _detector, _proximity) =
                collision;
            // println!("{:#?} {:#?}", coll_handle_1, coll_handle_2);
        }
    }
}
