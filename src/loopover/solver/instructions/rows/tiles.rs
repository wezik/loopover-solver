use crate::loopover::solver::movement;
use crate::loopover::solver::movement::{move_col, move_row};
use crate::loopover::structs::Board;

pub fn solve_bottom_tile(
    board: &mut Board,
    mut x: usize,
    mut y: usize,
    dest_x: usize,
    dest_y: usize,
) {
    let in_line = y == dest_y;
    let in_col = x == dest_x;

    if !in_col && in_line {
        (x, y) = movement::move_col(board, x, y, -1);
    }

    let offset = calculate_offset(x, dest_x);
    (x, y) = movement::move_row(board, x, y, offset);

    let offset = calculate_offset(y, dest_y);
    movement::move_col(board, x, y, offset);
}

pub fn solve_normal_tile_first(
    board: &mut Board,
    mut x: usize,
    mut y: usize,
    dest_y: usize,
    max_x: usize,
) {
    let in_line = y == dest_y;

    if in_line {
        let offset = calculate_offset(x, max_x - 1);
        movement::move_row(board, x, y, offset);
    } else {
        let offset = calculate_offset(x, max_x);
        (x, y) = movement::move_row(board, x, y, offset);

        let offset = calculate_offset(y, dest_y);
        (x, y) = movement::move_col(board, x, y, offset);

        movement::move_row(board, x, y, -1);

        movement::move_col(board, max_x, 0, -offset);
    }
}

pub fn solve_normal_tile_last(
    board: &mut Board,
    mut x: usize,
    mut y: usize,
    dest_y: usize,
    max_x: usize,
) {
    let in_line = y == dest_y;
    let in_col = x == max_x;

    if !in_line {
        if in_col {
            (x, y) = movement::move_row(board, x, y, -1);
        }

        let edge_offset = calculate_offset(dest_y, y);
        movement::move_col(board, max_x, 0, edge_offset);

        let offset = calculate_offset(x, max_x);
        movement::move_row(board, x, y, offset);

        movement::move_col(board, max_x, 0, -edge_offset);
    }
}

pub fn solve_normal_tile(
    board: &mut Board,
    mut x: usize,
    mut y: usize,
    dest_y: usize,
    max_x: usize,
) {
    let in_line = { y == dest_y };
    let in_last_col = x == max_x;

    if in_line {
        if in_last_col {
            movement::move_row(board, x, y, -1);
            return;
        }

        (x, y) = movement::move_col(board, x, y, 1);

        let helper_col_x = x;

        let offset = calculate_offset(x, max_x);
        (x, y) = movement::move_row(board, x, y, offset);

        movement::move_col(board, helper_col_x, 0, -1);
    } else {
        let offset = calculate_offset(x, max_x);
        (x, y) = movement::move_row(board, x, y, offset);
    }

    let edge_offset = calculate_offset(y, dest_y);
    (x, y) = movement::move_col(board, x, y, edge_offset);

    movement::move_row(board, x, y, -1);

    movement::move_col(board, max_x, 0, -edge_offset);
}

pub fn solve_second_from_last_tile_first(
    board: &mut Board,
    x: usize,
    y: usize,
    max_x: usize,
    max_y: usize,
) {
    let in_corner = y == max_y;
    if in_corner {
        second_from_last_corner_switch(board, x, y);
    } else {
        let offset = calculate_offset(x, max_x - 1);
        movement::move_row(board, x, y, offset);
    }
}

pub fn solve_second_from_last_tile(
    board: &mut Board,
    x: usize,
    y: usize,
    max_x: usize,
    max_y: usize,
) {
    let in_corner = y == max_y;
    if in_corner {
        second_from_last_corner_switch(board, x, y);
    } else {
        let offset = calculate_offset(x, max_x);
        let (new_x, new_y) = movement::move_row(board, x, y, offset);
        if x != new_x {
            move_col(board, new_x, new_y, -1);
            move_row(board, new_x, new_y, calculate_offset(new_x, x));
            move_col(board, new_x, new_y, 1);
        }
        move_row(board, x, y, -1);
    }
}

fn second_from_last_corner_switch(board: &mut Board, x: usize, y: usize) {
    move_col(board, x, y, -1);
    move_row(board, x, y - 1, -1);
    move_col(board, x, y, 1);
}

fn calculate_offset(current: usize, desired: usize) -> isize {
    desired as isize - current as isize
}
