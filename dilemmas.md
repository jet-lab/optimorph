difficult design decisions

## require Clone vs Rc<Morphism> vs Rc<MorphismMeta>?
require Clone:
- definer of the type knows the most about the type and whether it needs to be reference counted
Rc<Morphism>
- simple user interface
- prevents user from accidentally encouraging the cloning huge data structures
Rc<MorphismMeta>
- simple user interface if there is a constructor
- prevents user from accidentally encouraging the cloning huge data structures


# Decided
These dilemmas are considered settled for now, but may be revisited. The content here may be out of date since a superior approach was found to any of these options.

## terminology for the top layer of abstraction - graphs vs categories
graph pros:
- this is the language people are probably already thinking about when considering shortest-path optimizations, because those algorithms are typically conceptualized in the context of graph theory, not category theory.

category pros:
- the low level data structure is inevitably a graph. but another layer with different semantics is constructed out of that layer. should we also call that a graph? an edge in the top layer cannot be an edge in the bottom layer, it must actually be a vertex. it is confusing to use the same language to refer to opposite things. using the language from category theory for the top layer of abstraction prevents any words from having ambiguous meaning.
- the words "object" and "morphism" are less geometric, and more focused on the relationships between entities and actions. these concepts correspond well to the intended purpose of the library: you have some entity and you want to morph it into another entity using a carefully selected sequence of actions.
- in category theory it's typical to assume there are multiple morphisms pointing the same way between two objects. graph theory is compatible with this concept, but not typically used like this.
- this library is supposed to produce a sequence of actions. this is a path through a graph that is defined in terms of a list of edges. this is unusual for graph theory, which . but in category theory, this is one of the most fundamental and well-defined concepts. it's called morphism composition.

multigraph - MultiGraph, MultiEdge, MultiVertex - best of both worlds?
pros:
- uses familiar graph language
- distinguishable from lower layers of abstraction
cons:
- weird naming where everything needs "Multi" in the name even though that particular data type may not actually represent anything that is internally "multiple"

## where to put size information?
in the struct
- morphism method has less inputs
- enables an accumulation mode where input size is combined with initial object sizes

in the enum
- Object struct is not needed
- less type constraints on Morphism
- no meaningless values in the actual structs
- accumulation mode 

in a higher level wrapper (compared to enum):
pros:
- no meaningless values even in the enum
- explicitness about the extra value means it is only exists when meaningful/needed
cons:
- cumbersome to explicitly carry around a floating value everywhere

## accept HasId instead of Id?
pros:
- clients don't need to have their own map and can reuse the output
- ultimate flexibility
cons:
- more complicated to allow a category to be defined with only morphisms. need a constraint on that method that Id: HasId
- more complicated generic types and conversions within already complicated crate
- what do we do with the heavy item? Rc? keep internal map and use Id otherwise?
- if clients want to pass in just an Id then they have to somehow specify that it is its own Id. i think we need specialization to be stabilized to avoid this


## cost types brainstorming
conclusion: just went with different ApplyMorphism implementations, allow specialization on NON_NEGATIVE const generic.
----
you can define two cost types: Score and Cost

Score can be negative
Cost cannot be negative

you can optimize for Cost using dijkstra
