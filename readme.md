# Maze Runner

![logo](mazes/mazes.jpg?raw=true  "logo")

**Project is very much inspired from the book [Mazes for programmers](https://pragprog.com/titles/jbmaze/mazes-for-programmers/)**
## Running

```
cargo run <atomic number of maze name>

Ex: `cargo run 3` will render lithium maze

Will generate random mazes everytime :)
```

## Magnesium

> Masking mazes by images

**We are rendering a Recursive backtracker maze with the different images as masks**

![mag_2](mazes/mag_2.png?raw=true  "mag_2")

![mag_1](mazes/mag_1.png?raw=true  "mag_1")

Reference images [maze_1](mazes/maze_text.png), [maze_2](mazes/spade.png), spade image credits [@prithvihv](https://github.com/prithvihv/art/blob/master/01-spade.xcf)

## Sodium

> ASCII Masks

**We are rendering a Recursive backtracker maze with the below ascii pattern from a file**

```
X . . . . . . . . X
. . . . X X . . . .
. . . X X X X . . .
. . . . X X . . . .
X . . . . . . . . X
X . . . . . . . . X
. . . . X X . . . .
. . . X X X X . . .
. . . . X X . . . .
X . . . . . . . . X
```

![sodium](mazes/sodium.png?raw=true  "sodium")

## Neon
> Using Recursive backtracker algorithm

**Same properties as `Hunt-and-kill`, but the method to recover from deadends is different**

![neon](mazes/neon.png?raw=true  "neon")

## Fluorine
> Using Hunt and kill algorithm

**This is a biased algorithm, but its bias doesn't stand out like binary-tree or sidewinder**

![fluorine](mazes/fluorine.png?raw=true  "fluorine")

## Oxygen
> Using Wilson's algorithm for unbiased mazes

**Unlike Aldous-Broder, Wilson's search for visited cells to plot a path and also use loop-erased random walk, to reduce mindless random walk**

![oxygen](mazes/oxygen.png?raw=true  "oxygen")

## Nitrogen
> Using Aldous-Broder algorithm for unbiased mazes

**Unlike `binary-tree` or `sidewinder`, Aldous-Broder's eastern and northern borders are not completely linked**

![nitrogen](mazes/nitrogen_b.png?raw=true  "nitrogen")

## Carbon
> Coloring the mazes with help of Dijkstra

**On left `binary-tree` and right `sidewinder`. The root cell is at center of maze, farther the cell from root, more intensified its color will be.** 

**Coloring is very helpful to analyze the algorithm by which a maze is generated. `binary-tree` pattern is a zigzag everytime and `sidwinder` is more of vertically distributed**.

![carbon](mazes/carbon.png?raw=true  "Carbon")

## Boron
> Making maze harder by finding one of the largest paths

**Ironically we use Dijkstra's for finding the longest path, as in Beryllium we used it for finding shortest path. The trick is to find the farthest cell from northeast cell (which is the root) and then find the farthest point from the farthest point and plot a path**

![Boron](mazes/boron.png?raw=true  "Boron")
## Beryllium
> Maze solver using the eponymous Dijkstra's algorithm

**Below is the shortest past from southwest cell to northwest cell. The numbers indicate the relative distance of each cells from the root cell (which is northwest cell, which has 0 distance from itself)**

![Beryllium](mazes/beryllium.png?raw=true  "Beryllium")

## Lithium

> Random perfect maze with sidewinder algorithm, rendered with [Macroquad](https://macroquad.rs/) 

![lithium](mazes/lithium.png?raw=true  "lithium")

## Helium

> Random perfect maze with sidewinder algorithm. Northern open bias is visible 

```
+---+---+---+---+---+---+---+---+---+---+
|                                       |
+   +---+   +   +   +   +   +   +   +   +
|       |   |   |   |   |   |   |   |   |
+   +---+---+---+---+   +---+---+---+   +
|       |               |               |
+   +---+   +   +   +---+   +---+   +   +
|       |   |   |   |           |   |   |
+   +   +---+---+---+   +---+---+   +---+
|   |       |                   |       |
+   +---+---+   +---+   +   +---+---+   +
|           |   |       |   |           |
+   +   +---+---+   +---+   +---+   +   +
|   |           |       |   |       |   |
+   +   +   +---+   +---+   +   +---+   +
|   |   |       |   |       |   |       |
+   +---+---+   +   +   +---+---+   +   +
|           |   |   |           |   |   |
+   +---+---+   +---+---+---+   +---+   +
|   |           |               |       |
+---+---+---+---+---+---+---+---+---+---+
```


## Hydrogen

> Random perfect maze with binary tree algorithm. Northern and eastern open bias is visible 

```
+---+---+---+---+---+---+---+---+---+---+
|                                       |
+   +---+   +---+---+   +   +   +---+   +
|   |       |           |   |   |       |
+   +---+---+---+---+---+   +---+---+   +
|   |                       |           |
+---+   +---+   +   +   +   +   +---+   +
|       |       |   |   |   |   |       |
+   +   +---+---+---+---+---+---+---+   +
|   |   |                               |
+   +   +---+   +   +   +---+---+   +   +
|   |   |       |   |   |           |   |
+   +   +---+   +---+   +---+   +   +   +
|   |   |       |       |       |   |   |
+   +   +   +   +---+---+---+   +---+   +
|   |   |   |   |               |       |
+   +---+   +   +---+   +---+---+---+   +
|   |       |   |       |               |
+   +---+   +   +   +---+---+   +   +   +
|   |       |   |   |           |   |   |
+---+---+---+---+---+---+---+---+---+---+
```