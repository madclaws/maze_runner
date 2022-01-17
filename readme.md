# Maze Runner

> Mazes, Mazes everywhere

## Running

```
cargo run <atomic number of maze name>

Ex: `cargo run 3` will render lithium maze
```

## Boron
> Making maze harder by finding one of the largest paths

**Ironcally we use Dijkstra's for finding the largest path, as in Beryllium we used it for finding shortest path. The trick is to find the farthest cell from root(0) and then find the farthest point from the farthest point and plot a path**

![Boron](mazes/boron.png?raw=true  "Beryllium")
## Beryllium
> Maze solver using the eponymous Dijkstra's algorithm

**Below is the shortest past from southwest cell to northwest cell. The numbers indicate the relative distance of each cells from the root cell (which is northwest cell, which has 0 distance from itself)**

![Beryllium](mazes/beryllium.png?raw=true  "Beryllium")

## Lithium

> Random perfect maze with sidwinder algorithm, rendered with [Macroquad](https://macroquad.rs/) 

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