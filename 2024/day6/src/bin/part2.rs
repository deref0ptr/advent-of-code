use rustc_hash::{FxBuildHasher, FxHashSet};

use day6::{Direction, Lab, Vec2};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PatrolOutcome {
    Loop,
    Left,
}

fn guard_patrol(
    lab: &Lab, mut guard_position: Vec2, mut guard_direction: Direction,
    patrolled_states: &mut FxHashSet<(Vec2, Direction)>,
) -> PatrolOutcome {
    loop {
        if !patrolled_states.insert((guard_position, guard_direction)) {
            break PatrolOutcome::Loop;
        }
        if let Some(new_pos) = guard_direction.step(guard_position)
            && lab.in_bounds(new_pos)
        {
            if lab.obstacle_positions.contains(&new_pos) {
                guard_direction.turn_right();
            } else {
                guard_position = new_pos;
            }
        } else {
            break PatrolOutcome::Left;
        }
    }
}

fn main() -> day6::Result<()> {
    let (guard_position, mut lab) = day6::parse_lab("input.txt")?;
    let guard_direction = Direction::Up;

    let mut patrolled_states =
        FxHashSet::with_capacity_and_hasher(4 * lab.dimensions.0 * lab.dimensions.1, FxBuildHasher);
    let mut looping_obstacle_positions = 0;
    for x in 0..lab.dimensions.0 {
        for y in 0..lab.dimensions.1 {
            patrolled_states.clear();
            println!("Checking x={x}, y={y}:");
            let pos = Vec2(x, y);
            if pos != guard_position
                && guard_direction.step(guard_position) != Some(pos)
                && lab.obstacle_positions.insert(pos)
            {
                if guard_patrol(&lab, guard_position, guard_direction, &mut patrolled_states)
                    == PatrolOutcome::Loop
                {
                    looping_obstacle_positions += 1;
                }
                assert!(lab.obstacle_positions.remove(&pos));
            }
        }
    }

    println!("Looping obstacle positions: {looping_obstacle_positions}");

    Ok(())
}
