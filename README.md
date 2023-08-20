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

Depending on the optimizer you select, there is also support for additional features.
- `Accumulating`: Each morphism transforms the input size of its source object into an output size that acts as the size of the next object in that path.
- `Negatable` and `NegatableInfallible`: Morphism costs can be negative, allowing you to use it as a more abstract "score" that can go up and down. be careful to avoid negative cycles, which make it impossible to find a path.

The optimizer return value specifies three general ideas:
1. path selection: the sequence of morphisms and objects that constitute the optimized path
2. size: the input and output sizes for each step
3. cost: the overall cost of the returned path

This library guarantees perfect optimization with *either* accumulation *or* negative costs. No matter which optimizer you select, points 2 and 3 will accurately depict the effects of accumulation as applied to the path described in point 1. However, the path selection process only considers accumulation if you use the `Accumulating` optimizer, which is not compatible with negative costs.

You always specify some "input size" to the path optimizer. With the `Negatable` optimizer, it calculates the cost of each morphism by using that same input size for every morphism, then it finds the optimal path assuming those are the costs. After the path is found, it reapplies the morphisms within that path to properly account for any accumulation you may have specified in the cost function, and then it adjusts the size and cost as reported in the return value appropriately.

With the `Accumulating` optimizer, the actual optimization algorithm selects the optimal path by comparing the costs of their accumulated sizes, with output-to-input accumulation already applied uniquely to each individual path. The downside is that negative costs are not supported by this optimizer.


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
