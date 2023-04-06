dilemmas

# where to put size information?
in the struct
- morphism method has less inputs
- enables an accumulation mode where input size is combined with initial object sizes

in the enum
- Object struct is not needed
- less type constraints on Morphism
- no meaningless values in the actual structs

in a higher level wrapper (compared to enum):
pros:
- no meaningless values even in the enum
- explicitness about the extra value means it is only exists when meaningful/needed
cons:
- cumbersome to explicitly carry around a floating value everywhere



# require Clone vs Rc<Morphism> vs Rc<MorphismMeta>?
require Clone:
- 
Rc<Morphism>
- simple user interface
Rc<MorphismMeta>
- 


# accept HasId instead of Id?
pros:
- clients don't need to have their own map and can reuse the output
cons:
- more complicated generic types and conversions within already complicated crate
- what do we do with the heavy item? Rc? keep internal map and use Id otherwise?



# cost types

you can define two cost types: Score and Cost

Score can be negative
Cost cannot be negative


you can optimize for Cost using dijkstra