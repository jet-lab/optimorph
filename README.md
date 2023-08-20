# Optimorph

Use graph optimization algorithms dijkstra and bellman-ford to find the lowest cost composite morphism between any two objects.

This crate builds on the graph optimization crates petgraph and pathfinding, adding several additional features to the optimization algorithms that are not supported by these crates:
1. Multigraph support with unique identifiable edges called morphisms.
2. Variable morphism input sizes with input-dependent cost functions.
3. Accumulation: return values are output from one morphism and fed into successive morphisms as their inputs.
4. Infallible optimization algorithms with robust handling of negative cost cycles.

More on point 1: petgraph and pathfinding only support single anonymous edges between any two vertices. This crate takes a different approach, elevating the concept of an "edge" to a first class citizen called a "morphism". There may be any number of morphisms between any two "objects", which are analogous to vertices in a graph. Instead of returning a list of vertices, shortest-path optimization returns a CompositeMorphism that is defined as a sequence of individual morphisms.

## Cost

This crate can handle a wide variety of approaches to determine the cost of a composite morphism. By default, it uses the basic case where every morphism has equal cost. This is an ordinary shortest-path optimization of unweighted directed multigraph.

Alternatively, different morphisms can have different costs. You can define a custom cost function that calculates the cost based on some arbitrary input "size".

You can optionally also have one (but not both) of these special behaviors:
1. each morphism transforms the input size of its source object into an output size that acts as the size of the next object in that path.
2. morphism costs can be negative, allowing you to use it as a more abstract "score" that can go up and down. be careful to avoid negative cycles, which make it impossible to find a path.

You always specify some "input size" to the path optimizer. With option 1, it acts as the size of the first object only. With option 2, each morphism reuses this same exact "input size" as if it is the input it received from its source.

## Graph vs Category?

This crate primarly uses the language of category theory instead of graph theory, even though the data structures can be described as a graph. There are three reasons for this:
- The morphisms are composable, which is a definitive feature of morphisms in category theory, but not edges in graph theory.
- The category is actually a multigraph, where it is useful to think of distinct edges between two vertices as different types of transformations between those vertices, which is more consistent with the style of thinking in category than graph theory.
- The category is actually implemented with an underlying graph data structure whose vertices are not one-to-one with the objects in the top layer. Different terminology helps distinguish the layers. See below for a more detailed explanation of the layers.

### Layers

Layer 1 is the bottom layer implementation graph, and layer 2 is the category exposed by this library:

1. The petgraph and pathfinding crates define directed graphs composed of vertices, allowing you to connect any two vertices. Edges are anonymous and not represented in the type system. They are implied by defining a connection between two vertices. An edge can be thought of as an ordered pair of vertices. There can only be two edges between any two vertices - one for each direction.

2. The "category" defined by this crate is a directed multigraph that supports composition of edges. Graph layer 1 cannot directly represent a category, since it does not support unlimited distinguishable edges between nodes, nor does it support edge composition. So the category is defined at a higher level of abstraction, layered on top of graph 1. Objects and morphisms are both represented as vertices in the underlying graph 1.

Every object->object relationship in layer 2 is represented by three vertices in layer 1: one for each object and one for the morphism. No two object vertices are ever directly connect by an edge: they can only be connected through a morphism vertex. This allows us to treat morphisms as first class citizens with uniquely identifying metadata. Two morphisms with the same start and end objects can be distinguished, which is not true of the edges in graph 1.

Let's say we use a function from the pathfinding crate to calculate the optimal path from object A to object D as (A)-f->(B)-g->(C)-h->(D). The function returns a list of vertices: [A,f,B,g,C,h,D]. We can filter out all the objects from this list, leaving a list of morphisms: [f,g,h]. This can be thought of as the single composite morphism h∘g∘f in (A)--(h∘g∘f)-->(D).

For cost-based path optimization, we can assign relevant information to each vertex. Objects have sizes. Morphisms have cost functions. A cost function accepts the size of the input object as a parameter. The cost function tells us the cost of using that morphism to transform a particular size of its input object. This information is used by the pathfinding algorithm by assigning the morphism's cost as the weight of the edge from the input object to the morphism. Edges from morphisms to objects have 0 weight.
