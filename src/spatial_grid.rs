use std::fmt::Debug;

use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy::window::{PrimaryWindow, WindowResized};

use crate::boid::Boid;
use crate::movement::Position;

pub const CELL_SIZE: f32 = 30.;

type GridId = u32;

#[derive(Debug, Clone, Copy)]
struct GridInfo {
    cell_size: f32,
    grid_dimensions: UVec2,
}

impl GridInfo {
    fn new(cell_size: f32, grid_dimensions: UVec2) -> Self {
        Self {
            cell_size,
            grid_dimensions,
        }
    }

    fn get_grid_id(&self, position: Vec2) -> GridId {
        let conversion_factor = 1. / self.cell_size;

        (position.x * conversion_factor
            + position.y * conversion_factor * self.grid_dimensions.x as f32) as u32
    }

    fn get_grid_id_y(&self, grid_id: GridId) -> u32 {
        grid_id % self.grid_dimensions.x
    }

    fn update_grid_dimensions(&mut self, new_grid_dimensions: UVec2) {
        self.grid_dimensions = new_grid_dimensions;
    }
}

// https://leetless.de/posts/spatial-hashing-vs-ecs/
trait GridQuery: Debug {
    fn first_cell(&self, grid_info: GridInfo) -> GridId;
    fn next_cell(&self, grid_id: GridId, grid_info: GridInfo) -> Option<GridId>;
    fn in_range(&self, position: Vec2) -> bool;
}

#[derive(Debug, Default)]
struct SquareQuery {
    center: Vec2,
    radius: f32,
}

impl SquareQuery {
    fn new(center: Vec2, radius: f32) -> Self {
        Self { center, radius }
    }
}

impl GridQuery for SquareQuery {
    fn first_cell(&self, grid_info: GridInfo) -> GridId {
        grid_info.get_grid_id(self.center - Vec2::splat(self.radius))
    }

    fn next_cell(&self, mut grid_id: GridId, grid_info: GridInfo) -> Option<GridId> {
        let max_grid_id = grid_info.grid_dimensions.x * grid_info.grid_dimensions.y - 1;

        if grid_id == max_grid_id - 1 {
            return None;
        }

        if grid_info.get_grid_id_y(grid_id) as f32 * grid_info.cell_size
            > self.center.x + self.radius
        {
            grid_id = grid_info.get_grid_id(Vec2::new(
                self.center.x - self.radius,
                grid_info.get_grid_id_y(grid_id) as f32 * grid_info.cell_size,
            ));
        }

        if grid_info.get_grid_id_y(grid_id) as f32 * grid_info.cell_size
            > self.center.y + self.radius
        {
            return None;
        }

        Some(grid_id)
    }
    fn in_range(&self, position: Vec2) -> bool {
        position.x >= self.center.x - self.radius
            && position.x < self.center.x + self.radius
            && position.y >= self.center.y - self.radius
            && position.y < self.center.y + self.radius
    }
}

#[derive(Debug)]
struct GridIterator<'a, Q: GridQuery> {
    query: Q,
    current_cell: GridId,
    entity_iterator: Option<bevy::utils::hashbrown::hash_map::Iter<'a, Entity, Vec2>>,
    hash_grid: &'a HashMap<GridId, HashMap<Entity, Vec2>>,
    grid_info: GridInfo,
}

impl<'a, Q> Iterator for GridIterator<'a, Q>
where
    Q: GridQuery,
{
    type Item = (Entity, Vec2);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(mut it) = self.entity_iterator.take() {
            while let Some((&entity, &position)) = it.next() {
                if self.query.in_range(position) {
                    self.entity_iterator = Some(it);
                    return Some((entity, position));
                }
            }
        }

        while let Some(next_cell) = self.query.next_cell(self.current_cell, self.grid_info) {
            self.current_cell = next_cell;
            if let Some(entities) = self.hash_grid.get(&self.current_cell) {
                self.entity_iterator = Some(entities.iter());
                return self.next();
            }
        }

        None
    }
}

#[derive(Resource, Debug)]
pub struct Grid {
    grid_info: GridInfo,
    hash_grid: HashMap<GridId, HashMap<Entity, Vec2>>,
}

impl FromWorld for Grid {
    fn from_world(world: &mut World) -> Self {
        let window = world
            .query::<(&Window, With<PrimaryWindow>)>()
            .single(world);

        Self::new(Vec2::new(window.0.width(), window.0.height()), CELL_SIZE)
    }
}

impl Grid {
    fn new(workspace_dimensions: Vec2, cell_size: f32) -> Self {
        let (grid_dimensions_x, grid_dimensions_y) = (
            (workspace_dimensions.x / cell_size).ceil() as u32,
            (workspace_dimensions.y / cell_size).ceil() as u32,
        );

        Self {
            grid_info: GridInfo::new(cell_size, UVec2::new(grid_dimensions_x, grid_dimensions_y)),
            hash_grid: HashMap::new(),
        }
    }

    pub fn insert(&mut self, entity: Entity, position: Vec2) {
        self.hash_grid
            .entry(self.grid_info.get_grid_id(position))
            .or_default()
            .insert(entity, position);
    }

    pub fn remove(&mut self, entity: Entity) {
        self.hash_grid.values_mut().find_map(|hash_map| {
            hash_map
                .contains_key(&entity)
                .then(|| hash_map.remove(&entity))
        });
    }

    pub fn update(&mut self, entity: Entity, previous_position: Vec2, new_position: Vec2) {
        let previous_position_grid_id = self.grid_info.get_grid_id(previous_position);
        let new_position_grid_id = self.grid_info.get_grid_id(new_position);

        if previous_position_grid_id != new_position_grid_id {
            self.hash_grid
                .entry(previous_position_grid_id)
                .and_modify(|hash_map| {
                    hash_map.remove(&entity);
                });
        }

        self.hash_grid
            .entry(new_position_grid_id)
            .or_default()
            .insert(entity, new_position);
    }

    pub fn clear(&mut self) {
        self.hash_grid.clear();
    }

    pub fn query<Q: GridQuery>(&self, query: Q) -> GridIterator<'_, Q> {
        let first_cell = query.first_cell(self.grid_info);

        GridIterator {
            query,
            current_cell: first_cell,
            entity_iterator: self
                .hash_grid
                .get(&first_cell)
                .map(|entity_info| entity_info.iter()),
            hash_grid: &self.hash_grid,
            grid_info: self.grid_info,
        }
    }

    fn update_grid_dimensions(&mut self, new_workspace_dimensions: Vec2) {
        let (grid_dimensions_x, grid_dimensions_y) = (
            (new_workspace_dimensions.x / self.grid_info.cell_size).ceil() as u32,
            (new_workspace_dimensions.y / self.grid_info.cell_size).ceil() as u32,
        );

        self.grid_info
            .update_grid_dimensions(UVec2::new(grid_dimensions_x, grid_dimensions_y));
    }
}

pub fn update_grid_dimensions_on_window_resize(
    mut spatial_grid: ResMut<Grid>,
    mut window_resize_events: EventReader<WindowResized>,
) {
    for e in window_resize_events.iter() {
        spatial_grid.update_grid_dimensions(Vec2::new(e.width, e.height));
    }
}
