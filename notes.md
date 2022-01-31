## 2021-11-21 20:29:47
- Done
    - Implemented Grid and Cell modules.
- Next
    - Make iterators for each_row and each_cell
    - Implement binary tree algorithm

## 2021-12-15 04:36:31

- Done
    - Added configure_cells fn
    - Implemented each_cell iterator over grid
- Next
    - Implement binary tree algorithm

## 2021-12-16 22:18:45

- Done.
    - Binary Tree implementation
    - Implemented ASCII Renderer

- Next.
    - Refactoring
    - Sidewinder algorithm

## 2022-01-11 02:32:15

- We need to Refactor the Rendering code.
- For now we can implement sidewinder algorithm

## 2022-01-25 01:36:44

- We should implement trait for algorithms, so that we can reduce the code duplication
when we calculate the deadends

## 2022-01-28 01:49:32

- <s>create a mask module</s>
    - includes a 2d grid
    - where each element is a bool
    - setter and getter
    - random_valid_position
- Use mask struct in Grid struct.

## 2022-01-31 21:30:26

- The concept here is make a mask grid, where the grid will have boolean 
value of whether its there or not, and then use that as our seed to the 
actual grid

- Cell should have an `active` field, which determines will it eligible for take
part in rendering. And by default `true` (so that it doesnt affect usual flow). 

- Then in `prepare_grid_from_mask` of `Grid`, we make only those cells, `active`, if they are true in the grid too.

- In `configure_cells`, we would skip processing active cells and in `if` arms where a cell shares border with deactive cell.

        
