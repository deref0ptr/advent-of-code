use std::collections::HashSet;

use day6::Direction;

fn main() -> day6::Result<()> {
    let (mut guard_position, lab) = day6::parse_lab("input.txt")?;
    let mut guard_direction = Direction::Up;

    let mut patrolled_positions = HashSet::with_capacity(lab.dimensions.0 * lab.dimensions.1);

    while let Some(new_pos) = guard_direction.step(guard_position)
        && lab.in_bounds(new_pos)
    {
        patrolled_positions.insert(guard_position);
        if lab.obstacle_positions.contains(&new_pos) {
            guard_direction.turn_right();
        } else {
            guard_position = new_pos;
        }
    }

    println!("Patrolled positions: {}", patrolled_positions.len());

    Ok(())
}
