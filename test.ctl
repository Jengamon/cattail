3 + 2 - 1

point = new object {
  x: (3).sp,
  y: 4
}

_3dpoint = new point {
  z: 5
}

matrix = [3, 4, 5; 6, 7, 8; 9, 10, 11]
array_value = matrix(1)(2)

interface Point {
  x, y # Types are optional, and if not specied mean "object"
}

obj_proto = new object {
  v: function(x: number, y) {# Can return *anything*
    x * y
  }
}

obj_proto.another = function() {
  return 4
}

obj = new obj_proto {
  instance_var: 5
}

#{
Reserved words:
new
null -> Null
new
and
or
true
false
function
function@
#}
#{
Primitive types
Object (*)
Number
String
Matrix
Function
Null (*)
#}
#{
in prelude:
  object __internal_create_object()
  null = __internal_create(Null, new object {})
  Null is the only interface that doesn't natually match anything
#}

# "Type-checking" is done at runtime, so there is no preanalysis.

function dot(a: Point, b: Point) -> number {
  a.x * b.x + a.y * b.y
}

# Cannot just override, must append @, so function -> function@ to override
# Maybe introduce generics later, when we have a stronger typesystem
# All types inheirit from "object" and this are handled by reference, so to make a new item, clone.
method apply(a: matrix, b: lambda(number) -> number) -> matrix {
  m = a.clone()
  len = len(m)
  for(i = 0; i < len(0)*len(1); i += 1) {
    m(i) = b(m(i)) # Does do a bit of runtime typechecking
  }
}
